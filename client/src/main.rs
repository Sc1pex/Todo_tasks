use graphql_client::{reqwest::post_graphql, GraphQLQuery, Response};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "graphql/queries/users.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
struct Users;

pub async fn query(url: String) -> serde_json::Value {
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(&Users::build_query(users::Variables {}))
        .send()
        .await
        .expect("Failed to send request");

    let res: Response<users::ResponseData> = res.json().await.expect("Failed to get json");
    serde_json::to_value(&res).unwrap()
}

#[tokio::main]
async fn main() {
    let url = std::env::args().nth(1).unwrap();

    let res = query(url).await;
    println!("{}", res);
}
