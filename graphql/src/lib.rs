use async_graphql::{
    extensions::Analyzer, EmptySubscription, SDLExportOptions, Schema as BaseSchema, SchemaBuilder,
};
use database::PgPool;
use std::sync::Arc;
use svix::api::Svix;

mod errors;
mod mutation;
mod query;
mod webhooks;

use mutation::Mutation;
use query::Query;
#[cfg(feature = "schema")]
pub use webhooks::Payload;

/// The graphql schema for the service
pub type Schema = BaseSchema<Query, Mutation, EmptySubscription>;

/// Create a schema builder with the necessary extensions
fn builder() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(Query, Mutation::default(), EmptySubscription)
        .enable_federation()
        .extension(logging::GraphQL)
        .extension(Analyzer)
}

/// Build the schema with the necessary data
pub fn schema(db: PgPool, svix: Svix) -> Schema {
    builder().data(db).data(Arc::new(svix)).finish()
}

/// Export the GraphQL schema
pub fn sdl() -> String {
    let options = SDLExportOptions::new()
        .federation()
        .include_specified_by()
        .compose_directive();
    builder().finish().sdl_with_options(options)
}
