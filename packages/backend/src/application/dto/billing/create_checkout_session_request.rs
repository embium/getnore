use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateCheckoutSessionRequest {
    pub price_id: String,
}
