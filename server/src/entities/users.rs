//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use async_graphql::{InputObject, SimpleObject};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(unique)]
    pub username: String,

    #[graphql(skip)]
    pub password: String,

    #[graphql(skip)]
    pub created_at: DateTimeWithTimeZone,

    #[graphql(skip)]
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(InputObject)]
pub struct UserInputObject {
    pub username: String,
    pub password: String,
}