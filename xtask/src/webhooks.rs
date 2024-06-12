use eyre::{eyre, WrapErr};
use schemars::gen::SchemaSettings;
use std::{fs::OpenOptions, io::Write, path::PathBuf};
use tracing::info;

mod examples;
mod openapi;

use openapi::Webhook;

pub fn run(args: Args) -> eyre::Result<()> {
    if args.output.exists() && !args.force {
        return Err(eyre!("file already exists, use --force to overwrite"));
    }

    let mut generator = SchemaSettings::openapi3()
        .with(|settings| {
            settings.option_nullable = true;
            settings.inline_subschemas = true;
        })
        .into_generator();

    let application = examples::application();
    let schema = openapi::generate(&[Webhook::new(
        &mut generator,
        "application.submitted",
        "Published when a participant submits an application to your event.",
        &application,
    )]);

    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .create_new(!args.force)
        .open(&args.output)
        .wrap_err("failed to open output")?;

    output.write_all(serde_json::to_string_pretty(&schema).unwrap().as_bytes())?;

    info!(path = %args.output.display(), "successfully exported schema");

    Ok(())
}

#[derive(clap::Args, Debug)]
pub struct Args {
    /// Where to save the schemas
    #[arg(default_value = "./webhooks.json")]
    output: PathBuf,
    /// Whether to overwrite the output file if it already exists
    #[arg(short, long, default_value_t)]
    force: bool,
}
