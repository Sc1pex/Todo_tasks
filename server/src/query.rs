use crate::{model::User, schema::users::dsl::*, DbType};
use async_graphql::{Context, Object};
use diesel::prelude::*;

pub struct Query;

#[Object]
impl Query {
    async fn get_all<'a>(&self, ctx: &Context<'a>) -> Vec<User> {
        let pool = ctx
            .data::<DbType>()
            .expect("No data given to graphql schema");
        let conn = &mut pool.get().unwrap();

        users.load(conn).unwrap()
    }
}
