use axum::routing::post;
use axum::{Json, Router};
use jikan_moe::JikanClient;
use serde::Deserialize;
use serde_json::{Value, json};

pub fn routes() -> Router {
    Router::new().route("/anime", post(anime_data))
}

#[derive(Debug, Deserialize)]
struct AnimeRequestPayload {
    id: i32,
}

async fn anime_data(payload: Json<AnimeRequestPayload>) -> Result<Json<Value>, ()> {
    let client = JikanClient::new();
    let anime = client.get_anime_full(payload.id).await.map_err(|_| ());

    let body = Json(json!({
        "data": anime,
    }));

    Ok(body)
}
