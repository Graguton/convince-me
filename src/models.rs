use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

pub struct AppState {
    pub client: Client,
    pub api_key: String,
    pub server_addr: String,
    pub messages: Mutex<HashMap<String, Vec<String>>>,
}

#[derive(Deserialize)]
pub struct ChatRequest {
    pub user_id: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct ChatResponse {
    pub response: String,
}

#[derive(Serialize)]
pub struct DeleteResponse {
    pub message: String,
}