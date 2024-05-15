use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract::State, http::StatusCode, response::Html};
use context::{Scope, User};
use tracing::instrument;

/// Handle graphql requests
#[instrument(name = "graphql", skip_all)]
pub(crate) async fn graphql(
    State(schema): State<graphql::Schema>,
    scope: Scope,
    user: User,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let req = req.into_inner().data(scope).data(user);
    schema.execute(req).await.into()
}

/// Serve the GraphQL playground for development
#[instrument(name = "playground")]
pub(crate) async fn playground() -> Html<String> {
    let config = GraphQLPlaygroundConfig::new("/graphql").title("Identity Playground");
    Html(playground_source(config))
}

/// Check that the service is alive
pub(crate) async fn health() -> StatusCode {
    StatusCode::NO_CONTENT
}
