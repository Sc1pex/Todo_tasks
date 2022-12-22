use crate::schema::users;
use async_graphql::{InputObject, SimpleObject};
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, SimpleObject)]
pub struct User {
    #[graphql(skip)]
    pub id: Uuid,

    pub username: String,

    pub password: String,

    #[graphql(skip)]
    pub created_at: chrono::DateTime<Utc>,

    #[graphql(skip)]
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Insertable, InputObject)]
#[diesel(table_name = users)]
pub struct NewUser {
    #[graphql(skip)]
    pub id: Uuid,

    pub username: String,

    pub password: String,
}
