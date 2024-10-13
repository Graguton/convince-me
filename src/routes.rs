use std::env;
use axum::{extract::State, routing::{post, delete}, Router, Json};
use serde_json::{json, Value};
use std::sync::Arc;
use reqwest::Client;
use anyhow::Result;

use crate::models::{AppState, ChatRequest, ChatResponse, DeleteResponse};
use crate::error::AppError;

lazy_static::lazy_static! {
    static ref FIRST_MESSAGE_PROMPT: String = env::var("FIRST_MESSAGE_PROMPT")
        .expect("FIRST_MESSAGE_PROMPT must be defined");
    static ref JUDGE_SYSTEM_PROMPT: String = env::var("JUDGE_SYSTEM_PROMPT")
        .expect("JUDGE_SYSTEM_PROMPT must be defined");
    static ref JUDGE_FIRST_MESSAGE_PROMPT: String = env::var("JUDGE_FIRST_MESSAGE_PROMPT")
        .expect("JUDGE_FIRST_MESSAGE_PROMPT must be defined");
    static ref JUDGE_YES_RESPONSE: String = env::var("JUDGE_YES_RESPONSE")
        .expect("JUDGE_YES_RESPONSE must be defined");
    static ref JUDGE_NO_RESPONSE: String = env::var("JUDGE_NO_RESPONSE")
        .expect("JUDGE_NO_RESPONSE must be defined");
}

pub fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/chat/:user_id", post(chat_handler))
        .route("/users/:user_id", delete(delete_user_handler))
}

async fn chat_handler(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(user_id): axum::extract::Path<String>,
    Json(request): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, AppError> {
    // Get the user's message history
    let mut user_messages = {
        let messages = state.messages.lock().map_err(|_| anyhow::anyhow!("Failed to acquire lock"))?;
        messages.get(&user_id).cloned().unwrap_or_default()
    };

    // Check if this is the first message (to be used as system prompt)
    let is_first_message = user_messages.is_empty();

    if is_first_message {
        let system_message = format!(
            "{}\n\n{}",
            request.message,
            FIRST_MESSAGE_PROMPT.as_str()
        );

        let response = chat_api_call(&state.client, &state.server_addr, &state.api_key, vec![json!({
            "role": "system",
            "content": system_message,
        })]).await?;

        // Store the system prompt in the user's message history
        user_messages.push(request.message.clone());

        user_messages.push(response.clone());

        // Store the updated message history
        {
            let mut messages = state.messages.lock().map_err(|_| anyhow::anyhow!("Failed to acquire lock"))?;
            messages.insert(user_id, user_messages);
        }

        // Return early without making an API call or checking for win
        return Ok(Json(ChatResponse { response, win: false }));
    }

    let mut api_messages = messages_to_values(user_messages);

    // Add the current user message
    api_messages.push(json!({
        "role": "user",
        "content": request.message
    }));

    let response = chat_api_call(&state.client, &state.server_addr, &state.api_key, api_messages).await?;

    // Store the user's message and the assistant's response
    let messages = {
        let mut messages = state.messages.lock().map_err(|_| anyhow::anyhow!("Failed to acquire lock"))?;
        let user_messages = messages.entry(user_id).or_insert_with(Vec::new);
        user_messages.push(request.message);
        user_messages.push(response.clone());
        user_messages.clone()
    };

    let win = check_for_win(&state.client, &state.server_addr, &state.api_key, messages).await?;

    Ok(Json(ChatResponse { response, win }))
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

async fn chat_api_call(
    client: &Client,
    server_addr: &str,
    api_key: &str,
    messages: Vec<Value>,
) -> Result<String> {
    let api_response = client
        .post(server_addr)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
                "max_tokens": 512,
                "frequency_penalty": 0.5,
                "temperature": 0.7,
                "messages": messages
            }))
        .send()
        .await?
        .json::<Value>()
        .await?;

    let response = api_response["choices"][0]["message"]["content"]
        .as_str()
        .ok_or(anyhow::anyhow!("Failed to extract response content"))?
        .to_string();

    Ok(response)
}

async fn check_for_win(
    client: &Client,
    server_addr: &str,
    api_key: &str,
    mut messages: Vec<String>,
) -> Result<bool> {
    let system_message = messages.get_mut(0).ok_or(anyhow::anyhow!("No message"))?;

    // Edit the system message
    *system_message = JUDGE_SYSTEM_PROMPT.clone(); // TODO

    messages.push(JUDGE_FIRST_MESSAGE_PROMPT.clone());

    let response = chat_api_call(client, server_addr, api_key, messages_to_values(messages)).await?;

    let yes_position = response.rfind(JUDGE_YES_RESPONSE.as_str());
    let no_position = response.rfind(JUDGE_NO_RESPONSE.as_str());

    match (yes_position, no_position) {
        (Some(yes), Some(no)) => Ok(yes > no),
        (Some(_), None) => Ok(true),
        (None, Some(_)) => Ok(false),
        (None, None) => Ok(false), // If neither YES nor NO is found, assume not convinced
    }
}

fn messages_to_values(messages: Vec<String>) -> Vec<Value> {
    messages
        .into_iter()
        .enumerate()
        .map(|(index, content)| {
            let role = if index == 0 {
                "system"
            } else if index % 2 == 1 {
                "assistant"
            } else {
                "user"
            };

            json!({
                "role": role,
                "content": content
            })
        })
        .collect()
}