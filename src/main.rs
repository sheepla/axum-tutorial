mod database;
mod entity;
mod handler;
mod service;

use crate::handler::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handler::create_todo,
        handler::get_todo_by_id,
        handler::get_all_todos,
        handler::update_todo,
        handler::delete_todo,
    ),
    components(
        schemas(entity::TodoPayload, entity::Model)
    ),
    tags(
        (name = "Todo", description = "Todo items management API")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_todos=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = database::establish_connection().await?;

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/todos", post(handler::create_todo))
        .route("/todos", get(handler::get_all_todos))
        .route("/todos/:id", get(handler::get_todo_by_id))
        .route("/todos/:id", put(handler::update_todo))
        .route("/todos/:id", delete(handler::delete_todo))
        .layer(CorsLayer::new().allow_origin(Any))
        .with_state(AppState { db });

    let listen_address = env::var("LISTEN_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let addr = listen_address.parse::<SocketAddr>()?;
    info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
