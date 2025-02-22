#![allow(unused)]

use axum::extract::State;
use axum::routing::get;
use axum::Router;
use rqlite_rs::prelude::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Database(Arc<RqliteClient>);

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(example_endpoint))
        .with_state(Database::default());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

async fn example_endpoint(State(db): State<Database>) -> &'static str {
    ""
}

impl Default for Database {
    fn default() -> Self {
        let inner = RqliteClientBuilder::new()
            .known_host("localhost:4001")
            .build()
            .unwrap();

        Self(Arc::new(inner))
    }
}
