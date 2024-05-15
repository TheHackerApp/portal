use async_graphql::Object;

pub struct Query;

#[Object]
impl Query {
    async fn test(&self) -> &'static str {
        "test"
    }
}
