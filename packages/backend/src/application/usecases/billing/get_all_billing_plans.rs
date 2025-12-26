use std::sync::Arc;

use crate::{
    domain::{entities::billing::Billing, repositories::billing_repo::BillingRepository},
    infra::errors::app_error::AppError,
};

#[derive(Clone)]
pub struct GetAllBillingPlans<R> {
    billing_repo: Arc<R>,
}

impl<R> GetAllBillingPlans<R>
where
    R: BillingRepository,
{
    pub fn new(billing_repo: Arc<R>) -> Self {
        Self { billing_repo }
    }

    pub async fn execute(&self) -> Result<Vec<Billing>, AppError> {
        let plan = self.billing_repo.find_all().await?;

        Ok(plan)
    }
}
