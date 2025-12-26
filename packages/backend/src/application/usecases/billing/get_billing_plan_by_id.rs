use std::sync::Arc;

use crate::{
    domain::{entities::billing::Billing, repositories::billing_repo::BillingRepository},
    infra::errors::app_error::AppError,
};

#[derive(Clone)]
pub struct GetBillingPlanById<R> {
    billing_repo: Arc<R>,
}

impl<R> GetBillingPlanById<R>
where
    R: BillingRepository,
{
    pub fn new(billing_repo: Arc<R>) -> Self {
        Self { billing_repo }
    }

    pub async fn execute(&self, plan_id: &str) -> Result<Billing, AppError> {
        let plan = self.billing_repo.find_by_id(plan_id).await?;

        Ok(plan)
    }
}
