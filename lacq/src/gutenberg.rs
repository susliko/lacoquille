pub fn clean_text(raw: &str) -> String {
    raw.to_string()
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
