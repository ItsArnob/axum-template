use axum::{
    extract::State,
    routing::post,
    Router, response::IntoResponse,
};
use mongodb::bson::doc;

use crate::{
    app::AppState,
    models::User,
    utils::{error::Error, extractors::JsonExtractor},
};

pub fn build_router() -> Router<AppState> {
    Router::new()
        .route("/", post(index))
}

#[axum::debug_handler]
async fn index(State(state): State<AppState>, JsonExtractor(payload): JsonExtractor<Test>) -> impl IntoResponse {
    let insert_doc = state
        .db
        .users
        .insert_one(
            User {
                id: None,
                username: payload.username,
            },
            None,
        )
        .await?;
    match state.db.users.find_one(doc! { "_id": &insert_doc.inserted_id }, None).await? {

        Some(doc) => Ok(axum::Json(doc)),
        None => {
            tracing::error!("This is impossible!");
            Err(Error::Unknown)
        }

    }
}



// Request body structs

#[derive(Debug, serde::Deserialize, validator::Validate)]
struct Test {
    #[validate(length(min = 5, message = "Cannot be empty"))]
    pub username: String,
}
