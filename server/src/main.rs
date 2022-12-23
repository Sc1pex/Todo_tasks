use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

mod model;
mod mutation;
mod query;
mod schema;

type SchemaType = Schema<query::Query, mutation::Mutation, EmptySubscription>;
type DbType = Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[post("/")]
async fn index(schema: web::Data<SchemaType>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

fn create_db() -> DbType {
    let manager = ConnectionManager::<PgConnection>::new(
        std::env::var("DATABASE_URL").expect("Database url is not specified"),
    );
    Pool::builder()
        .max_size(
            std::env::var("MAX_DB_CONNECTIONS")
                .expect("Max database connections not specified")
                .parse()
                .expect("Max database connection is not a number"),
        )
        .build(manager)
        .expect("Could not build connection pool")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();

    let db_pool = create_db();
    let schema = Schema::build(query::Query, mutation::Mutation, EmptySubscription)
        .data(db_pool.clone())
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
    .await
}
