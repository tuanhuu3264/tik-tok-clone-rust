use axum::{Router, routing::get};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use crate::presentation::graphql::AppSchema;
use crate::di::AppContext;
use std::sync::Arc;

pub async fn create_router(context: Arc<AppContext>) -> Result<Router, Box<dyn std::error::Error>> {
    let schema = crate::presentation::graphql::create_schema((*context).clone());
    
    let router = Router::new()
        .route("/graphql", axum::routing::post(graphql_handler))
        .route("/graphiql", get(graphql_playground))
        .with_state(schema);

    Ok(router)
}

async fn graphql_handler(
    axum::extract::State(schema): axum::extract::State<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl axum::response::IntoResponse {
    axum::response::Html(
        async_graphql::http::playground_source(
            async_graphql::http::GraphQLPlaygroundConfig::new("/graphql")
        )
    )
}

