use crate::{
    domain::entities::billing::Billing, infra::errors::app_error::AppError,
};

#[async_trait::async_trait]
pub trait BillingRepository {
    async fn find_all(&self) -> Result<Vec<Billing>, AppError>;
    async fn find_by_id(&self, id: &str) -> Result<Billing, AppError>;
    async fn create(&self, billing: Billing) -> Result<Billing, AppError>;
    async fn update(&self, id: &str, billing: Billing) -> Result<Billing, AppError>;
    async fn delete(&self, id: &str) -> Result<(), AppError>;
}
