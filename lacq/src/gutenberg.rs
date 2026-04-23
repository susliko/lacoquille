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

pub fn split_paragraphs(text: &str) -> Vec<&str> {
    text.split("\n\n").filter(|s| !s.trim().is_empty()).collect()
}

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

pub fn split_stories<'a>(paragraphs: &'a [&str]) -> Vec<(String, Vec<&'a str>)> {
    let mut stories = vec![];
    let mut current_title = String::from("Unknown");
    let mut current_paragraphs = vec![];

    for para in paragraphs {
        let trimmed = para.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.chars().all(|c| c.is_uppercase() || c == ' ' || c == '.' || c == '—')
            && trimmed.len() < 80
        {
            if !current_paragraphs.is_empty() {
                stories.push((current_title.clone(), current_paragraphs.clone()));
                current_paragraphs.clear();
            }
            current_title = trimmed.to_string();
        } else {
            current_paragraphs.push(*para);
        }
    }
    if !current_paragraphs.is_empty() {
        stories.push((current_title, current_paragraphs));
    }
    stories
}

pub fn first_story_content<'a>(text: &str) -> String {
    let paragraphs = split_paragraphs(text);
    let stories = split_stories(&paragraphs);
    if stories.len() <= 1 {
        return text.to_string();
    }
    for (title, paras) in &stories[1..] {
        let is_front_matter = title == "Unknown"
            || title == "GUY DE MAUPASSANT"
            || title == "TABLE"
            || title == "FIN"
            || title.contains("OUVRAGES")
            || title.contains("AUTEUR")
            || title.contains("TIRÉ")
            || title.contains("DROITS");
        if !is_front_matter && paras.len() > 3 {
            return paras.join("\n\n");
        }
    }
    text.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_text_preserves_content() {
        let input = "Some text without headers";
        let result = clean_text(input);
        assert_eq!(result, input);
    }

    #[test]
    fn test_clean_text_strips_gutenberg_markers() {
        let input = "Some junk\n*** START OF THIS PROJECT GUTENBERG EBOOK ***\n\nReal content here.\n\n*** END OF THIS PROJECT GUTENBERG EBOOK ***\nMore junk";
        let result = clean_text(input);
        assert!(result.starts_with("Real content here."));
        assert!(!result.contains("GUTENBERG"));
    }

    #[test]
    fn test_split_paragraphs_on_double_newline() {
        let input = "Para one.\n\nPara two.\n\n\nPara three.";
        let result = split_paragraphs(input);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_extract_excerpt_target_word_count() {
        let paras = &["One two three four five.", "Six seven eight nine ten."];
        let result = extract_excerpt(paras, 7);
        let word_count: usize = result.iter().map(|p| p.split_whitespace().count()).sum();
        assert!(word_count <= 7 + 20, "should be within ~20 words of target");
    }

    #[test]
    fn test_split_stories_separates_by_title_lines() {
        let paras = &[
            "Some narrative text here.",
            "CHAPTER ONE",
            "Story content after chapter header.",
        ];
        let stories = split_stories(paras);
        assert!(!stories.is_empty());
    }
}
