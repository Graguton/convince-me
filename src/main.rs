use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Duration;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create a reqwest client
    let client = Client::new();

    // Create the application state
    let state = Arc::new(AppState {
        client,
        api_key: std::env::var("API_KEY").expect("API_KEY must be set"),
        server_addr: std::env::var("LLM_SERVER_ADDR").expect("LLM_SERVER_ADDR must be set"),
    });

    // Build our application with a route
    let app = Router::new()
        .route("/chat", post(chat_handler))
        .nest_service("/", ServeDir::new("path/to/your/frontend/build"))
        .layer((
            TraceLayer::new_for_http(),
            // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
            // requests don't hang forever.
            TimeoutLayer::new(Duration::from_secs(10)),
        ))
        .with_state(state);

    // Run our application
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

// Application state
struct AppState {
    client: Client,
    api_key: String,
    server_addr: String,
}

// Request and response structures
#[derive(Deserialize)]
struct ChatRequest {
    message: String,
}

#[derive(Serialize)]
struct ChatResponse {
    response: String,
}

// Chat handler
async fn chat_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, AppError> {
    let api_response = state
        .client
        .post(&state.server_addr)
        .header("Authorization", format!("Bearer {}", state.api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                {
                    "role": "system",
                    "content": "You are a helpful assistant."
                },
                {
                    "role": "user",
                    "content": request.message
                }
            ]
        }))
        .send()
        .await?
        .json::<Value>()
        .await?;

    let response = api_response["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Failed to extract response content"))?
        .to_string();

    Ok(Json(ChatResponse { response }))
}

// Error handling
struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}