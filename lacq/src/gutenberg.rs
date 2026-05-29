/// Strip Gutenberg header/footer and normalize whitespace.
pub fn clean_text(raw: &str) -> String {
    const START_MARKER: &str = "*** START";
    const END_MARKER: &str = "*** END";

    let start = match raw.find(START_MARKER) {
        Some(idx) => {
            let after_marker = idx + START_MARKER.len();
            match raw[after_marker..].find('\n') {
                Some(j) => after_marker + j + 1,
                None => after_marker,
            }
        }
        None => 0,
    };
    let end = raw.find(END_MARKER).unwrap_or(raw.len());
    let mut text = raw[start..end].to_string();
    text = text.replace("\r\n", "\n").replace("\r", "\n");
    text = text.replace("\n---", "\n\n").replace("---\n", "\n\n");
    while text.contains("\n\n\n") {
        text = text.replace("\n\n\n", "\n\n");
    }
    // Preserve paragraph breaks before collapsing single newlines.
    // Handles: "\n\n" (blank line), "\n \n" (blank line with trailing space), "\n\n" (from \r\n replacement).
    text = text.replace("\n\n", "\x00PARA\x00");
    text = text.replace("\n \n", "\x00PARA\x00");
    text = text.split('\n').collect::<Vec<_>>().join(" ");
    text = text.replace("\x00PARA\x00", "\n\n");
    text.trim().to_string()
}

/// Split text into paragraphs on blank lines.
pub fn split_paragraphs(text: &str) -> Vec<&str> {
    text.split("\n\n").map(|s| s.trim()).filter(|s| !s.is_empty()).collect()
}

/// Return ~target_words from the beginning of the text.
/// Stops once adding the next paragraph would exceed the target.
pub fn extract_excerpt(paragraphs: &[&str], target_words: usize) -> Vec<String> {
    let mut word_count = 0;
    let mut result = vec![];
    
    for p in paragraphs {
        let trimmed = p.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        let wc = trimmed.split_whitespace().count();
        
        // If adding this paragraph would exceed target, stop
        if word_count + wc > target_words {
            // Only skip if we already have some content
            if !result.is_empty() {
                break;
            }
            // If we have nothing yet, truncate this paragraph to fit
            if wc > target_words {
                let truncated = truncate_to_words(p, target_words);
                if !truncated.is_empty() {
                    result.push(truncated);
                }
            }
            break;
        }
        
        word_count += wc;
        result.push(p.to_string());
    }
    
    // Guard: if every paragraph was tiny, return at least the first one.
    if result.is_empty() && !paragraphs.is_empty() {
        return vec![paragraphs[0].to_string()];
    }
    result
}

/// Truncate text to at most target_words.
fn truncate_to_words(text: &str, target: usize) -> String {
    let mut words = 0;
    let mut end_idx = text.len();
    
    for (i, c) in text.char_indices() {
        if c.is_whitespace() {
            words += 1;
            if words >= target {
                end_idx = i;
                break;
            }
        }
    }
    
    text[..end_idx].trim().to_string()
}

/// Extract the first real story from a collection. Falls back to the full text.
pub fn first_story_content(text: &str) -> String {
    let paragraphs = split_paragraphs(text);
    let mut current_body: Vec<&str> = vec![];

    fn has_dialogue(paras: &[&str]) -> bool {
        for p in paras {
            if p.contains("--") && p.len() < 500 {
                return true;
            }
        }
        false
    }

    /// Returns true if this paragraph is a chapter marker (roman numeral I-X).
    fn is_chapter_marker(s: &str) -> bool {
        let t = s.trim();
        let chars: Vec<char> = t.chars().filter(|c| !c.is_whitespace()).collect();
        chars.len() >= 1 && chars.len() <= 4
            && chars.iter().all(|c| matches!(c, 'I' | 'V' | 'X' | 'L' | 'C' | 'D' | 'M'))
    }

    /// Returns true if this paragraph looks like an author/collection sign-off.
    /// After this, the real book content begins.
    fn is_author_signoff(s: &str) -> bool {
        let t = s.trim();
        let words = t.split_whitespace().count();
        (words <= 5 && t.ends_with('.'))
            && (t.contains("MAUPASSANT")
                || t.contains("de ") && t.contains("1870")
                || t.contains("Zola")
                || t.contains("Paris") && words <= 4
                || words <= 3 && t.chars().all(|c| c.is_uppercase() || c.is_whitespace() || c == '.'))
    }

    for para in paragraphs {
        let trimmed = para.trim();
        if trimmed.is_empty() {
            continue;
        }

        // After an author sign-off, discard any accumulated front matter
        if is_author_signoff(trimmed) {
            current_body.clear();
            continue;
        }

        // All-caps, short lines that aren't chapter markers = potential titles
        if trimmed.chars().all(|c| c.is_uppercase() || c == ' ' || c == '.' || c == '—')
            && trimmed.len() < 80
        {
            // Chapter markers end the current chapter — return it if it has content
            if is_chapter_marker(trimmed) {
                if !current_body.is_empty()
                    && (has_dialogue(&current_body) || current_body.len() >= 3)
                {
                    return current_body.join("\n\n");
                }
                current_body.clear();
                continue;
            }
            // Regular all-caps title: keep accumulating after it
            // (don't add to current_body so it doesn't appear in output)
        } else {
            current_body.push(para);
        }
    }

    // Return body if it has enough content
    if !current_body.is_empty()
        && (has_dialogue(&current_body) || current_body.len() >= 3)
    {
        return current_body.join("\n\n");
    }

    text.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_text_strips_gutenberg_markers() {
        let input = "junk\n*** START OF THIS PROJECT GUTENBERG EBOOK ***\n\nReal content.\n\n*** END OF THIS PROJECT GUTENBERG EBOOK ***\nmore junk";
        let result = clean_text(input);
        assert!(result.starts_with("Real content."));
        assert!(!result.contains("GUTENBERG"));
    }

    #[test]
    fn test_clean_text_preserves_plain_text() {
        let input = "Plain text without markers.";
        assert_eq!(clean_text(input), input);
    }

    #[test]
    fn test_clean_text_normalizes_newlines() {
        let input = "Para one.\r\n\r\nPara two.\r\nPara three.";
        let result = clean_text(input);
        assert!(result.contains("Para one.\n\nPara two."));
    }

    #[test]
    fn test_split_paragraphs_on_double_newline() {
        let input = "A.\n\nB.\n\n\nC.";
        let result = split_paragraphs(input);
        assert_eq!(result, &["A.", "B.", "C."]);
    }

    #[test]
    fn test_extract_excerpt_stops_at_boundary() {
        let paras = &["One two three.", "Four five six seven eight nine ten."];
        let result = extract_excerpt(paras, 7);
        assert_eq!(result, &["One two three."]);
    }

    #[test]
    fn test_extract_excerpt_includes_second_para_when_first_is_small() {
        let paras = &["One.", "Two three four five six seven eight nine ten."];
        let result = extract_excerpt(paras, 7);
        // First para (1 word) + second para (9 words) = 10. Exceeds 7+10=17? No, 10 > 7, so stop after first.
        // Actually with wc=1 and target=7: 1 <= 7 so include, then 1+9=10 > 7 so stop. So result = ["One."]
        assert_eq!(result, &["One."]);
    }

    #[test]
    fn test_extract_excerpt_empty_input() {
        let paras: Vec<&str> = vec![];
        let result = extract_excerpt(&paras, 350);
        assert!(result.is_empty(), "empty input must yield empty output");
    }

    #[test]
    fn test_extract_excerpt_single_long_paragraph() {
        let words: String = (0..400).map(|i| format!("word{}", i)).collect::<Vec<_>>().join(" ");
        let paras = &[words.as_str()];
        let result = extract_excerpt(paras, 350);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_extract_excerpt_zero_word_paragraph() {
        let paras = &["   ", "Real words here.", "More real words here too."];
        let result = extract_excerpt(paras, 5);
        assert!(result.len() >= 1);
    }

    #[test]
    fn test_first_story_short_with_dialogue_returned() {
        // Story with dialogue should be returned even if short
        let text = "STORY TITLE\n\n--Hello, how are you?\n\n--Fine, thanks.";
        let result = first_story_content(text);
        // Should return the body (with dialogue), not the full text
        assert!(result.contains("--Hello"));
        assert!(!result.contains("STORY TITLE"));
    }

    #[test]
    fn test_first_story_long_story_returned() {
        // Long stories should be returned even without dialogue
        let text = "LONG TITLE\n\nParagraph 0 here.\n\nParagraph 1 here.\n\nParagraph 2 here.\n\nParagraph 3 here.\n\nParagraph 4 here.\n\nParagraph 5 here.\n\nParagraph 6 here.\n\nParagraph 7 here.\n\nParagraph 8 here.\n\nParagraph 9 here.";
        let result = first_story_content(text);
        assert!(!result.contains("LONG TITLE"));
        assert!(result.contains("Paragraph 9 here."));
    }

    #[test]
    fn test_split_paragraphs_empty() {
        assert!(split_paragraphs("").is_empty());
        assert!(split_paragraphs("   ").is_empty());
        assert!(split_paragraphs("\n\n\n").is_empty());
    }


    #[test]
    fn test_split_paragraphs_single_paragraph_no_blank_line() {
        let result = split_paragraphs("Single paragraph with no double newline.");
        assert_eq!(result.len(), 1);
    }
}
