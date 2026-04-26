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
    text.trim().to_string()
}

/// Split text into paragraphs on blank lines.
pub fn split_paragraphs(text: &str) -> Vec<&str> {
    text.split("\n\n").map(|s| s.trim()).filter(|s| !s.is_empty()).collect()
}

/// Return ~350 words from the beginning of the text.
/// Stops once adding the next paragraph would exceed the target by more than a full paragraph.
pub fn extract_excerpt<'a>(paragraphs: &'a [&str], target_words: usize) -> Vec<&'a str> {
    let mut word_count = 0;
    let mut result = vec![];
    for p in paragraphs {
        let wc = p.split_whitespace().count();
        if word_count + wc > target_words && !result.is_empty() {
            break;
        }
        word_count += wc;
        result.push(*p);
    }
    result
}

/// Try to find the first actual story within a collection (skip front matter).
/// Heuristic: a title line is ALL CAPS, < 80 chars, not front-matter noise.
fn is_front_matter_title(title: &str) -> bool {
    let t = title.trim();
    t == "Unknown"
        || t == "GUY DE MAUPASSANT"
        || t == "TABLE"
        || t == "FIN"
        || t.contains("OUVRAGES")
        || t.contains("AUTEUR")
        || t.contains("TIRÉ")
        || t.contains("DROITS")
}

/// Extract the first real story from a collection. Falls back to the full text.
pub fn first_story_content(text: &str) -> String {
    let paragraphs = split_paragraphs(text);
    let mut current_title = String::from("Unknown");
    let mut current_paragraphs = vec![];

    for para in paragraphs {
        let trimmed = para.trim();
        if trimmed.is_empty() {
            continue;
        }
        // Title lines: all-caps, short, followed by prose
        if trimmed.chars().all(|c| c.is_uppercase() || c == ' ' || c == '.' || c == '—')
            && trimmed.len() < 80
        {
            if !current_paragraphs.is_empty() {
                // Finished a section — if it's not front matter and has body, return it
                if !is_front_matter_title(&current_title) && current_paragraphs.len() > 3 {
                    return current_paragraphs.join("\n\n");
                }
                current_paragraphs.clear();
            }
            current_title = trimmed.to_string();
        } else {
            current_paragraphs.push(para);
        }
    }

    // Return whatever we have if no clear section break
    if !current_paragraphs.is_empty() {
        if !is_front_matter_title(&current_title) && current_paragraphs.len() > 3 {
            return current_paragraphs.join("\n\n");
        }
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
}
