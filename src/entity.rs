use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "todos")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[schema(read_only)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    #[schema(example = "Buy milk")]
    pub title: String,
    #[schema(example = false)]
    pub completed: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TodoPayload {
    #[schema(example = "Buy milk")]
    pub title: String,
    #[schema(example = false)]
    pub completed: bool,
}
