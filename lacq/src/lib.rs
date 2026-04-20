pub mod config;
pub mod gutendex;
pub mod gutenberg;
pub mod llm;
pub mod vocab;
pub mod translation;

pub mod routes {
    pub mod article;
    pub mod stories;
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Book {
    pub id: u64,
    pub title: String,
    pub authors: Vec<String>,
    pub languages: Vec<String>,
    pub download_count: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Story {
    pub id: String,
    pub title: String,
    pub source: String,
    pub book_id: u64,
    pub published_year: i32,
    pub level: String,
    pub paragraphs: Vec<String>,
    pub excerpt_paragraphs: Vec<usize>,
    pub has_english_translation: bool,
    pub english_book_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StoryListItem {
    pub id: String,
    pub title: String,
    pub source: String,
    pub published_year: i32,
    pub level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleResponse {
    pub story: StoryMeta,
    pub french: FrenchContent,
    pub english: Option<EnglishContent>,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryMeta {
    pub id: String,
    pub title: String,
    pub source: String,
    pub published_year: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrenchContent {
    pub paragraphs: Vec<String>,
    pub vocab_highlights: Vec<VocabHighlight>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabHighlight {
    pub word: String,
    pub paragraph_index: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

impl VocabHighlight {
    pub fn new(word: String, paragraph_index: usize, start_offset: usize, end_offset: usize) -> Self {
        Self { word, paragraph_index, start_offset, end_offset }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnglishContent {
    pub paragraphs: Vec<String>,
    pub source: String,
}
