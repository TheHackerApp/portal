use reqwest::header::{HeaderMap, HeaderValue};
use serde::Serialize;
use std::collections::HashMap;
use tracing::instrument;

/// An email client for sending templated messages to one or more participants
#[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    /// Create a new client
    pub fn new(token: String) -> Self {
        let client = reqwest::Client::builder()
            .user_agent("the-hacker-app/portal")
            .danger_accept_invalid_certs(true)
            .default_headers({
                let mut headers = HeaderMap::with_capacity(1);
                headers.insert("Accept", HeaderValue::from_static("application/json"));
                headers.insert(
                    "X-Postmark-Server-Token",
                    HeaderValue::try_from(token).expect("token must be valid"),
                );
                headers
            })
            .build()
            .expect("client must build");

        Self { client }
    }

    /// Send a templated email to the specified address
    #[instrument(skip(self))]
    pub async fn send_templated(&self, id: &str, to: &str) -> Result<(), reqwest::Error> {
        self.client
            .post("https://api.postmarkapp.com/email/withTemplate")
            .json(&SendTemplateRequest {
                template_alias: id,
                template_model: HashMap::with_capacity(0),
                to,
                from: "apply@wafflehacks.org",
                reply_to: "operations@wafflehacks.org",
                track_opens: true,
                message_stream: "outbound",
            })
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct SendTemplateRequest<'a> {
    template_alias: &'a str,
    template_model: HashMap<String, String>,
    to: &'a str,
    from: &'static str,
    reply_to: &'static str,
    track_opens: bool,
    message_stream: &'static str,
}
