use crate::{
    entity::{self, TodoPayload},
    service::{self, ServiceError},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::DbConn;

#[derive(Clone)]
pub struct AppState {
    pub db: DbConn,
}

#[utoipa::path(
    post,
    path = "/todos",
    request_body = TodoPayload,
    responses(
        (status = 201, description = "Todo created successfully", body = entity::Model),
        (status = 500, description = "Internal server error")
    )
)]
#[axum::debug_handler]
pub async fn create_todo(
    State(state): State<AppState>,
    Json(data): Json<TodoPayload>,
) -> Result<(StatusCode, Json<entity::Model>), AppError> {
    let todo = service::create_todo(&state.db, data).await?;
    Ok((StatusCode::CREATED, Json(todo)))
}

#[utoipa::path(
    get,
    path = "/todos/{id}",
    params(
        ("id" = i32, Path, description = "Todo id")
    ),
    responses(
        (status = 200, description = "Todo found", body = entity::Model),
        (status = 404, description = "Todo not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[axum::debug_handler]
pub async fn get_todo_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<entity::Model>, AppError> {
    let todo = service::get_todo_by_id(&state.db, id).await?;
    match todo {
        Some(todo) => Ok(Json(todo)),
        None => Err(AppError::NotFound),
    }
}

#[utoipa::path(
    get,
    path = "/todos",
    responses(
        (status = 200, description = "List of todos", body = [entity::Model]),
        (status = 500, description = "Internal server error")
    )
)]
#[axum::debug_handler]
pub async fn get_all_todos(
    State(state): State<AppState>,
) -> Result<Json<Vec<entity::Model>>, AppError> {
    let todos = service::get_all_todos(&state.db).await?;
    Ok(Json(todos))
}

#[utoipa::path(
    put,
    path = "/todos/{id}",
    params(
        ("id" = i32, Path, description = "Todo id")
    ),
    request_body = TodoPayload,
    responses(
        (status = 200, description = "Todo updated successfully", body = entity::Model),
        (status = 404, description = "Todo not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[axum::debug_handler]
pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(data): Json<TodoPayload>,
) -> Result<Json<entity::Model>, AppError> {
    let todo = service::update_todo(&state.db, id, data).await?;
    Ok(Json(todo))
}

#[utoipa::path(
    delete,
    path = "/todos/{id}",
    params(
        ("id" = i32, Path, description = "Todo id")
    ),
    responses(
        (status = 204, description = "Todo deleted successfully"),
        (status = 404, description = "Todo not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[axum::debug_handler]
pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    service::delete_todo(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub enum AppError {
    ServiceError(ServiceError),
    NotFound,
}

impl From<ServiceError> for AppError {
    fn from(error: ServiceError) -> Self {
        AppError::ServiceError(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::ServiceError(error) => {
                let body = Json(serde_json::json!({ "error": error.to_string() }));
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
            AppError::NotFound => (StatusCode::NOT_FOUND).into_response(),
        }
    }
}
