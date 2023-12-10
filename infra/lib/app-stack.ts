import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import * as dynamodb from 'aws-cdk-lib/aws-dynamodb';
import { AttributeType, BillingMode, TableEncryption } from 'aws-cdk-lib/aws-dynamodb';
import * as apigateway from 'aws-cdk-lib/aws-apigateway';
import { IRestApi, SecurityPolicy } from 'aws-cdk-lib/aws-apigateway';
import * as acm from 'aws-cdk-lib/aws-certificatemanager';
import * as route53 from 'aws-cdk-lib/aws-route53';
import * as route53Targets from 'aws-cdk-lib/aws-route53-targets';
import { CertificateValidation } from 'aws-cdk-lib/aws-certificatemanager';
import { HostedZone, IHostedZone } from "aws-cdk-lib/aws-route53";
import { Duration } from "aws-cdk-lib";

const domainName = 'toalettkoder.prod.lokalvert.tech';

export class AppStack extends cdk.Stack {
  private readonly hostedZoneId = 'Z07564492IFZPEI4UUD2C';

  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const locationsTable = new dynamodb.Table(this, 'LocationsTable', {
      tableName: `${id}-locations`,
      billingMode: BillingMode.PAY_PER_REQUEST,
      encryption: TableEncryption.AWS_MANAGED,
      partitionKey: {
        name: 'id',
        type: AttributeType.STRING,
      },
      sortKey: {
        // "current" or "previous". We don't bother storing any longer history than that
        name: 'version',
        type: AttributeType.STRING,
      }
    });

    const hostedZone = HostedZone.fromHostedZoneAttributes(this, 'HostedZone', {
      hostedZoneId: this.hostedZoneId,
      zoneName: 'prod.lokalvert.tech',
    });

    const api = this.buildRestApi(this, 'RestApi', hostedZone, domainName);
    const v1Resource = api.root.addResource('v1');

    const listLocationsFunction = new lambda.Function(this, 'list-locations-v1', {
      functionName: `${id}-list-locations-v1`,
      runtime: lambda.Runtime.PROVIDED_AL2023,
      handler: 'rust.handler',
      code: lambda.Code.fromAsset('../list-locations-v1/target/lambda/list-locations-v1/bootstrap.zip'),
      architecture: lambda.Architecture.ARM_64,
      environment: {
          'LOCATIONS_TABLE_NAME': locationsTable.tableName,
      },
    });

    locationsTable.grantReadData(listLocationsFunction);

    // URL /v1/locations/ is where we'll place locations specific actions
    const locationsResource = v1Resource.addResource('locations');
    locationsResource.addMethod('GET', new apigateway.LambdaIntegration(listLocationsFunction));
  }

  private buildRestApi(scope: Construct, idPrefix: string, hostedZone: IHostedZone, domainName: string): IRestApi {
    const certificate = new acm.Certificate(scope, 'ApiCertificate', {
      domainName,
      validation: CertificateValidation.fromDns(hostedZone),
    });
    const api = new apigateway.RestApi(scope, 'RestApi', {
      restApiName: 'toalettkoder',
      description: 'Provide easy availability to valuable information of locations',
      endpointTypes: [apigateway.EndpointType.REGIONAL],
      domainName: {
        domainName,
        certificate: certificate,
        securityPolicy: SecurityPolicy.TLS_1_2,
      },
      deployOptions: {
        metricsEnabled: true,
        // Utilize cache to reduce load on the backend
        cachingEnabled: true,
        cacheTtl: Duration.minutes(60),
        // Heavily rate limit the API to avoid potential for abuse
        throttlingRateLimit: 1,
        throttlingBurstLimit: 2,
      },
    });
    new route53.ARecord(scope, 'ApiDnsRecord', {
      recordName: domainName,
      zone: hostedZone,
      target: route53.RecordTarget.fromAlias(new route53Targets.ApiGateway(api)),
    });

    return api;
  }
}
