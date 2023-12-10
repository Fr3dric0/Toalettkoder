mod api_utils;
mod domain;
mod utils;

use crate::api_utils::response_ok;
use crate::domain::dynamodb_location_repository::DynamoDbLocationRepository;
use crate::domain::location_repository::LocationRepository;
use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use std::env;

async fn init_dynamodb() -> aws_sdk_dynamodb::Client {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;
    return aws_sdk_dynamodb::Client::new(&config);
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let dynamodb_client = init_dynamodb().await;
    let location_repository = DynamoDbLocationRepository {
        dynamodb_client,
        table_name: env::var("LOCATIONS_TABLE_NAME").unwrap().to_string(),
    };

    tracing::info!("Retrieving list of locations...");

    let locations = location_repository.list_all().await?;
    let locations_formatted = serde_json::to_string(&locations).unwrap();
    tracing::info!({ %locations_formatted }, "Locations retrieved!");

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = response_ok(locations)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::DEBUG)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
