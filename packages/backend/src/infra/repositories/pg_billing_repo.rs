use crate::{
    domain::{ entities::billing::Billing, repositories::billing_repo::BillingRepository },
    infra::errors::app_error::AppError,
};

#[derive(Clone, Debug)]
pub struct PgBillingRepository {
    pool: sqlx::PgPool,
}

impl PgBillingRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl BillingRepository for PgBillingRepository {
    async fn find_all(&self) -> Result<Vec<Billing>, AppError> {
      let plans = sqlx::query_as!(Billing, "SELECT * FROM billing WHERE deleted_at IS NULL")
        .fetch_all(&self.pool)
        .await?;

      Ok(plans)
    }

    async fn find_by_id(&self, user_id: &str) -> Result<Billing, AppError> {
        let plan = sqlx::query_as!(Billing, "SELECT * FROM billing WHERE id = $1 AND deleted_at IS NULL", user_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(plan)
    }

    async fn create(&self, entity: Billing) -> Result<Billing, AppError> {
        let plan = sqlx::query_as!(
            Billing,
            "INSERT INTO billing (id, name, description, price_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
            entity.id,
            entity.name,
            entity.description,
            entity.price_id,
            entity.created_at,
            entity.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(plan)
    }

    async fn update(&self, id: &str, entity: Billing) -> Result<Billing, AppError> {
        let plan = sqlx::query_as!(
            Billing,
            "UPDATE billing SET name = $1, description = $2, price_id = $3, updated_at = NOW() WHERE id = $4 RETURNING *",
            entity.name,
            entity.description,
            entity.price_id,
            id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(plan)
    }

    async fn delete(
        &self,
        id: &str,
    ) -> Result<(), AppError> {
        sqlx
            ::query!("UPDATE billing SET deleted_at = NOW() WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
