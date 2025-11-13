mod application;
mod domain;
mod infrastructure;
mod presentation;
mod di;

use infrastructure::config::Config;
use presentation::server::create_server;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = Config::load()?;

    create_server(config).await?;

    Ok(())
}

