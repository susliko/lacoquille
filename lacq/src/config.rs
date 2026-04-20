use std::env;

#[derive(Clone)]
pub struct Config {
    pub minimax_api_key: String,
    pub minimax_base_url: String,
    pub gutendex_base_url: String,
    pub gutenberg_base_url: String,
    pub data_dir: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            minimax_api_key: env::var("MINIMAX_API_KEY").unwrap_or_else(|_| "mock-key".to_string()),
            minimax_base_url: env::var("MINIMAX_BASE_URL")
                .unwrap_or_else(|_| "https://api.minimax.chat/v1".to_string()),
            gutendex_base_url: env::var("GUTENDEX_BASE_URL")
                .unwrap_or_else(|_| "https://gutendex.com".to_string()),
            gutenberg_base_url: env::var("GUTENBERG_BASE_URL")
                .unwrap_or_else(|_| "https://www.gutenberg.org".to_string()),
            data_dir: env::var("LACQ_DATA_DIR").unwrap_or_else(|_| "data".to_string()),
        }
    }
}
