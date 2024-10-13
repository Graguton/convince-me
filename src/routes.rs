use axum::{
    extract::State,
    routing::{post, delete},
    Router, Json,
};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::models::{AppState, ChatRequest, ChatResponse, DeleteResponse};
use crate::error::AppError;

pub fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/chat", post(chat_handler))
        .route("/users/:user_id", delete(delete_user_handler))
}

async fn chat_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, AppError> {
    // Get the user's message history
    let user_messages = {
        let messages = state.messages.lock().map_err(|_| anyhow::anyhow!("Failed to acquire lock"))?;
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
        let mut messages = state.messages.lock().map_err(|_| anyhow::anyhow!("Failed to acquire lock"))?;
        let user_messages = messages.entry(request.user_id).or_insert_with(Vec::new);
        user_messages.push(request.message);
        user_messages.push(response.clone());
    }

    Ok(Json(ChatResponse { response }))
}

async fn delete_user_handler(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(user_id): axum::extract::Path<String>,
) -> Result<Json<DeleteResponse>, AppError> {
    let mut messages = state.messages.lock().map_err(|_| anyhow::anyhow!("Failed to acquire lock"))?;
    if messages.remove(&user_id).is_some() {
        Ok(Json(DeleteResponse {
            message: format!("User {} has been deleted", user_id),
        }))
    } else {
        Err(anyhow::anyhow!("User not found").into())
    }
}