use std::env;

use crate::tokenizer::TranslationProvider;

#[derive(Clone)]
pub struct Config {
    pub data_dir: String,
    pub openrouter_api_key: Option<String>,
    pub openrouter_models: Vec<String>,
    pub gemini_api_key: Option<String>,
    pub groq_api_key: Option<String>,
    pub groq_models: Vec<String>,
    pub gemini_models: Vec<String>,
    pub translation_timeout_secs: u64,
    pub translation_retries: u32,
}

impl Config {
    pub fn from_env() -> Self {
        let openrouter_models = env::var("OPENROUTER_MODELS")
            .unwrap_or_else(|_| "z-ai/glm-4.5-air:free".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let groq_models = env::var("GROQ_MODELS")
            .unwrap_or_else(|_| "llama-3.1-8b-instant,llama-3.3-70b-versatile".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let gemini_models = env::var("GEMINI_MODELS")
            .unwrap_or_else(|_| "gemini-2.0-flash,gemini-3.1-flash-lite".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Self {
            data_dir: env::var("LACQ_DATA_DIR").unwrap_or_else(|_| "data".to_string()),
            openrouter_api_key: env::var("OPENROUTER_API_KEY").ok(),
            openrouter_models,
            gemini_api_key: env::var("GEMINI_API_KEY").ok(),
            groq_api_key: env::var("GROQ_API_KEY").ok(),
            groq_models,
            gemini_models,
            translation_timeout_secs: env::var("TRANSLATION_TIMEOUT_SECS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            translation_retries: env::var("TRANSLATION_RETRIES")
                .unwrap_or_else(|_| "2".to_string())
                .parse()
                .unwrap_or(2),
        }
    }

    /// Returns the API key matching the active translation provider.
    pub fn translation_api_key(&self) -> Option<String> {
        // OpenRouter takes precedence
        if self.openrouter_api_key.is_some() {
            self.openrouter_api_key.clone()
        } else if self.groq_api_key.is_some() {
            self.groq_api_key.clone()
        } else {
            self.gemini_api_key.clone()
        }
    }

    /// Returns which provider is active for logging.
    pub fn translation_provider(&self) -> &'static str {
        if self.openrouter_api_key.is_some() {
            "OpenRouter"
        } else if self.groq_api_key.is_some() {
            "Groq"
        } else if self.gemini_api_key.is_some() {
            "Gemini"
        } else {
            "none"
        }
    }

    /// Returns the first available TranslationProvider.
    pub fn first_provider(&self) -> Option<TranslationProvider> {
        // Groq takes precedence (free, fast, reliable)
        if let Some(ref api_key) = self.groq_api_key {
            if !self.groq_models.is_empty() {
                return Some(TranslationProvider::Groq(self.groq_models[0].clone()));
            }
        }
        if let Some(ref api_key) = self.openrouter_api_key {
            if !self.openrouter_models.is_empty() {
                return Some(TranslationProvider::OpenRouter(self.openrouter_models[0].clone()));
            }
        }
        if let Some(ref api_key) = self.gemini_api_key {
            if !self.gemini_models.is_empty() {
                return Some(TranslationProvider::Gemini(self.gemini_models[0].clone()));
            }
        }
        None
    }

    /// Returns the ordered chain of (TranslationProvider, api_key) to try.
    pub fn translation_chain(&self) -> Vec<(TranslationProvider, String)> {
        let mut chain = Vec::new();

        // Groq models first (free, fast, reliable)
        if let Some(ref api_key) = self.groq_api_key {
            for model in &self.groq_models {
                chain.push((TranslationProvider::Groq(model.clone()), api_key.clone()));
            }
        }

        // Then OpenRouter models
        if let Some(ref api_key) = self.openrouter_api_key {
            for model in &self.openrouter_models {
                chain.push((TranslationProvider::OpenRouter(model.clone()), api_key.clone()));
            }
        }

        // Then Gemini models
        if let Some(ref api_key) = self.gemini_api_key {
            for model in &self.gemini_models {
                chain.push((TranslationProvider::Gemini(model.clone()), api_key.clone()));
            }
        }

        chain
    }

    /// Number of retries per provider.
    pub fn retries(&self) -> u32 {
        self.translation_retries
    }

    /// Timeout in seconds for translation requests.
    pub fn timeout_secs(&self) -> u64 {
        self.translation_timeout_secs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[test]
    fn test_translation_chain_groq_first() {
        // With all keys, Groq comes first
        let config = Config {
            data_dir: "data".to_string(),
            openrouter_api_key: Some("test-openrouter".to_string()),
            openrouter_models: vec!["z-ai/glm-4.5-air:free".to_string()],
            groq_api_key: Some("test-groq".to_string()),
            gemini_api_key: Some("test-gemini".to_string()),
            groq_models: vec!["llama-3.1-8b-instant".to_string()],
            gemini_models: vec!["gemini-3.1-flash-lite".to_string()],
            translation_timeout_secs: 30,
            translation_retries: 2,
        };


        let chain = config.translation_chain();
        assert!(!chain.is_empty());
        assert_eq!(chain[0].0.provider_name(), "groq");
    }

    #[test]
    fn test_first_provider_groq() {
        let config = Config {
            data_dir: "data".to_string(),
            openrouter_api_key: Some("test-openrouter".to_string()),
            openrouter_models: vec!["z-ai/glm-4.5-air:free".to_string()],
            groq_api_key: Some("test-groq".to_string()),
            gemini_api_key: Some("test-gemini".to_string()),
            groq_models: vec!["llama-3.1-8b-instant".to_string()],
            gemini_models: vec!["gemini-3.1-flash-lite".to_string()],
            translation_timeout_secs: 30,
            translation_retries: 2,
        };

        let first = config.first_provider().unwrap();
        assert_eq!(first.provider_name(), "groq");
        assert_eq!(first.model_name(), "llama-3.1-8b-instant");
    }

    #[test]
    fn test_multiple_models_in_chain() {
        let config = Config {
            data_dir: "data".to_string(),
            openrouter_api_key: None,
            openrouter_models: vec![],
            groq_api_key: Some("test-groq".to_string()),
            gemini_api_key: None,
            groq_models: vec![
                "llama-3.1-8b-instant".to_string(),
                "llama-3.3-70b-versatile".to_string(),
            ],
            gemini_models: vec![],
            translation_timeout_secs: 30,
            translation_retries: 2,
        };

        let chain = config.translation_chain();
        assert_eq!(chain.len(), 2);
        assert_eq!(chain[0].0.model_name(), "llama-3.1-8b-instant");
        assert_eq!(chain[1].0.model_name(), "llama-3.3-70b-versatile");
    }
}