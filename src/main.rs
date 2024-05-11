use eyre::WrapErr;
use tracing::info;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    dotenv()?;

    logging::config().init()?;

    info!("Hello, world!");

    Ok(())
}

/// Load environment variables from a .env file, if it exists.
fn dotenv() -> eyre::Result<()> {
    if let Err(error) = dotenvy::dotenv() {
        if !error.not_found() {
            return Err(error).wrap_err("failed to load .env");
        }
    }

    Ok(())
}
