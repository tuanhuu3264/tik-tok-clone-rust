mod application;
mod domain;
mod infrastructure;

use infrastructure::config::Config;
use application::workers::start_workers;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = Config::load()?;
    start_workers(config).await?;

    Ok(())
}

