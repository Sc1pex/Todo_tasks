use crate::entities::{prelude::*, users};
use async_graphql::{Context, Object};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

pub struct Query;

#[Object]
impl Query {
    async fn get_all<'a>(&self, ctx: &Context<'a>) -> Result<Vec<users::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Users::find().all(db).await
    }
}
