use chrono::{DateTime, Utc};
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::Serialize;
use svix::api::{MessageIn, Svix};
use tracing::{error, instrument};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Payload<'p, T> {
    /// The type of webhook
    #[serde(rename = "type")]
    pub type_: &'p str,
    /// The event the webhook applies to
    #[serde(rename = "for")]
    pub for_: &'p str,
    /// The object the webhook applies to
    pub object: &'p T,
    /// When the webhook was sent
    pub at: DateTime<Utc>,
}

/// Send a webhook event
#[instrument(name = "webhook::send", skip(client, object))]
pub async fn send<T>(client: &Svix, event_type: &str, event_slug: &str, object: &T)
where
    T: Serialize,
{
    let body = serde_json::to_value(Payload {
        type_: event_type,
        for_: event_slug,
        object,
        at: Utc::now(),
    })
    .expect("must serialize");

    // TODO: spawn as a background task
    let result = client
        .message()
        .create(
            event_slug.to_owned(),
            MessageIn::new(event_type.to_owned(), body),
            None,
        )
        .await;
    if let Err(error) = result {
        error!(%error, "failed to send webhook");
    }
}
