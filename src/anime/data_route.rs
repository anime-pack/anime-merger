use axum::routing::get;
use axum::{Json, Router};
use jikan_moe::JikanClient;
use serde::Deserialize;
use serde_json::{json, Value};

pub fn routes() -> Router {
	Router::new().route("/anime", get(anime_data))
}

#[derive(Debug, Deserialize)]
struct AnimeRequestPayload {
    id: i32,
}

async fn anime_data(
	payload: Json<AnimeRequestPayload>,
) -> Result<Json<Value> , ()> {

    let client = JikanClient::new();
    let anime = client.get_anime_full(payload.id)
    .await
    .map_err(|_| ());

	let body = Json(json!({
		"data": anime,
	}));

	Ok(body)
}