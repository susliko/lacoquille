use std::env;

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

    /// Returns the translation API key to use, preferring Groq over Gemini.
    pub fn translation_api_key(&self) -> Option<String> {
        self.groq_api_key.clone().or_else(|| self.gemini_api_key.clone())
    }

    /// Returns which provider is active for logging.
    pub fn translation_provider(&self) -> &'static str {
        if self.groq_api_key.is_some() {
            "Groq"
        } else if self.gemini_api_key.is_some() {
            "Gemini"
        } else {
            "none"
        }
    }
}