use std::env;

#[derive(Clone)]
pub struct Config {
    pub data_dir: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            data_dir: env::var("LACQ_DATA_DIR").unwrap_or_else(|_| "data".to_string()),
        }
    }
}
