use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Span(pub [usize; 2]);

impl PartialEq for Span {
    fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
}

impl<'de> Deserialize<'de> for Span {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let arr: Vec<serde_json::Value> = Deserialize::deserialize(deserializer)?;
        if arr.len() != 2 {
            return Err(serde::de::Error::custom("span must have exactly 2 elements"));
        }
        let a = match &arr[0] {
            serde_json::Value::Number(n) => n.as_u64()
                .ok_or_else(|| serde::de::Error::custom("span start must be non-negative integer"))?
                as usize,
            serde_json::Value::String(s) => s.parse::<u64>().map_err(serde::de::Error::custom)?
                as usize,
            _ => return Err(serde::de::Error::custom("span value must be number or numeric string")),
        };
        let b = match &arr[1] {
            serde_json::Value::Number(n) => n.as_u64()
                .ok_or_else(|| serde::de::Error::custom("span end must be non-negative integer"))?
                as usize,
            serde_json::Value::String(s) => s.parse::<u64>().map_err(serde::de::Error::custom)?
                as usize,
            _ => return Err(serde::de::Error::custom("span value must be number or numeric string")),
        };
        Ok(Span([a, b]))
    }
}

impl From<Span> for [usize; 2] {
    fn from(s: Span) -> Self { s.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrenchToken {
    #[serde(rename = "text")]
    pub text: String,           // FRENCH base text, NO punctuation
    #[serde(rename = "trailingPunct", default)]
    pub trailing_punct: String, // Punctuation AFTER token (e.g., "," or "" or ".")
    #[serde(rename = "leadingPunct", default)]
    pub leading_punct: String,  // Punctuation BEFORE token (rare, e.g., "(")
    #[serde(rename = "translation")]
    pub translation: String,    // ENGLISH translation + trailing punctuation
    #[serde(rename = "spans")]
    pub spans: Vec<Span>,
    #[serde(rename = "enIndices", default)]
    pub en_indices: Vec<usize>,
}

impl FrenchToken {
    /// Returns the full text including punctuation for span matching
    pub fn full_text(&self) -> String {
        format!("{}{}{}", self.leading_punct, self.text, self.trailing_punct)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnglishToken {
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "index")]
    pub index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenizedText {
    #[serde(rename = "frTokens")]
    pub fr_tokens: Vec<FrenchToken>,
    #[serde(rename = "enTokens")]
    pub en_tokens: Vec<EnglishToken>,
    #[serde(default)]
    pub book_id: u64,
    #[serde(rename = "providerUsed", default)]
    pub provider_used: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GroqResponse {
    id: String,
    choices: Vec<GroqChoice>,
    #[serde(default)]
    error: Option<GroqError>,
}

#[derive(Debug, Deserialize)]
struct GroqChoice {
    message: GroqMessage,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GroqMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
struct GroqError {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
    #[serde(default)]
    code: Option<String>,
}

/// TranslationProvider with model name
#[derive(Debug, Clone, PartialEq)]
pub enum TranslationProvider {
    OpenRouter(String), // OpenRouter(model_name)
    Groq(String),       // Groq(model_name)
    Gemini(String),     // Gemini(model_name)
}

impl TranslationProvider {
    pub fn provider_name(&self) -> &'static str {
        match self {
            TranslationProvider::OpenRouter(_) => "openrouter",
            TranslationProvider::Groq(_) => "groq",
            TranslationProvider::Gemini(_) => "gemini",
        }
    }

    pub fn model_name(&self) -> &str {
        match self {
            TranslationProvider::OpenRouter(m) => m,
            TranslationProvider::Groq(m) => m,
            TranslationProvider::Gemini(m) => m,
        }
    }

    pub fn display(&self) -> String {
        format!("{}/{}", self.provider_name(), self.model_name())
    }
}

/// Split text into tokens with separated punctuation.
/// Preserves contractions like "d'une", "l'homme" as single tokens.
/// Returns: Vec<(content, trailing_punct, leading_punct)>
pub fn split_tokens_with_punct(text: &str) -> Vec<(String, String, String)> {
    let mut results = Vec::new();

    for word in text.split_whitespace() {
        let word_chars: Vec<char> = word.chars().collect();
        let mut leading = String::new();
        let mut trailing = String::new();
        let mut content_start = 0;
        let mut content_end = word_chars.len();

        // Handle standalone punctuation (single punctuation characters like "—" or ":")
        // These should be treated as content, not extracted as trailing/leading
        if word_chars.len() == 1 && is_closing_punct(word_chars[0]) {
            // Single punctuation - add as-is as content
            results.push((word.to_string(), String::new(), String::new()));
            continue;
        }

        // Extract leading punctuation (opening marks)
        for (i, c) in word_chars.iter().enumerate() {
            if is_opening_punct(*c) {
                leading.push(*c);
                content_start = i + 1;
            } else {
                break;
            }
        }

        // Extract trailing punctuation (closing marks) - scan backwards
        for (i, c) in word_chars.iter().rev().enumerate() {
            // Apostrophe at the end of a word is part of contraction, not trailing punct
            // e.g., "d'une" → d'une (leading='), not d (trailing=')
            if *c == '\'' && content_end == word_chars.len() - i {
                // This is an apostrophe that's part of content, not trailing
                break;
            }
            if is_closing_punct(*c) {
                trailing.push(*c);
                content_end -= 1;
            } else {
                break;
            }
        }

        let content: String = word_chars[content_start..content_end].iter().collect();
        if !content.is_empty() || !leading.is_empty() || !trailing.is_empty() {
            results.push((content, trailing, leading));
        }
    }

    results
}

fn is_opening_punct(c: char) -> bool {
    matches!(c, '«' | '"' | '(' | '[')
}

fn is_closing_punct(c: char) -> bool {
    matches!(c, '»' | '"' | ')' | ']' | '.' | ',' | '!' | '?' | ';' | ':' | '—' | '-')
}

fn build_prompt(french_text: &str) -> String {
    format!(
        r#"Tokenize FR→EN for bilingual reading. Translate naturally, including multi-word phrases.

Output format:
{{"frTokens":[{{"text":"phrase","trailingPunct":".","leadingPunct":"","translation":"phrase.","spans":[[0,6]],"enIndices":[0]}}],"enTokens":[{{"text":"phrase.","index":0}}]}}

IMPORTANT RULES:
1. Natural phrase boundaries: translate multi-word units together when natural ("Le petit" → "The little")
2. Contractions stay together: "d'une", "au", "du", "l'homme"
3. Spans are CHARACTER offsets in the original French text
4. Translation MUST include trailing punctuation: "Le chat," → "The cat," (keep comma!)
5. Translation should read naturally in English

EXAMPLES:
Input: "La fortune."
Tokens:
{{"text":"La fortune","trailingPunct":".","leadingPunct":"","translation":"The fortune.","spans":[[0,11]],"enIndices":[0]}}

Input: "Le petit Georges, à quatre pattes."
Tokens:
{{"text":"Le","trailingPunct":"","leadingPunct":"","translation":"The","spans":[[0,2]],"enIndices":[0]}}
{{"text":"petit","trailingPunct":"","leadingPunct":"","translation":"little","spans":[[3,8]],"enIndices":[1]}}
{{"text":"Georges","trailingPunct":",","leadingPunct":"","translation":"Georges,","spans":[[9,16]],"enIndices":[2]}}
{{"text":"à","trailingPunct":"","leadingPunct":"","translation":"at","spans":[[18,19]],"enIndices":[3]}}
{{"text":"quatre","trailingPunct":"","leadingPunct":"","translation":"four","spans":[[20,26]],"enIndices":[4]}}
{{"text":"pattes","trailingPunct":".","leadingPunct":"","translation":"paws.","spans":[[27,33]],"enIndices":[5]}}

French text:
{}"#,
        french_text
    )
}

/// Helper: convert character position to byte position in UTF-8 text.
fn char_pos_to_byte_pos(text: &str, char_pos: usize) -> usize {
    text.char_indices()
        .nth(char_pos)
        .map(|(b, _)| b)
        .unwrap_or(text.len())
}

/// Helper: convert byte position to character position in UTF-8 text.
fn byte_pos_to_char_pos(text: &str, byte_pos: usize) -> usize {
    text[..byte_pos.min(text.len())].chars().count()
}

/// Verify that each token's text matches the span positions in the original text.
/// If a span is wrong, try to find the correct position. All spans are CHARACTER positions.
fn repair_spans(text: &str, tokens: &mut Vec<FrenchToken>) {
    let mut valid_tokens = Vec::new();
    for mut token in tokens.drain(..) {
        let text_chars = text.chars().count();

        // Try to find the text in the original (char-based)
        let original_char_pos = token.spans.first().map(|s| s.0[0]).unwrap_or(0);
        let mut best_pos: Option<usize> = None;
        let mut best_dist = usize::MAX;

        let mut search_char_pos = 0usize;
        loop {
            if let Some(pos) = text[char_pos_to_byte_pos(text, search_char_pos)..].find(&token.text) {
                let abs_byte = char_pos_to_byte_pos(text, search_char_pos) + pos;
                let abs_char = byte_pos_to_char_pos(text, abs_byte);
                let dist = if abs_char >= original_char_pos {
                    abs_char - original_char_pos
                } else {
                    original_char_pos - abs_char
                };
                if dist < best_dist {
                    best_dist = dist;
                    best_pos = Some(abs_char);
                }
                // Continue searching for closer matches
                let next_byte = abs_byte + token.text.len().max(1);
                search_char_pos = byte_pos_to_char_pos(text, next_byte.min(text.len()));
                if search_char_pos >= text_chars { break; }
            } else {
                break;
            }
        }

        if let Some(char_pos) = best_pos {
            let end_char = char_pos + token.text.chars().count();
            if end_char <= text_chars {
                token.spans = vec![Span([char_pos, end_char])];
                valid_tokens.push(token);
            }
        }
        // Couldn't repair — drop the token
    }
    *tokens = valid_tokens;
}

/// Dispatch to the appropriate tokenization function based on provider.
pub async fn tokenize(
    client: &reqwest::Client,
    api_key: &str,
    french_text: &str,
    provider: &TranslationProvider,
) -> Result<TokenizedText, String> {
    match provider {
        TranslationProvider::OpenRouter(model) => tokenize_openrouter(client, api_key, french_text, model).await,
        TranslationProvider::Groq(model) => tokenize_groq(client, api_key, french_text, model).await,
        TranslationProvider::Gemini(model) => tokenize_gemini(client, api_key, french_text, model).await,
    }
}

pub async fn tokenize_openrouter(
    client: &reqwest::Client,
    api_key: &str,
    french_text: &str,
    model: &str,
) -> Result<TokenizedText, String> {
    let url = "https://openrouter.ai/api/v1/chat/completions";
    let prompt = build_prompt(french_text);

    let body = serde_json::json!({
        "model": model,
        "messages": [{ "role": "user", "content": prompt }],
        "max_tokens": 8192,
        "temperature": 0.2
    });

    let resp = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .header("HTTP-Referer", "https://lacoquille.com")
        .header("X-Title", "Lacoquille")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("OpenRouter request failed: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body_text = resp.text().await.unwrap_or_default();
        return Err(format!("OpenRouter API error ({}): {}", status, body_text));
    }

    #[derive(Deserialize)]
    struct OpenRouterResponse {
        id: String,
        choices: Vec<OpenRouterChoice>,
        error: Option<OpenRouterError>,
    }
    #[derive(Deserialize)]
    struct OpenRouterChoice {
        message: OpenRouterMessage,
    }
    #[derive(Deserialize)]
    struct OpenRouterMessage {
        content: String,
    }
    #[derive(Deserialize)]
    struct OpenRouterError {
        message: String,
    }

    let openrouter_resp: OpenRouterResponse = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse OpenRouter response: {}", e))?;

    let choice = openrouter_resp
        .choices
        .first()
        .ok_or_else(|| "No choices in OpenRouter response".to_string())?;

    // Strip markdown code fences if present
    let content = choice.message.content.trim().to_string();
    let json_str = content
        .trim_start_matches("```json\n")
        .trim_start_matches("```\n")
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```");


    let mut parsed: TokenizedText = serde_json::from_str(json_str)
        .map_err(|e| format!("Failed to parse JSON from OpenRouter: {}. Raw: {}", e, &content[..content.len().min(300)]))?;


    repair_spans(french_text, &mut parsed.fr_tokens);
    parsed.provider_used = Some(format!("openrouter/{}", model));

    Ok(parsed)
}

async fn tokenize_groq(
    client: &reqwest::Client,
    api_key: &str,
    french_text: &str,
    model: &str,
) -> Result<TokenizedText, String> {
    let url = "https://api.groq.com/openai/v1/chat/completions";
    let prompt = build_prompt(french_text);

    let body = serde_json::json!({
        "model": model,
        "messages": [{ "role": "user", "content": prompt }],
        "response_format": { "type": "json_object" },
        "max_tokens": 8192,
        "temperature": 0.2
    });

    let resp = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Groq request failed: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body_text = resp.text().await.unwrap_or_default();
        if let Ok(err_resp) = serde_json::from_str::<GroqResponse>(&body_text) {
            if let Some(err) = err_resp.error {
                return Err(format!(
                    "Groq API error ({}): {} — {}",
                    status, err.error_type, err.message
                ));
            }
        }
        return Err(format!("Groq API error ({}): {}", status, body_text));
    }

    let groq_resp: GroqResponse = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse Groq response: {}", e))?;

    if let Some(err) = groq_resp.error {
        return Err(format!("Groq API error: {} — {}", err.error_type, err.message));
    }

    let choice = groq_resp
        .choices
        .first()
        .ok_or_else(|| "No choices in Groq response".to_string())?;

    // Strip think blocks (e.g. <think>...) that qwen may emit
    let think_re: once_cell::sync::Lazy<Regex> =
        once_cell::sync::Lazy::new(|| Regex::new(r"<think>[\s\S]*?").unwrap());

    let content = think_re.replace_all(&choice.message.content, "").trim().to_string();

    // Strip markdown code fences if present (defensive)
    let json_str = content
        .trim_start_matches("```json\n")
        .trim_start_matches("```\n")
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```");

    let mut parsed: TokenizedText = serde_json::from_str(json_str)
        .map_err(|e| format!(
            "Failed to parse JSON from Groq: {}. Raw: {}",
            e,
            &json_str[..json_str.len().min(300)]
        ))?;

    repair_spans(french_text, &mut parsed.fr_tokens);
    parsed.provider_used = Some(format!("groq/{}", model));

    Ok(parsed)
}

async fn tokenize_gemini(
    client: &reqwest::Client,
    api_key: &str,
    french_text: &str,
    model: &str,
) -> Result<TokenizedText, String> {
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model,
        api_key
    );

    let prompt = build_prompt(french_text);
    let body = serde_json::json!({
        "contents": [{
            "parts": [{ "text": prompt }]
        }],
        "generationConfig": {
            "responseMimeType": "application/json",
            "temperature": 0.2,
            "maxOutputTokens": 8192,
        }
    });

    let resp = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Gemini request failed: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body_text = resp.text().await.unwrap_or_default();
        return Err(format!(
            "Gemini API error ({}): {}",
            status,
            &body_text[..body_text.len().min(200)]
        ));
    }

    #[derive(Deserialize)]
    struct GeminiResponse {
        candidates: Vec<GeminiCandidate>,
    }
    #[derive(Deserialize)]
    struct GeminiCandidate {
        content: GeminiContent,
    }
    #[derive(Deserialize)]
    struct GeminiContent {
        parts: Vec<GeminiPart>,
    }
    #[derive(Deserialize)]
    struct GeminiPart {
        text: Option<String>,
    }

    let gemini_resp: GeminiResponse = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse Gemini response: {}", e))?;

    let text = gemini_resp
        .candidates
        .first()
        .and_then(|c| c.content.parts.first())
        .and_then(|p| p.text.clone())
        .ok_or_else(|| "No text in Gemini response".to_string())?;

    let mut parsed: TokenizedText = serde_json::from_str(&text)
        .map_err(|e| format!("Failed to parse JSON from Gemini: {}", e))?;

    repair_spans(french_text, &mut parsed.fr_tokens);
    parsed.provider_used = Some(format!("gemini/{}", model));

    Ok(parsed)
}

pub async fn mock_tokenize_async(french_text: &str) -> Result<TokenizedText, String> {
    let splits = split_tokens_with_punct(french_text);
    let mut fr_tokens = Vec::new();
    let mut en_tokens = Vec::new();
    let mut en_idx = 0usize;

    for (content, trailing_punct, leading_punct) in splits {
        // Calculate character positions in the original text
        let char_start = french_text.find(&content).unwrap_or(0);
        // Simple approximation: find the first occurrence and use content length
        let char_end = char_start + content.chars().count();

        fr_tokens.push(FrenchToken {
            text: content.to_string(),
            trailing_punct: trailing_punct.to_string(),
            leading_punct: leading_punct.to_string(),
            translation: content.to_string(), // Mock: no real translation
            spans: vec![Span([char_start, char_end])],
            en_indices: vec![en_idx],
        });
        en_tokens.push(EnglishToken {
            text: format!("{} ", content),
            index: en_idx,
        });
        en_idx += 1;
    }

    Ok(TokenizedText {
        fr_tokens,
        en_tokens,
        book_id: 0,
        provider_used: Some("mock".to_string()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_deserializer() {
        let s = r#"[[30, 60]]"#;
        let spans: Vec<Span> = serde_json::from_str(s).unwrap();
        assert_eq!(spans, vec![Span([30, 60])]);

        let s = r#"[["02", "30"]]"#;
        let spans: Vec<Span> = serde_json::from_str(s).unwrap();
        assert_eq!(spans, vec![Span([2, 30])]);

        let s = r#"[[0, 10], [20, 35]]"#;
        let spans: Vec<Span> = serde_json::from_str(s).unwrap();
        assert_eq!(spans, vec![Span([0, 10]), Span([20, 35])]);

        let s = r#"[[-1, 30]]"#;
        assert!(serde_json::from_str::<Vec<Span>>(s).is_err());

        let s = r#"[[1.5, 30]]"#;
        assert!(serde_json::from_str::<Vec<Span>>(s).is_err());

        let s = r#"[[30]]"#;
        assert!(serde_json::from_str::<Vec<Span>>(s).is_err());

        let s = r#"[["hello", "30"]]"#;
        assert!(serde_json::from_str::<Vec<Span>>(s).is_err());
    }

    // Punctuation splitting tests
    #[test]
    fn test_split_simple_trailing_comma() {
        // "Le chat," splits at whitespace into "Le" and "chat,"
        // "chat," then has comma extracted as trailing punctuation
        let result = split_tokens_with_punct("Le chat,");
        assert_eq!(result.len(), 2); // "Le" and "chat," -> 2 words
        // First word "Le" has no trailing
        assert_eq!(result[0].0, "Le");
        assert_eq!(result[0].1, "");
        // Second word "chat," -> content "chat", trailing ","
        assert_eq!(result[1].0, "chat");
        assert_eq!(result[1].1, ",");
    }

    #[test]
    fn test_split_contractions_preserved() {
        // Apostrophe is NOT a punctuation separator for contractions
        let result = split_tokens_with_punct("d'une");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "d'une");
        assert_eq!(result[0].1, "");
        assert_eq!(result[0].2, "");
    }

    #[test]
    fn test_split_elided_article() {
        let result = split_tokens_with_punct("l'homme");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "l'homme");
        assert_eq!(result[0].1, "");
        assert_eq!(result[0].2, "");
    }

    #[test]
    fn test_split_parentheses() {
        let result = split_tokens_with_punct("(texte)");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "texte");
        assert_eq!(result[0].1, ")");
        assert_eq!(result[0].2, "(");
    }

    #[test]
    fn test_split_mixed_punctuation() {
        // "Le chat, pas le chien." splits at whitespace:
        // ["Le", "chat,", "pas", "le", "chien."]
        let result = split_tokens_with_punct("Le chat, pas le chien.");
        assert_eq!(result.len(), 5); // 5 words
        // Last word "chien." should have period as trailing
        assert_eq!(result[4].0, "chien");
        assert_eq!(result[4].1, ".");
        // "chat," should have comma as trailing
        assert_eq!(result[1].0, "chat");
        assert_eq!(result[1].1, ",");
    }

    #[test]
    fn test_split_empty_input() {
        let result = split_tokens_with_punct("");
        assert!(result.is_empty());
    }

    #[test]
    fn test_split_sentence_with_period() {
        let result = split_tokens_with_punct("Bonjour monde.");
        assert_eq!(result.len(), 2);
        // Second token should be "monde" with trailing "."
        assert_eq!(result[1].0, "monde");
        assert_eq!(result[1].1, ".");
    }

    #[test]
    fn test_split_quotes() {
        let result = split_tokens_with_punct(r#""Bonjour""#);
        // Should handle quotation marks
        assert!(result.iter().any(|(c, _, _)| c == "Bonjour"));
    }

    #[test]
    fn test_split_complex_contraction() {
        let result = split_tokens_with_punct("qu'il");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "qu'il");
        assert_eq!(result[0].1, "");
        assert_eq!(result[0].2, "");
    }

    #[test]
    fn test_split_whitespace_only() {
        let result = split_tokens_with_punct("   ");
        assert!(result.is_empty());
    }

    #[test]
    fn test_split_example_from_spec() {
        let result = split_tokens_with_punct("Le petit Georges, à quatre pattes.");
        // The input has 6 words: "Le", "petit", "Georges,", "à", "quatre", "pattes."
        // Each becomes a token with punctuation extracted
        assert_eq!(result.len(), 6);
        // Georges, has comma as trailing
        assert!(result.iter().any(|(c, t, _)| c == "Georges" && t == ","));
        // pattiens. has period as trailing
        assert!(result.iter().any(|(c, t, _)| c == "pattes" && t == "."));
    }

    #[test]
    fn test_split_em_dash() {
        // "Il dit — enfin, il crie." splits into: ["Il", "dit", "—", "enfin,", "il", "crie."]
        let result = split_tokens_with_punct("Il dit — enfin, il crie.");
        // "—" is a standalone word with em-dash as its content (and no trailing punct)
        assert!(result.iter().any(|(c, t, _)| c == "—" && t.is_empty()));
        // "enfin," has comma as trailing
        assert!(result.iter().any(|(c, t, _)| c == "enfin" && t == ","));
        // "crie." has period as trailing
        assert!(result.iter().any(|(c, t, _)| c == "crie" && t == "."));
    }

    #[test]
    fn test_split_semicolon() {
        // "Il vient; elle part." splits into: ["Il", "vient;", "elle", "part."]
        let result = split_tokens_with_punct("Il vient; elle part.");
        // "vient;" has semicolon as trailing
        assert!(result.iter().any(|(c, t, _)| c == "vient" && t == ";"));
        // "part." has period as trailing
        assert!(result.iter().any(|(c, t, _)| c == "part" && t == "."));
    }

    #[test]
    fn test_split_colon() {
        // "Il répond : oui." splits into: ["Il", "répond", ":", "oui."]
        let result = split_tokens_with_punct("Il répond : oui.");
        // ":" is a standalone word with colon as its content
        assert!(result.iter().any(|(c, t, _)| c == ":" && t.is_empty()));
        // "oui." has period as trailing
        assert!(result.iter().any(|(c, t, _)| c == "oui" && t == "."));
    }

    #[test]
    fn test_split_guillemets() {
        // '"Il dit : «Oui !»"' splits into words with « » as leading/trailing
        let result = split_tokens_with_punct(r#"Il dit : «Oui !»"#);
        // «Oui is a word with « as leading
        assert!(result.iter().any(|(c, t, l)| c == "Oui" && l == "«"));
        // !» is captured as trailing punctuation (content empty, trailing contains both)
        assert!(result.iter().any(|(c, t, _)| t.contains('»') || c.contains('»')));
    }

    #[test]
    fn test_token_full_text_with_leading() {
        let token = FrenchToken {
            text: "texte".to_string(),
            trailing_punct: ")".to_string(),
            leading_punct: "(".to_string(),
            translation: "(text)".to_string(),
            spans: vec![Span([0, 7])],
            en_indices: vec![0],
        };
        assert_eq!(token.full_text(), "(texte)");
    }

    #[tokio::test]
    async fn test_mock_tokenize_async() {
        let result = mock_tokenize_async("Bonjour monde").await;
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(tokens.fr_tokens.len(), 2);
        assert_eq!(tokens.en_tokens.len(), 2);
        assert_eq!(tokens.fr_tokens[0].text, "Bonjour");
    }

    #[test]
    fn test_french_token_full_text() {
        let token = FrenchToken {
            text: "Le chat".to_string(),
            trailing_punct: ",".to_string(),
            leading_punct: "".to_string(),
            translation: "The cat,".to_string(),
            spans: vec![Span([0, 7])],
            en_indices: vec![0],
        };
        assert_eq!(token.full_text(), "Le chat,");
    }

    #[test]
    fn test_translation_provider_display() {
        let provider = TranslationProvider::OpenRouter("z-ai/glm-4.5-air:free".to_string());
        assert_eq!(provider.display(), "openrouter/z-ai/glm-4.5-air:free");



        let provider = TranslationProvider::Groq("llama-3.1-8b-instant".to_string());
        assert_eq!(provider.display(), "groq/llama-3.1-8b-instant");

        let provider = TranslationProvider::Gemini("gemini-3.1-flash-lite".to_string());
        assert_eq!(provider.display(), "gemini/gemini-3.1-flash-lite");
    }

    #[tokio::test]
    async fn test_openrouter_integration() {
        // Skip if no API key
        let api_key = match std::env::var("OPENROUTER_API_KEY") {
            Ok(k) if !k.is_empty() => k,
            _ => {
                tracing::info!("Skipping OpenRouter integration test: OPENROUTER_API_KEY not set");
                return;
            }
        };


        let client = reqwest::Client::new();
        let french_text = "Le chat dort.";
        let model = "z-ai/glm-4.5-air:free";

        tracing::info!("Testing OpenRouter with model: {}", model);
        let result = tokenize_openrouter(&client, &api_key, french_text, model).await;

        match result {
            Ok(tokenized) => {
                tracing::info!("SUCCESS: Got {} tokens", tokenized.fr_tokens.len());
                assert!(!tokenized.fr_tokens.is_empty(), "Should have French tokens");
                assert_eq!(tokenized.provider_used.as_deref(), Some("openrouter/z-ai/glm-4.5-air:free"));
                // Verify translations exist
                for token in &tokenized.fr_tokens {
                    assert!(!token.translation.is_empty(), "Token '{}' should have translation", token.text);
                }
            }
            Err(e) => {
                panic!("OpenRouter failed: {}", e);
            }
        }
    }
}