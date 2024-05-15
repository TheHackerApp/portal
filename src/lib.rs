use axum::{routing::get, Router};
use database::PgPool;

mod handlers;
mod state;

use state::AppState;

/// Setup the routes
pub fn router(db: PgPool) -> Router {
    let router = Router::new()
        .route(
            "/graphql",
            get(handlers::playground).post(handlers::graphql),
        )
        .with_state(AppState::new(db))
        .layer(logging::http());

    Router::new()
        .route("/health", get(handlers::health))
        .merge(router)
}
