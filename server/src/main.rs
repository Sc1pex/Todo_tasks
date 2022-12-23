use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use anyhow::Result;
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use migrations::Migrator;
use sea_orm::Database;
use sea_orm_migration::MigratorTrait;

mod entities;
mod migrations;
mod mutation;
mod query;

type SchemaType = Schema<query::Query, mutation::Mutation, EmptySubscription>;

#[get("/")]
async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish())
}

#[post("/")]
async fn index(schema: web::Data<SchemaType>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();

    let db_url = std::env::var("DATABASE_URL")?;
    let db = Database::connect(db_url).await?;
    Migrator::refresh(&db).await?;

    let schema = Schema::build(query::Query, mutation::Mutation, EmptySubscription)
        .data(db)
        .finish();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(index)
            .service(graphql_playground)
    })
    .bind((
        "0.0.0.0",
        std::env::var("PORT")
            .expect("Port not specified")
            .parse()
            .expect("Port is not a number"),
    ))?
    .run()
    .await?;

    Ok(())
}
