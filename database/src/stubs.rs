use async_graphql::SimpleObject;

/// Stub for an event in the identity service
#[derive(SimpleObject)]
#[graphql(unresolvable)]
pub(crate) struct Event<'e> {
    pub slug: &'e str,
}

/// Stub for a participant in the identity service
#[derive(SimpleObject)]
#[graphql(unresolvable)]
pub(crate) struct Participant {
    pub id: i32,
}
