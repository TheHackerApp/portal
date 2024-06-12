use chrono::{DateTime, Utc};
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::Serialize;
use std::sync::Arc;
use svix::api::{MessageIn, Svix};
use tracing::{error, info_span, instrument, Instrument};

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
pub async fn send<T>(client: &Arc<Svix>, event_type: &str, event_slug: &str, object: &T)
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

    let client = client.clone();
    let task_event_slug = event_slug.to_owned();
    let task_event_type = event_type.to_owned();
    tokio::task::spawn(
        async move {
            let result = client
                .message()
                .create(task_event_slug, MessageIn::new(task_event_type, body), None)
                .await;
            if let Err(error) = result {
                error!(%error, "failed to send webhook");
            }
        }
        .instrument(info_span!("webhook::send::task", kind = %event_type, slug = %event_slug)),
    );
}
