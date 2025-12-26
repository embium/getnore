use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SessionUrl {
  pub session_url: String
}

#[derive(Debug, Clone, Serialize)]
pub struct Billing {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub price_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Billing {
    pub fn new(id: String, name: String, description: String, price_id: String) -> Self {
        Self {
            id,
            name,
            description: Some(description),
            price_id,
            created_at: chrono::Utc::now(),
            updated_at: Some(chrono::Utc::now()),
            deleted_at: None,
        }
    }
}
