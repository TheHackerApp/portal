use async_graphql::SimpleObject;

/// Stub for an event in the identity service
#[derive(SimpleObject)]
#[graphql(unresolvable)]
pub(crate) struct Event<'e> {
    pub slug: &'e str,
}

/// Stub for a participant in the identity service
#[derive(SimpleObject)]
#[graphql(unresolvable = "event { slug } user { id }")]
pub(crate) struct Participant<'p> {
    pub event: Event<'p>,
    pub user: User,
}

impl<'p> Participant<'p> {
    /// Create a new participant reference
    pub fn new(user_id: i32, event: &'p str) -> Self {
        Self {
            event: Event { slug: event },
            user: User { id: user_id },
        }
    }
}

/// Stub for a user in the identity service
#[derive(SimpleObject)]
#[graphql(unresolvable)]
pub(crate) struct User {
    pub id: i32,
}
