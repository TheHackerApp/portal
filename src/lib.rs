use axum::{routing::get, Router};
use database::PgPool;
use svix::api::Svix;

mod handlers;
mod state;

use state::AppState;

/// Setup the routes
pub fn router(db: PgPool, mail: mail::Client, svix: Svix) -> Router {
    let router = Router::new()
        .route(
            "/graphql",
            get(handlers::playground).post(handlers::graphql),
        )
        .nest("/webhooks", handlers::webhooks())
        .with_state(AppState::new(db, mail, svix))
        .layer(logging::http());

    Router::new()
        .route("/health", get(handlers::health))
        .merge(router)
}
