use clap::Parser;
use eyre::{eyre, WrapErr};
use logging::OpenTelemetryProtocol;
use std::net::SocketAddr;
use svix::api::Svix;
use tokio::{net::TcpListener, signal};
use tracing::{info, Level};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    dotenv()?;

    let config = Config::parse();

    let mut logging = logging::config().default_directive(config.log_level);
    if let Some(endpoint) = &config.opentelemetry_endpoint {
        logging = logging.opentelemetry(config.opentelemetry_protocol, endpoint);
    }
    logging.init()?;

    let db = database::connect(&config.database_url).await?;
    let svix = Svix::new(config.svix_api_key, None);

    let router = portal::router(db, svix);

    let listener = TcpListener::bind(&config.address)
        .await
        .wrap_err("failed to bind listener")?;
    info!(address = %config.address, "listening and ready to handle requests");

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown())
        .await
        .wrap_err("failed to start server")?;

    Ok(())
}

/// Setup hyper graceful shutdown for SIGINT (ctrl+c) and SIGTERM
async fn shutdown() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install ctrl+c handler")
    };
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install sigterm handler")
            .recv()
            .await
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("server successfully shutdown");
    info!("goodbye! o/");
}

/// The participant application, attendance, and communication platform.
#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Config {
    /// The address for the server to listen on
    #[arg(long, default_value = "127.0.0.1:7878", env = "ADDRESS")]
    address: SocketAddr,

    /// The database to run migrations on
    #[arg(long, env = "DATABASE_URL")]
    database_url: String,

    /// The Svix environment API key
    #[arg(long, env = "SVIX_API_KEY")]
    svix_api_key: String,

    /// The default level to log at
    #[arg(long, default_value_t = Level::INFO, env = "LOG_LEVEL")]
    log_level: Level,

    /// The OpenTelemetry endpoint to send traces to
    #[arg(long, env = "OTEL_EXPORTER_OTLP_ENDPOINT")]
    opentelemetry_endpoint: Option<String>,

    /// The protocol to use when exporting OpenTelemetry traces
    #[arg(
        long,
        default_value = "grpc",
        value_parser = opentelemetry_protocol_parser,
        env = "OTEL_EXPORTER_OTLP_PROTOCOL",
    )]
    opentelemetry_protocol: OpenTelemetryProtocol,
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

/// Parse the OpenTelemetry protocol from a command line argument
fn opentelemetry_protocol_parser(raw: &str) -> eyre::Result<OpenTelemetryProtocol> {
    match raw.to_lowercase().as_str() {
        "grpc" => Ok(OpenTelemetryProtocol::Grpc),
        "http" | "http/protobuf" => Ok(OpenTelemetryProtocol::HttpBinary),
        _ => Err(eyre!(
            "invalid exporter protocol, must be one of: 'grpc' or 'http/protobuf'"
        )),
    }
}
