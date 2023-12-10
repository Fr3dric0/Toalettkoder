use crate::domain::location::Location;
use crate::domain::location_repository::LocationRepository;
use async_trait::async_trait;
use aws_sdk_dynamodb::Error;

pub struct DynamoDbLocationRepository {
    pub dynamodb_client: aws_sdk_dynamodb::Client,
    pub table_name: String,
}

#[async_trait]
impl LocationRepository for DynamoDbLocationRepository {
    async fn list_all(&self) -> Result<Vec<Location>, Error> {
        let table_name = self.table_name.to_string();
        tracing::info!({ %table_name }, "Listing all locations from table");

        let results = self
            .dynamodb_client
            .scan()
            .table_name(table_name)
            .send()
            .await?;

        if let Some(items) = results.items {
            let locations: Vec<Location> = items.iter().map(|item| item.into()).collect();
            return Ok(locations);
        } else {
            return Ok(Vec::new());
        }
    }
}
