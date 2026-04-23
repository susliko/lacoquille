use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn translate(&self, text: &str) -> Result<String, LlmError>;
}

#[derive(Debug)]
pub struct LlmError {
    pub message: String,
}

impl std::fmt::Display for LlmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LlmError {}

pub struct MockMinimaxProvider {
    client: Client,
}

impl MockMinimaxProvider {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl LlmProvider for MockMinimaxProvider {
    async fn translate(&self, text: &str) -> Result<String, LlmError> {
        let preview = if text.len() > 50 { &text[..50] } else { text };
        Ok(format!("[MOCK TRANSLATION of: {}...]", preview))
    }
}

pub struct MinimaxProvider {
    api_key: String,
    base_url: String,
    client: Client,
}

impl MinimaxProvider {
    pub fn new(api_key: String, base_url: String) -> Self {
        Self {
            api_key,
            base_url,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl LlmProvider for MinimaxProvider {
    async fn translate(&self, text: &str) -> Result<String, LlmError> {
        #[derive(serde::Serialize)]
        struct Request {
            model: String,
            messages: Vec<serde_json::Value>,
        }
        #[derive(serde::Deserialize)]
        struct Response {
            choices: Vec<serde_json::Value>,
        }

        let body = Request {
            model: "deepseek-ai/DeepSeek-V2.5".to_string(),
            messages: vec![
                serde_json::json!({"role": "system", "content": "You are a translator. Translate the following French text to English. Return ONLY the translation, no explanations."}),
                serde_json::json!({"role": "user", "content": text}),
            ],
        };

        let resp = self.client
            .post(format!("{}/v1/text/translatechatcompletion", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| LlmError { message: format!("HTTP error: {}", e) })?;

        let resp_json: Response = resp.json().await
            .map_err(|e| LlmError { message: format!("JSON parse error: {}", e) })?;

        resp_json.choices.get(0)
            .and_then(|c| c.get("messages"))
            .and_then(|m| m.as_array())
            .and_then(|arr| arr.last())
            .and_then(|m| m.get("text"))
            .and_then(|t| t.as_str())
            .map(|t| t.to_string())
            .ok_or_else(|| LlmError { message: "No translation in response".to_string() })
    }
}
