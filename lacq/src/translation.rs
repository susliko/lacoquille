use sha2::{Digest, Sha256};
use crate::llm::{LlmProvider, MockMinimaxProvider};

pub struct TranslationCache;

impl TranslationCache {
    fn cache_path(data_dir: &str, book_id: u64, hash: &str) -> String {
        format!("{}/translations/{}/{}.json", data_dir, book_id, hash)
    }

    pub async fn get(data_dir: &str, book_id: u64, text: &str) -> Option<String> {
        let hash = format!("{:x}", Sha256::digest(text.as_bytes()));
        let path = Self::cache_path(data_dir, book_id, &hash);
        tokio::fs::read_to_string(path).await.ok()
    }

    pub async fn set(data_dir: &str, book_id: u64, text: &str, content: &str) -> std::io::Result<()> {
        let hash = format!("{:x}", Sha256::digest(text.as_bytes()));
        let path = Self::cache_path(data_dir, book_id, &hash);
        if let Some(parent) = std::path::Path::new(&path).parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        tokio::fs::write(&path, content).await
    }
}

pub async fn get_translation(
    paragraphs: &[String],
    book_id: u64,
    _has_english: bool,
    data_dir: &str,
) -> Result<Option<crate::EnglishContent>, Box<dyn std::error::Error + Send + Sync>> {
    let text = paragraphs.join("\n\n");

    if let Some(cached) = TranslationCache::get(data_dir, book_id, &text).await {
        if let Ok(parsed) = serde_json::from_str::<crate::EnglishContent>(&cached) {
            return Ok(Some(parsed));
        }
    }

    let llm = MockMinimaxProvider;
    let translated: String = llm.translate(&text).await?;
    let result = crate::EnglishContent {
        paragraphs: translated.split("\n\n").map(|s| s.to_string()).collect(),
        source: "llm".to_string(),
    };

    let json = serde_json::to_string(&result)?;
    TranslationCache::set(data_dir, book_id, &text, &json).await?;

    Ok(Some(result))
}
