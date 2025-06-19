use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

mod anime;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(pong))
        .merge(anime::data_route::routes());

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn pong() -> Json<Value> {
    Json(json!({
        "pong": "!",
    }))
}
