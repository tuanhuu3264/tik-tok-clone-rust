use axum::{Router, Server};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use crate::infrastructure::config::Config;
use crate::presentation::routes::create_router;

pub async fn create_server(config: Config) -> Result<Server<axum::extract::connect_info::IntoMakeServiceWithConnectInfo<Router, std::net::SocketAddr>, Router>, Box<dyn std::error::Error>> {
    let app = create_router().await?;

    let app = app
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
        );

    let addr = format!("{}:{}", config.server.host, config.server.port)
        .parse()?;

    let server = Server::bind(&addr).serve(app.into_make_service());

    tracing::info!("Server running on {}", addr);

    Ok(server)
}

