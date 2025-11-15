use std::sync::Arc;

use axum::{ Extension, Json, Router, extract::{Path, State}, middleware, routing::get};

use crate::{
    application::{dto::project::create_update_project_request::CreateOrUpdateProject, state::AppState},
    domain::entities::{project::{Project, ProjectWithOwnerEmail}, user::UserFull},
    infra::{
        errors::app_error::AppError,
        utils::response::SuccessResponse,
    },
    interface::middleware::auth_mw::is_authorized,
};

pub fn setup_project_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_projects_by_user_id).post(create_project))
        .route("/{id}", get(get_project_by_id_and_user_id).put(update_project).delete(delete_project))
        // Authentication layer (runs first)
        .layer(middleware::from_fn_with_state(app_state.clone(), is_authorized))
}

async fn get_all_projects_by_user_id(
    Extension(current_user): Extension<UserFull>,
    State(state): State<Arc<AppState>>
) -> Result<SuccessResponse<Vec<Project>>, AppError> {
    // Check authorization
    state.rbac.check_access(&current_user.roles, "projects", "read").await?;
    
    let user_id = &current_user.user.id;

    let projects = state.uc.project.get_all_by_user_id.execute(user_id).await?;

    Ok(SuccessResponse::with_data(200, projects))
}

async fn get_project_by_id_and_user_id(
    Extension(current_user): Extension<UserFull>,
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<String>
) -> Result<SuccessResponse<ProjectWithOwnerEmail>, AppError> {
    // Check authorization
    state.rbac.check_access(&current_user.roles, "projects", "read").await?;
    
    let user_id = &current_user.user.id;

    let project = state.uc.project.get_by_id_and_user_id.execute(&project_id, user_id).await?;

    if project.user_id != Some(user_id.to_string()) {
        return Ok(SuccessResponse::with_message(403, "You don't have access to this project"));
    }

    Ok(SuccessResponse::with_data(200, project))
}

async fn create_project(
    Extension(current_user): Extension<UserFull>,
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateOrUpdateProject>
) -> Result<SuccessResponse<Project>, AppError> {
    // Check authorization
    state.rbac.check_access(&current_user.roles, "projects", "write").await?;
    
    let user_id = &current_user.user.id;

    let project = state.uc.project.create_project.execute(user_id, req).await?;

    Ok(SuccessResponse::with_data(201, project))
}

async fn update_project(
    Extension(current_user): Extension<UserFull>,
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<String>,
    Json(req): Json<CreateOrUpdateProject>
) -> Result<SuccessResponse<ProjectWithOwnerEmail>, AppError> {
    // Check authorization
    state.rbac.check_access(&current_user.roles, "projects", "write").await?;
    
    let user_id = &current_user.user.id;

    let project = state.uc.project.get_by_id_and_user_id.execute(&project_id, user_id).await?;

    if project.user_id != Some(user_id.to_string()) {
        return Ok(SuccessResponse::with_message(403, "You don't have access to this project"));
    }

    let project = state.uc.project.update_project.execute(&project_id, user_id, req).await?;

    Ok(SuccessResponse::with_data(200, project))
}

async fn delete_project(
    Extension(current_user): Extension<UserFull>,
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<String>,
) -> Result<SuccessResponse<Project>, AppError> {
    // Check authorization
    state.rbac.check_access(&current_user.roles, "projects", "delete").await?;
    
    let user_id = &current_user.user.id;

    let project = state.uc.project.get_by_id_and_user_id.execute(&project_id, user_id).await?;

    if project.user_id != Some(user_id.to_string()) {
        return Ok(SuccessResponse::with_message(403, "You don't have access to this project"));
    }

    state.uc.project.delete_project.execute(&project_id, user_id).await?;

    Ok(SuccessResponse::with_code(200))
}
