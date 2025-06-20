use axum::routing::post;
use axum::{Json, Router};
use jikan_moe::JikanClient;
use serde::Deserialize;
use serde_json::{Value, json};

pub fn routes() -> Router {
    Router::new().route("/manga", post(manga_data))
}

#[derive(Debug, Deserialize)]
struct MangaRequestPayload {
    id: i32,
}

async fn manga_data(payload: Json<MangaRequestPayload>) -> Result<Json<Value>, ()> {
    let client = JikanClient::new();
    let manga = client.get_manga_full(payload.id).await.map_err(|_| ());

    let body = Json(json!({
        "data": manga,
    }));

    Ok(body)
}
