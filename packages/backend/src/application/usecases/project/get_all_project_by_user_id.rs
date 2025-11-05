use std::sync::Arc;

use crate::{
    domain::{entities::project::Project, repositories::project_repo::ProjectRepository},
    infra::errors::app_error::AppError,
};

#[derive(Clone)]
pub struct GetAllProjectByUserId<R> {
    project_repo: Arc<R>,
}

impl<R> GetAllProjectByUserId<R>
where
    R: ProjectRepository,
{
    pub fn new(project_repo: Arc<R>) -> Self {
        Self { project_repo }
    }

    pub async fn execute(&self, user_id: &str) -> Result<Vec<Project>, AppError> {
        let projects = self.project_repo.find_all_by_user_id(user_id).await?;

        Ok(projects)
    }
}
