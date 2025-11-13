use axum::Router;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use crate::infrastructure::config::Config;
use crate::presentation::routes::create_router;
use crate::di::AppContext;
use std::sync::Arc;

pub async fn create_server(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let context = Arc::new(AppContext::new(config.clone()).await?);
    let app = create_router(context).await?;

    let app = app
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
        );

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(&addr).await?;

    tracing::info!("Server running on {}", addr);
    tracing::info!("GraphQL endpoint: http://{}/graphql", addr);
    tracing::info!("GraphiQL playground: http://{}/graphiql", addr);

    axum::serve(listener, app).await?;
    Ok(())
}

