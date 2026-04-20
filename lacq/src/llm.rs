use async_trait::async_trait;

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

pub struct MockMinimaxProvider;

#[async_trait]
impl LlmProvider for MockMinimaxProvider {
    async fn translate(&self, text: &str) -> Result<String, LlmError> {
        let preview = if text.len() > 50 { &text[..50] } else { text };
        Ok(format!("[MOCK TRANSLATION of: {}...]", preview))
    }
}
