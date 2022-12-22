use crate::{
    model::{NewUser, User},
    schema::users::dsl::*,
    DbType,
};
use async_graphql::{Context, Object};
use diesel::prelude::*;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_user<'a>(&self, ctx: &Context<'a>, mut user: NewUser) -> Result<User, String> {
        let pool = ctx
            .data::<DbType>()
            .expect("No data given to graphql schema");
        let conn = &mut pool.get().unwrap();

        user.id = uuid::Uuid::new_v4();

        diesel::insert_into(users)
            .values(&user)
            .get_result(conn)
            .map_err(|e| e.to_string())
    }
}
