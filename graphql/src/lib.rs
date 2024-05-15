use async_graphql::{
    extensions::Analyzer, EmptyMutation, EmptySubscription, SDLExportOptions, Schema as BaseSchema,
    SchemaBuilder,
};
use database::PgPool;

mod query;

use query::Query;

/// The graphql schema for the service
pub type Schema = BaseSchema<Query, EmptyMutation, EmptySubscription>;

/// Create a schema builder with the necessary extensions
fn builder() -> SchemaBuilder<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .enable_federation()
        .extension(logging::GraphQL)
        .extension(Analyzer)
}

/// Build the schema with the necessary data
pub fn schema(db: PgPool) -> Schema {
    builder().data(db).finish()
}

/// Export the GraphQL schema
pub fn sdl() -> String {
    let options = SDLExportOptions::new()
        .federation()
        .include_specified_by()
        .compose_directive();
    builder().finish().sdl_with_options(options)
}
