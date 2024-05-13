use eyre::WrapErr;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions,
};
use std::{str::FromStr, time::Duration};
use tracing::{info, instrument, log::LevelFilter};

mod application;

pub use application::{Application, ApplicationStatus, Education, Gender, RaceEthnicity};
pub use sqlx::{Error as SqlxError, PgPool};

/// Connect to the database and ensure it works
#[instrument(skip_all)]
pub async fn connect(url: &str) -> eyre::Result<PgPool> {
    let options = PgConnectOptions::from_str(url)
        .wrap_err("invalid database url format")?
        .log_statements(LevelFilter::Info)
        .log_slow_statements(LevelFilter::Warn, Duration::from_secs(5));

    let db = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(10))
        .connect_with(options)
        .await
        .wrap_err("failed to connect to the database")?;

    info!("database connected");
    Ok(db)
}
