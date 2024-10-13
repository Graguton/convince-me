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
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use dotenv::dotenv;
use anyhow::Result;

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
        messages: Mutex::new(HashMap::new()),
    });

    // Build our application with a route
    let app = Router::new()
        .route("/chat", post(chat_handler))
        .nest_service("/", ServeDir::new("frontend/build"))
        .layer((
            TraceLayer::new_for_http(),
            // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
            // requests don't hang forever.
            TimeoutLayer::new(Duration::from_secs(60)),
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
    messages: Mutex<HashMap<String, Vec<String>>>,
}

// Request and response structures
#[derive(Deserialize)]
struct ChatRequest {
    user_id: String,
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
    // Get the user's message history
    let user_messages = {
        let messages = state.messages.lock().unwrap();
        messages.get(&request.user_id).cloned().unwrap_or_default()
    };

    // Create the messages array for the API request
    let mut api_messages = vec![
        json!({
            "role": "system",
            "content": "You are a helpful assistant."
        }),
    ];

    // Add previous messages to the API request
    for (i, message) in user_messages.iter().enumerate() {
        api_messages.push(json!({
            "role": if i % 2 == 0 { "user" } else { "assistant" },
            "content": message
        }));
    }

    // Add the current user message
    api_messages.push(json!({
        "role": "user",
        "content": request.message
    }));

    let api_response = state
        .client
        .post(&state.server_addr)
        .header("Authorization", format!("Bearer {}", state.api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": "",
            "messages": api_messages
        }))
        .send()
        .await?
        .json::<Value>()
        .await?;

    let response = api_response["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Failed to extract response content"))?
        .to_string();

    // Store the user's message and the assistant's response
    {
        let mut messages = state.messages.lock().unwrap();
        let user_messages = messages.entry(request.user_id).or_insert_with(Vec::new);
        user_messages.push(request.message);
        user_messages.push(response.clone());
    }

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