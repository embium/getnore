use std::sync::Arc;
use stripe::Client;

use crate::infra::repositories::pg_billing_repo::PgBillingRepository;

use super::{
    get_billing_plan_by_id::GetBillingPlanById,
    get_all_billing_plans::GetAllBillingPlans,
    create_checkout_session::CreateCheckoutSessionUseCase,
};

#[derive(Clone)]
pub struct BillingUsecase {
    pub get_billing_plan_by_id: Arc<GetBillingPlanById<PgBillingRepository>>,
    pub get_all_billing_plans: Arc<GetAllBillingPlans<PgBillingRepository>>,
    pub create_checkout_session: Arc<CreateCheckoutSessionUseCase>,
}

impl BillingUsecase {
    pub fn new(billing_repo: Arc<PgBillingRepository>, stripe_client: Client) -> Self {
        let get_billing_plan_by_id = Arc::new(GetBillingPlanById::new(billing_repo.clone()));
        let get_all_billing_plans = Arc::new(GetAllBillingPlans::new(billing_repo.clone()));
        let create_checkout_session = Arc::new(CreateCheckoutSessionUseCase::new(stripe_client));

        Self {
            get_billing_plan_by_id,
            get_all_billing_plans,
            create_checkout_session
        }
    }
}
