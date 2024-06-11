use axum::{routing::get, Router};
use database::PgPool;
use svix::api::Svix;

mod handlers;
mod state;

use state::AppState;

/// Setup the routes
pub fn router(db: PgPool, svix: Svix) -> Router {
    let router = Router::new()
        .route(
            "/graphql",
            get(handlers::playground).post(handlers::graphql),
        )
        .with_state(AppState::new(db, svix))
        .layer(logging::http());

    Router::new()
        .route("/health", get(handlers::health))
        .merge(router)
}
