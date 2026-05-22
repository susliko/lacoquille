use std::env;

use crate::tokenizer::TranslationProvider;

#[derive(Clone)]
pub struct Config {
    pub data_dir: String,
    pub gemini_api_key: Option<String>,
    pub groq_api_key: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            data_dir: env::var("LACQ_DATA_DIR").unwrap_or_else(|_| "data".to_string()),
            gemini_api_key: env::var("GEMINI_API_KEY").ok(),
            groq_api_key: env::var("GROQ_API_KEY").ok(),
        }
    }

    /// Returns the API key matching the active translation provider.
    pub fn translation_api_key(&self) -> Option<String> {
        match self.translation_provider_enum() {
            TranslationProvider::Gemini => self.gemini_api_key.clone(),
            TranslationProvider::Groq => self.groq_api_key.clone(),
        }
    }

    /// Returns which provider is active for logging.
    pub fn translation_provider(&self) -> &'static str {
        if self.gemini_api_key.is_some() {
            "Gemini"
        } else if self.groq_api_key.is_some() {
            "Groq"
        } else {
            "none"
        }
    }

    /// Returns the TranslationProvider enum to use for tokenization.
    pub fn translation_provider_enum(&self) -> TranslationProvider {
        if self.gemini_api_key.is_some() {
            TranslationProvider::Gemini
        } else if self.groq_api_key.is_some() {
            TranslationProvider::Groq
        } else {
            TranslationProvider::Gemini // Default fallback (will fail at runtime without key)
        }
    }
}