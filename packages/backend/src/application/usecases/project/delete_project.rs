use std::sync::Arc;

use crate::{
    domain::repositories::project_repo::ProjectRepository,
    infra::{errors::app_error::AppError},
};

#[derive(Clone)]
pub struct DeleteProject<R> {
    project_repo: Arc<R>
}

impl<R> DeleteProject<R>
where
    R: ProjectRepository,
{
    pub fn new(project_repo: Arc<R>) -> Self {
        Self { project_repo }
    }

    pub async fn execute(&self, id: &str, user_id: &str) -> Result<(), AppError> {

        self.project_repo.delete(id, user_id).await?;

        Ok(())
    }
}
