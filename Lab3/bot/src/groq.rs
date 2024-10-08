use crate::config::config;
use crate::error::AppResult;
use anyhow::anyhow;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use crate::State;

#[derive(Debug, Deserialize)]
struct GrokLLMResponse {
    choices: Vec<Choice>,
    created: u64,
    id: String,
    model: String,
    object: String,
    system_fingerprint: String,
    usage: Usage,
    x_groq: XGroq,
}

#[derive(Debug, Deserialize)]
struct Choice {
    finish_reason: String,
    index: u32,
    logprobs: Option<serde_json::Value>,
    message: Message,
}

#[derive(Debug, Deserialize)]
struct Message {
    content: String,
    role: String,
}

#[derive(Debug, Deserialize)]
struct Usage {
    completion_time: f64,
    completion_tokens: u32,
    prompt_time: f64,
    prompt_tokens: u32,
    queue_time: f64,
    total_time: f64,
    total_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct XGroq {
    id: String,
}

pub async fn send_prompt_to_groq(prompt: &str, state: State) -> AppResult<String> {
    let url = "https://api.groq.com/openai/v1/chat/completions";

    let body = json!({
        "messages": [{
            "role": "user",
            "content": prompt
        }],
        "model": "llama3-8b-8192"
    });

    let res: GrokLLMResponse = state.http_client
        .post(url)
        .header("Authorization", format!("Bearer {}", config().GROQ.API_KEY))
        .json(&body)
        .send()
        .await?
        .json()
        .await?;

    let reply = res
        .choices
        .first()
        .ok_or(anyhow!("There isn't any choice in LLM response"))?
        .message
        .content
        .clone();
    
    Ok(reply)
}
