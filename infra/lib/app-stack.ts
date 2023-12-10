import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import { FunctionUrlAuthType } from 'aws-cdk-lib/aws-lambda';
import * as dynamodb from 'aws-cdk-lib/aws-dynamodb';
import { AttributeType, BillingMode, TableEncryption } from 'aws-cdk-lib/aws-dynamodb';

export class AppStack extends cdk.Stack {
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

    listLocationsFunction.addFunctionUrl({
      authType: FunctionUrlAuthType.NONE,
    })

    // The code that defines your stack goes here

    // example resource
    // const queue = new sqs.Queue(this, 'InfraQueue', {
    //   visibilityTimeout: cdk.Duration.seconds(300)
    // });
  }
}
