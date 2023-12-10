use crate::domain::location::Location;
use async_trait::async_trait;
use aws_sdk_dynamodb::Error;

#[async_trait]
pub trait LocationRepository {
    /// Lists all locations in the repository
    async fn list_all(&self) -> Result<Vec<Location>, Error>;
}
