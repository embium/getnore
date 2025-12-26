use stripe_checkout::CheckoutSessionMode;
use stripe_checkout::checkout_session::{CreateCheckoutSession, CreateCheckoutSessionLineItems};
use stripe::Client;

use crate::{
    application::dto::billing::create_checkout_session_request::CreateCheckoutSessionRequest,
    infra::{errors::app_error::AppError},
};

#[derive(Clone)]
pub struct CreateCheckoutSessionUseCase {
    stripe_client: Client,
}

impl CreateCheckoutSessionUseCase {
    pub fn new(stripe_client: Client) -> Self {
        Self { stripe_client }
    }

    pub async fn execute(&self, user_id: &str, req: CreateCheckoutSessionRequest  ) -> Result<String, AppError> {
      // Set up the line item
      let line_items = vec![CreateCheckoutSessionLineItems {
          price: Some(req.price_id.clone()),
          quantity: Some(1),
          ..Default::default()
      }];

      let checkout_session = CreateCheckoutSession::new()
          .success_url("http://getnore.com/payment-success")
          .cancel_url("http://getnore.com/pricing")
          .client_reference_id(user_id)
          .mode(CheckoutSessionMode::Subscription)
          .line_items(line_items)
          .expand([String::from("line_items"), String::from("line_items.data.price.product")])
          .send(&self.stripe_client.clone())
          .await?;

      // Return the URL for the frontend to redirect to
      Ok(checkout_session.url.ok_or_else(|| AppError::StripeError(stripe::StripeError::ClientError("Stripe session has no URL".to_string())))?)
    }
}
