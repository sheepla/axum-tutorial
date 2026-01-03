use crate::entity::{self, Entity as TodoEntity, Model as TodoModel, TodoPayload};
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait, Set};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    DbErr(#[from] sea_orm::DbErr),
}

pub async fn create_todo(db: &DbConn, data: TodoPayload) -> Result<TodoModel, ServiceError> {
    let new_todo = entity::ActiveModel {
        title: Set(data.title.to_owned()),
        completed: Set(data.completed.to_owned()),
        ..Default::default()
    };
    let todo = new_todo.insert(db).await?;
    Ok(todo)
}

pub async fn get_todo_by_id(db: &DbConn, id: i32) -> Result<Option<TodoModel>, ServiceError> {
    let todo = TodoEntity::find_by_id(id).one(db).await?;
    Ok(todo)
}

pub async fn get_all_todos(db: &DbConn) -> Result<Vec<TodoModel>, ServiceError> {
    let todos = TodoEntity::find().all(db).await?;
    Ok(todos)
}

pub async fn update_todo(
    db: &DbConn,
    id: i32,
    data: TodoPayload,
) -> Result<TodoModel, ServiceError> {
    let mut todo: entity::ActiveModel = match get_todo_by_id(db, id).await? {
        Some(todo) => todo.into(),
        None => {
            return Err(ServiceError::DbErr(sea_orm::DbErr::RecordNotFound(
                "Todo not found".to_string(),
            )))
        }
    };
    todo.title = Set(data.title.to_owned());
    todo.completed = Set(data.completed.to_owned());
    let todo = todo.update(db).await?;
    Ok(todo)
}

pub async fn delete_todo(db: &DbConn, id: i32) -> Result<(), ServiceError> {
    let todo: entity::ActiveModel = match get_todo_by_id(db, id).await? {
        Some(todo) => todo.into(),
        None => {
            return Err(ServiceError::DbErr(sea_orm::DbErr::RecordNotFound(
                "Todo not found".to_string(),
            )))
        }
    };
    todo.delete(db).await?;
    Ok(())
}
