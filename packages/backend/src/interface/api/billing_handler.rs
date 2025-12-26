use std::sync::Arc;

use axum::{ Extension, Json, Router, extract::{Path, State}, middleware, routing::{get,post}};

use crate::{
    application::{dto::billing::create_checkout_session_request::CreateCheckoutSessionRequest, state::AppState, usecases::billing::get_billing_plan_by_id},
    domain::entities::{billing::Billing, billing::SessionUrl, user::UserFull},
    infra::{
        errors::app_error::AppError,
        utils::response::SuccessResponse,
    },
    interface::middleware::auth_mw::is_authorized,
};

pub fn setup_billing_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/checkout", post(create_checkout_session))
        .layer(middleware::from_fn_with_state(app_state.clone(), is_authorized))
        .route("/{id}", get(get_billing_plan_by_id))
        .route("/", get(get_all_billing_plans))
}

async fn create_checkout_session(
    Extension(current_user): Extension<UserFull>,
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateCheckoutSessionRequest>
) -> Result<SuccessResponse<SessionUrl>, AppError> {
    // Check authorization
    // state.rbac.check_access(&current_user.roles, "billing", "read").await?;

    let user_id = &current_user.user.id;

    let session_url = state.uc.billing.create_checkout_session.execute(user_id, req).await?;

    Ok(SuccessResponse::with_data(200, SessionUrl {
      session_url
    }))
}

async fn get_billing_plan_by_id(
  State(state): State<Arc<AppState>>,
  Path(plan_id): Path<String>
) -> Result<SuccessResponse<Billing>, AppError> {

  let billing_plan =  state.uc.billing.get_billing_plan_by_id.execute(&plan_id).await?;

  Ok(SuccessResponse::with_data(200, billing_plan))
}

async fn get_all_billing_plans(
  State(state): State<Arc<AppState>>,
) -> Result<SuccessResponse<Vec<Billing>>, AppError> {

  let billing_plans =  state.uc.billing.get_all_billing_plans.execute().await?;

  Ok(SuccessResponse::with_data(200, billing_plans))
}
