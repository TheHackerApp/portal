use chrono::Utc;
use graphql::Payload;
use schemars::{
    gen::SchemaGenerator,
    schema::{RootSchema, Schema},
    JsonSchema,
};
use serde::Serialize;

pub struct Webhook {
    kind: &'static str,
    description: &'static str,
    schema: Schema,
    example: serde_json::Value,
}

impl Webhook {
    pub fn new<T>(
        generator: &mut SchemaGenerator,
        kind: &'static str,
        description: &'static str,
        example: &T,
    ) -> Self
    where
        T: JsonSchema + Serialize,
    {
        let example = Payload {
            type_: kind,
            for_: "wafflehacks-2024",
            object: example,
            at: Utc::now(),
        };

        Self {
            kind,
            description,
            schema: Payload::<T>::json_schema(generator),
            example: serde_json::to_value(example).expect("schemas must serialize"),
        }
    }
}

pub fn generate(webhooks: &[Webhook]) -> RootSchema {
    let mut schema = RootSchema::default();
    schema.meta_schema = Some(String::from(
        "https://spec.openapis.org/oas/3.1/schema/latest#/definitions/Schema",
    ));

    let webhooks = webhooks
        .into_iter()
        .map(|webhook| {
            let payload = serde_json::to_value(&webhook.schema).expect("schemas must serialize");
            let schema = serde_json::json!({
                "post": {
                    "operationId": webhook.kind,
                    "description": webhook.description,
                    "requestBody": {
                        "content": {
                            "application/json": {
                                "schema": payload,
                                "example": webhook.example,
                            }
                        }
                    }
                }
            });
            (webhook.kind, schema)
        })
        .collect();
    schema
        .schema
        .extensions
        .insert(String::from("webhooks"), webhooks);

    schema
}
