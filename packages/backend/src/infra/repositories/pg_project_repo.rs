use crate::{
    domain::{ entities::project::Project, repositories::project_repo::ProjectRepository },
    infra::errors::app_error::AppError,
};

#[derive(Clone, Debug)]
pub struct PgProjectRepository {
    pool: sqlx::PgPool,
}

impl PgProjectRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ProjectRepository for PgProjectRepository {
    async fn find_all_by_user_id(&self, user_id: &str) -> Result<Vec<Project>, AppError> {
        let projects = sqlx::query_as!(Project, "SELECT * FROM projects WHERE user_id = $1 AND deleted_at IS NULL", user_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(projects)
    }

    async fn find_by_id_and_user_id(&self, id: &str, user_id: &str) -> Result<Project, AppError> {
      let project = sqlx::query_as!(Project, "SELECT * FROM projects WHERE id = $1 AND user_id = $2", id, user_id)
          .fetch_one(&self.pool)
          .await?;

      Ok(project)
    }
    async fn create(&self, entity: Project) -> Result<Project, AppError> {
        let project = sqlx::query_as!(
            Project,
            "INSERT INTO projects (id, user_id, name, description, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
            entity.id,
            entity.user_id,
            entity.name,
            entity.description,
            entity.created_at,
            entity.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(project)
    }

    async fn update(&self, id: &str, user_id: &str, entity: Project) -> Result<Project, AppError> {
        let project = sqlx::query_as!(
            Project,
            "UPDATE projects SET name = $1, description = $2, updated_at = NOW() WHERE id = $3 AND user_id = $4 RETURNING *",
            entity.name,
            entity.description,
            id,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(project)
    }

    async fn delete(
        &self,
        id: &str,
        user_id: &str
    ) -> Result<(), AppError> {
        sqlx
            ::query!("UPDATE projects SET deleted_at = NOW() WHERE id = $1 AND user_id = $2", id, user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
