use axum::extract::State;
use axum::routing::get;
use axum::Router;
use rqlite_rs::prelude::*;

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(example_endpoint));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, router).await.unwrap()
}

async fn example_endpoint(State(db): State<RqliteClient>) -> &'static str {
    ""
}
