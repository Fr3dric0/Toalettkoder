mod location;
mod api_utils;

use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use uuid::Uuid;
use crate::api_utils::response_ok;
use crate::location::{Coordinate, Location, Toilet, Wifi};


/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");

    let locations = [
      Location {
          id: Uuid::new_v4().to_string(),
          name: "Espresso house Torshov".to_string(),
          location: Coordinate {
              lat: 59.9345054,
              lon: 10.7639602,
          },
          toilet: Option::from(Toilet { code: 2023 }),
          wifi: Option::from(Wifi { code: None, is_open_network: true }),
      }  
    ];

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = response_ok(locations)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
