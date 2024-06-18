use eyre::WrapErr;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions,
};
use std::{
    fmt::{Debug, Display, Formatter},
    ops::Deref,
    str::FromStr,
    sync::Arc,
    time::Duration,
};
use tracing::{info, instrument, log::LevelFilter};

#[macro_use]
mod macros;
mod application;
mod check_in;
mod draft_application;
mod school;
#[cfg(feature = "graphql")]
mod stubs;

pub use application::{Application, ApplicationStatus, Education, Gender, RaceEthnicity, Referrer};
pub use check_in::CheckIn;
pub use draft_application::DraftApplication;
pub use school::School;
pub use sqlx::{Error as SqlxError, PgPool};

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

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

/// Represents the different way the database can fail
#[derive(Clone)]
pub struct Error(Arc<SqlxError>);

impl Error {
    /// Returns whether the error kind is a violation of a unique/primary key constraint.
    pub fn is_unique_violation(&self) -> bool {
        match self.0.as_ref() {
            SqlxError::Database(e) => e.is_unique_violation(),
            _ => false,
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl Deref for Error {
    type Target = SqlxError;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<SqlxError> for Error {
    fn as_ref(&self) -> &SqlxError {
        &self.0
    }
}

impl From<SqlxError> for Error {
    fn from(error: SqlxError) -> Self {
        Self(Arc::new(error))
    }
}

#[cfg(feature = "graphql")]
impl async_graphql::ErrorExtensions for Error {
    fn extend(&self) -> async_graphql::Error {
        use std::error::Error as _;

        match self.source() {
            Some(e) => tracing::error!(error = %self.0, source = %e, "unexpected database error"),
            None => tracing::error!(error = %self.0, "unexpected database error"),
        }

        async_graphql::Error::new("internal server error")
    }
}
