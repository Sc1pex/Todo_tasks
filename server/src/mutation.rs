use crate::entities::{prelude::*, users};
use async_graphql::{Context, Object};
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use thiserror::Error;

#[derive(Error, Debug)]
enum UserError {
    #[error("Username Error: {0}")]
    UsernameErr(String),

    #[error("Password Error: {0}")]
    PasswordErr(String),

    #[error("Database Error")]
    DbErr,
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn register<'a>(
        &self,
        ctx: &Context<'a>,
        user: users::UserInputObject,
    ) -> Result<users::Model, UserError> {
        if user.username.len() < 3 {
            return Err(UserError::UsernameErr("Username too short".to_owned()));
        }
        if user.password.len() < 4 {
            return Err(UserError::PasswordErr("Password too short".to_owned()));
        }

        let salt = std::env::var("PASSWORD_SALT").expect("Password hashing salt not specified");
        let password = argon2::hash_encoded(
            user.password.as_bytes(),
            salt.as_bytes(),
            &argon2::Config::default(),
        )
        .unwrap();

        let db = ctx.data::<DatabaseConnection>().unwrap();

        let user = users::ActiveModel {
            username: ActiveValue::Set(user.username),
            password: ActiveValue::Set(password),
            id: ActiveValue::Set(uuid::Uuid::new_v4()),
            created_at: ActiveValue::Set(chrono::Utc::now().into()),
            updated_at: ActiveValue::Set(chrono::Utc::now().into()),
        };

        let res = Users::insert(user).exec(db).await.map_err(|e| {
            let e = e.to_string();
            if e.contains("duplicate key") {
                UserError::UsernameErr("Username already exists".to_owned())
            } else {
                log::error!("{}", e);
                UserError::DbErr
            }
        })?;

        Users::find_by_id(res.last_insert_id)
            .one(db)
            .await
            .map(|u| u.unwrap())
            .map_err(|_| UserError::DbErr)
    }

    async fn login<'a>(
        &self,
        ctx: &Context<'a>,
        user: users::UserInputObject,
    ) -> Result<users::Model, UserError> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        let db_user = Users::find()
            .filter(users::Column::Username.eq(user.username))
            .one(db)
            .await
            .map(|u| u.unwrap())
            .map_err(|_| UserError::UsernameErr("Username not found".to_owned()))?;

        if argon2::verify_encoded(&db_user.password, user.password.as_bytes()).unwrap() {
            Ok(db_user)
        } else {
            Err(UserError::PasswordErr("Wrong password".to_owned()))
        }
    }
}
