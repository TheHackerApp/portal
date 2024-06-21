use axum::{
    extract::{FromRef, State},
    http::StatusCode,
    routing::post,
    Json, Router,
};
use database::{Email, PgPool};
use serde::Deserialize;
use tracing::{error, instrument};

/// Create the webhook router
pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    PgPool: FromRef<S>,
{
    Router::new().route("/participant", post(participant))
}

#[derive(Debug, Deserialize)]
struct Participant {
    id: i32,
    primary_email: String,
}

/// Ensure a participant's details are in sync
#[instrument(name = "webhooks::participant", skip(db))]
async fn participant(State(db): State<PgPool>, participant: Json<Participant>) -> StatusCode {
    let result = Email::upsert(participant.id, &participant.primary_email, &db).await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(error) => {
            error!(participant.id, %error, "failed to update email");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
