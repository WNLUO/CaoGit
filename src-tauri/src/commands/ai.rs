//! AI commands
//!
//! Commands for AI-related operations (OpenAI API calls).

use serde::{Deserialize, Serialize};
use super::response::ApiResponse;

/// AI message for chat completions
#[derive(Debug, Serialize, Deserialize)]
pub struct AIMessage {
    pub role: String,
    pub content: String,
}

/// AI response data structure
#[derive(Debug, Serialize, Deserialize)]
pub struct AIResponseData {
    pub choices: Vec<AIChoice>,
}

/// AI choice in response
#[derive(Debug, Serialize, Deserialize)]
pub struct AIChoice {
    pub message: AIMessageResponse,
}

/// AI message response
#[derive(Debug, Serialize, Deserialize)]
pub struct AIMessageResponse {
    pub content: String,
}

/// Call AI API for generating commit messages
#[tauri::command]
pub async fn call_ai_api(
    endpoint: String,
    api_key: String,
    model: String,
    messages: Vec<AIMessage>,
    temperature: f32,
    max_tokens: u32
) -> ApiResponse<String> {
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "model": model,
        "messages": messages,
        "temperature": temperature,
        "max_tokens": max_tokens
    });

    match client
        .post(&endpoint)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<AIResponseData>().await {
                    Ok(data) => {
                        if let Some(choice) = data.choices.first() {
                            ApiResponse::success(choice.message.content.clone())
                        } else {
                            ApiResponse::error("AI returned no choices".to_string())
                        }
                    }
                    Err(e) => ApiResponse::error(format!("Failed to parse AI response: {}", e)),
                }
            } else {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                ApiResponse::error(format!("HTTP {}: {}", status, error_text))
            }
        }
        Err(e) => ApiResponse::error(format!("Failed to call AI API: {}", e)),
    }
}
