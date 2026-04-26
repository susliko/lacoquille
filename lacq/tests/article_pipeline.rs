use lacq::{AppState, BookMeta, Config};

#[test]
fn test_curated_books_has_real_gutenberg_ids() {
    let books = BookMeta::curated();
    assert!(!books.is_empty(), "curated list should not be empty");
    for book in &books {
        assert!(book.gutenberg_id > 0, "gutenberg_id should be positive for {}", book.title);
    }
}

#[test]
fn test_book_rotation_is_deterministic() {
    let book1 = BookMeta::for_today();
    let book2 = BookMeta::for_today();
    assert_eq!(book1.gutenberg_id, book2.gutenberg_id, "same day should return same book");
}

#[test]
fn test_book_meta_fields() {
    let books = BookMeta::curated();
    for book in &books {
        assert!(!book.title.is_empty(), "title should not be empty");
        assert!(!book.collection.is_empty(), "collection should not be empty");
    }
}

#[tokio::test]
async fn test_app_state_creates_successfully() {
    let http = reqwest::Client::new();
    let state = AppState::new(http, "data".to_string(), Config::from_env());
    assert_eq!(state.curated_books.len(), 5);
}

#[tokio::test]
async fn test_app_state_text_cache_starts_empty() {
    let http = reqwest::Client::new();
    let state = AppState::new(http, "data".to_string(), Config::from_env());
    let cache = state.text_cache.lock().await;
    assert!(cache.is_empty(), "cache should start empty");
}

use lacq::gutenberg;

#[test]
fn test_clean_text_strips_gutenberg_header_footer() {
    let input = "Some junk\n*** START OF THIS PROJECT GUTENBERG EBOOK ***\n\nReal content here.\n\n*** END OF THIS PROJECT GUTENBERG EBOOK ***\nMore junk";
    let result = gutenberg::clean_text(input);
    assert!(result.starts_with("Real content here."));
    assert!(!result.contains("GUTENBERG"));
}

#[test]
fn test_clean_text_normalizes_line_endings() {
    let input = "Line one.\r\n\r\nLine two.\r\nLine three.";
    let result = gutenberg::clean_text(input);
    assert!(result.contains("Line one.\n\nLine two."));
}

#[test]
fn test_clean_text_collapses_multiple_blank_lines() {
    let input = "Para one.\n\n\n\nPara two.";
    let result = gutenberg::clean_text(input);
    assert!(!result.contains("\n\n\n"));
}

#[test]
fn test_split_paragraphs_splits_on_double_newline() {
    let input = "Para one.\n\nPara two.\n\n\nPara three.";
    let result = gutenberg::split_paragraphs(input);
    assert_eq!(result.len(), 3);
}

#[test]
fn test_extract_excerpt_stops_before_exceeding_target() {
    let paras = &["One two three four five.", "Six seven eight nine ten.", "Eleven twelve thirteen fourteen."];
    let result = gutenberg::extract_excerpt(paras, 7);
    // First para: 5 words (≤ 7). Second para would add 5 = 10 (> 7), so stop after first.
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], "One two three four five.");
}
