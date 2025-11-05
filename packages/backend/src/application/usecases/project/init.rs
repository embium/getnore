use std::sync::Arc;

use crate::infra::repositories::pg_project_repo::PgProjectRepository;

use super::{
    get_all_project_by_user_id::GetAllProjectByUserId,
    create_project::CreateProject,
    get_project_by_id_and_user_id::GetProjectByIdAndUserId,
    update_project::UpdateProject,
    delete_project::DeleteProject,
};

#[derive(Clone)]
pub struct ProjectUsecase {
    pub get_all_by_user_id: Arc<GetAllProjectByUserId<PgProjectRepository>>,
    pub create_project: Arc<CreateProject<PgProjectRepository>>,
    pub get_by_id_and_user_id: Arc<GetProjectByIdAndUserId<PgProjectRepository>>,
    pub update_project: Arc<UpdateProject<PgProjectRepository>>,
    pub delete_project: Arc<DeleteProject<PgProjectRepository>>,
}

impl ProjectUsecase {
    pub fn new(project_repo: Arc<PgProjectRepository>) -> Self {
        let get_all_by_user_id = Arc::new(GetAllProjectByUserId::new(project_repo.clone()));
        let create_project = Arc::new(CreateProject::new(project_repo.clone()));
        let get_by_id_and_user_id = Arc::new(GetProjectByIdAndUserId::new(project_repo.clone()));
        let update_project = Arc::new(UpdateProject::new(project_repo.clone()));
        let delete_project = Arc::new(DeleteProject::new(project_repo.clone()));

        Self {
            get_all_by_user_id,
            create_project,
            get_by_id_and_user_id,
            update_project,
            delete_project,
        }
    }
}
