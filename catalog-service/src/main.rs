mod application;
mod domain;
mod infrastructure;
mod presentation;

use infrastructure::config::Config;
use presentation::server::create_server;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = Config::load()?;
    let server = create_server(config).await?;
    server.await?;

    Ok(())
}

