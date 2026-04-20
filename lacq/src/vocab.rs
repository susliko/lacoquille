use crate::VocabHighlight;

pub fn extract_vocab_words(paragraphs: &[&str]) -> Vec<VocabHighlight> {
    let stopwords: std::collections::HashSet<&str> = [
        "le", "la", "les", "un", "une", "des", "du", "de", "d", "l",
        "et", "en", "à", "au", "aux", "avec", "pour", "sans", "sur",
        "qui", "que", "quoi", "dont", "où", "que", "ce", "cette", "ces",
        "il", "elle", "ils", "elles", "nous", "vous", "je", "tu", "on",
        "se", "son", "sa", "ses", "leur", "leurs", "mon", "ma", "mes",
        "ton", "ta", "tes", "notre", "votre", "nos",
        "est", "sont", "été", "être", "avoir", "a", "ait", "ont",
        "mais", "ou", "donc", "car", "ni",
        "plus", "moins", "très", "bien", "mal", "ainsi", "comme",
        "par", "pas", "ne", "me", "te", "se", "moi", "toi", "lui",
        "y", "via", "si",
    ].into_iter().collect();

    let mut highlights = vec![];
    let mut seen = std::collections::HashSet::new();

    for (para_idx, para) in paragraphs.iter().enumerate() {
        for word in para.split_whitespace() {
            let clean = word.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase();
            if clean.len() > 2 && !stopwords.contains(&clean.as_str()) {
                if seen.insert((clean.clone(), para_idx)) {
                    if let Some(start) = para.find(word) {
                        highlights.push(VocabHighlight::new(
                            word.to_string(),
                            para_idx,
                            start,
                            start + word.len(),
                        ));
                    }
                }
            }
        }
    }
    highlights
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stopwords_filtered() {
        let paras = &["le chat et la maison"];
        let highlights = extract_vocab_words(paras);
        let words: Vec<_> = highlights.iter().map(|h| h.word.as_str()).collect();
        assert!(!words.contains(&"le"), "'le' should not be highlighted");
        assert!(!words.contains(&"la"), "'la' should not be highlighted");
        assert!(!words.contains(&"et"), "'et' should not be highlighted");
    }

    #[test]
    fn test_non_stopwords_highlighted() {
        let paras = &["Maupassant écrivait en Normandie"];
        let highlights = extract_vocab_words(paras);
        let words: Vec<_> = highlights.iter().map(|h| h.word.as_str()).collect();
        assert!(words.contains(&"Maupassant"), "Maupassant should be highlighted");
        assert!(words.contains(&"Normandie"), "Normandie should be highlighted");
    }

    #[test]
    fn test_duplicate_words_appear_once() {
        let paras = &["Le chat mange la chat", "Le chien mange aussi"];
        let highlights = extract_vocab_words(paras);
        let chat_count = highlights.iter().filter(|h| h.word == "chat").count();
        assert_eq!(chat_count, 1, "duplicate words should appear once");
    }

    #[test]
    fn test_empty_paragraph() {
        let paras: &[&str] = &["", "   ", "Normal text here"];
        let highlights = extract_vocab_words(paras);
        assert!(!highlights.is_empty(), "should find words in non-empty paragraphs");
    }
}
