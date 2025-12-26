use std::sync::Arc;
use axum::{
    Error, Router,
    body::Body,
    extract::{FromRequest, State},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
};
use stripe_webhook::{Event, EventObject, Webhook};

use crate::application::state::AppState;

pub type Result<T, E = Error> = std::result::Result<T, E>;struct StripeEvent(Event);

impl<S> FromRequest<S> for StripeEvent
where
    String: FromRequest<S>,
    S: Send + Sync + AsRef<AppState>,
{
    type Rejection = Response;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let signature = if let Some(sig) = req.headers().get("stripe-signature") {
            sig.to_owned()
        } else {
            return Err(StatusCode::BAD_REQUEST.into_response());
        };

        let payload =
            String::from_request(req, state).await.map_err(IntoResponse::into_response)?;

        let app_state = state.as_ref();

        let stripe_webhook_secret = app_state.cfg.stripe_webhook_secret.clone();

        Ok(Self(
            Webhook::construct_event(&payload, signature.to_str().unwrap(), &stripe_webhook_secret.clone())
                .map_err(|_| StatusCode::BAD_REQUEST.into_response())?,
        ))
    }
}


pub fn setup_stripe_routes() -> Router<Arc<AppState>> {
    Router::new()
      .route("/webhook", post(handle_webhook))
}

#[axum::debug_handler]
async fn handle_webhook(
  State(state): State<Arc<AppState>>,
  StripeEvent(event): StripeEvent
) {
    match event.data.object {
        EventObject::CustomerSubscriptionCreated(subscription) => {
            println!("Received customer subscription created webhook with id: {:?}", subscription.id);
        }
        EventObject::CustomerSubscriptionUpdated(subscription) => {
            println!("Received customer subscription updated webhook with id: {:?}", subscription.id);
        }
        EventObject::CustomerSubscriptionDeleted(subscription) => {
            println!("Received customer subscription deleted webhook with id: {:?}", subscription.id);
        }
        EventObject::AccountUpdated(account) => {
            println!("Received account updated webhook for account: {:?}", account.id);
        }
        _ => println!("Unknown event encountered in webhook: {:?}", event.type_),
    }
}
