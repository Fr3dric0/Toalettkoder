import * as cdk from 'aws-cdk-lib';
import {Construct} from 'constructs';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import {FunctionUrlAuthType} from 'aws-cdk-lib/aws-lambda';

export class AppStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const listLocationsFunction = new lambda.Function(this, 'list-locations-v1', {
      functionName: `${id}-list-locations-v1`,
      runtime: lambda.Runtime.PROVIDED_AL2023,
      handler: 'rust.handler',
      code: lambda.Code.fromAsset('../list-locations-v1/target/lambda/list-locations-v1/bootstrap.zip'),
      architecture: lambda.Architecture.ARM_64,
    })

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
