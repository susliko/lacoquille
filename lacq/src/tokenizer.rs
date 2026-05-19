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
    pub text: String,
    #[serde(rename = "translation")]
    pub translation: String,
    #[serde(rename = "spans")]
    pub spans: Vec<Span>,
    #[serde(rename = "enIndices", default)]
    pub en_indices: Vec<usize>,
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
    code: Option<String>,
}

fn build_prompt(french_text: &str) -> String {
    format!(
        r#"You are a French language learning assistant. Tokenize this French text for parallel bilingual reading.

Rules:
- Each token is either: (a) a single word that makes sense on its own, OR (b) an expression/idiom where parts don't translate independently
- For multi-word tokens, they must be contiguous in the French text
- Output ONLY valid JSON matching this schema — no markdown, no explanation:
{{
  "frTokens": [
    {{
      "text": "the exact French text (word or phrase)",
      "translation": "English meaning",
      "spans": [[start, end], ...],  // char offsets in original text
      "enIndices": [N, ...]          // indices into enTokens array
    }}
  ],
  "enTokens": [
    {{ "text": "English text", "index": N }}
  ]
}}

IMPORTANT:
- enIndices must be consecutive and cover the entire English translation in order
- Output only the JSON — no code fences, no preamble, no commentary

French text:
{}"#,
        french_text
    )
}

pub async fn tokenize(
    client: &reqwest::Client,
    api_key: &str,
    french_text: &str,
) -> Result<TokenizedText, String> {
    let url = "https://api.groq.com/openai/v1/chat/completions";
    let prompt = build_prompt(french_text);

    let body = serde_json::json!({
        "model": "llama-3.1-8b-instant",
        "messages": [{ "role": "user", "content": prompt }],
        "response_format": { "type": "json_object" },
        "max_tokens": 2048,
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
        // Try to extract error message from JSON body
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

    // Check for API-level errors embedded in response
    if let Some(err) = groq_resp.error {
        return Err(format!("Groq API error: {} — {}", err.error_type, err.message));
    }

    let choice = groq_resp
        .choices
        .first()
        .ok_or_else(|| "No choices in Groq response".to_string())?;

    // Strip think blocks (e.g. <think>...</think>) that qwen may emit
    let think_re: once_cell::sync::Lazy<Regex> =
        once_cell::sync::Lazy::new(|| Regex::new(r"<think>[\s\S]*?</think>").unwrap());

    let content = think_re.replace_all(&choice.message.content, "").trim().to_string();

    // Strip markdown code fences if present (defensive)
    let json_str = content
        .trim_start_matches("```json\n")
        .trim_start_matches("```\n")
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let parsed: TokenizedText = serde_json::from_str(json_str)
        .map_err(|e| format!(
            "Failed to parse JSON from Groq: {}. Raw: {}",
            e,
            &json_str[..json_str.len().min(300)]
        ))?;

    // Validate enIndices invariant: must be consecutive [0, N) covering all English tokens
    if !parsed.en_tokens.is_empty() {
        let n = parsed.en_tokens.len();
        let mut seen = vec![false; n];
        for ft in &parsed.fr_tokens {
            for &idx in &ft.en_indices {
                if idx >= n {
                    return Err(format!(
                        "enIndices out of bounds: {} >= {} (token '{}')",
                        idx, n, ft.text
                    ));
                }
                seen[idx] = true;
            }
        }
        let missing: Vec<usize> = (0..n).filter(|&i| !seen[i]).collect();
        if !missing.is_empty() {
            return Err(format!(
                "enIndices invariant violated: missing indices {:?} — English tokens must be fully covered with no gaps",
                &missing[..missing.len().min(5)]
            ));
        }
    }

    Ok(parsed)
}

pub async fn mock_tokenize_async(french_text: &str) -> Result<TokenizedText, String> {
    let mut fr_tokens = Vec::new();
    let mut en_tokens = Vec::new();
    let mut en_idx = 0usize;
    let word_regex = Regex::new(r"(\S+)").unwrap();
    for cap in word_regex.captures_iter(french_text) {
        let m = cap.get(1).unwrap();
        let start = m.start();
        let end = m.end();
        let word = m.as_str();
        let translation = word.to_string();
        fr_tokens.push(FrenchToken {
            text: word.to_string(),
            translation: translation.clone(),
            spans: vec![Span([start, end])],
            en_indices: vec![en_idx],
        });
        en_tokens.push(EnglishToken {
            text: format!("{} ", translation),
            index: en_idx,
        });
        en_idx += 1;
    }
    Ok(TokenizedText {
        fr_tokens,
        en_tokens,
        book_id: 0,
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_deserializer() {
        // Plain number arrays: [30, 60]
        let s = r#"[[30, 60]]"#;
        let spans: Vec<Span> = serde_json::from_str(s).unwrap();
        assert_eq!(spans, vec![Span([30, 60])]);

        // String-encoded numbers (e.g. "02"): Llama sometimes emits these
        let s = r#"[["02", "30"]]"#;
        let spans: Vec<Span> = serde_json::from_str(s).unwrap();
        assert_eq!(spans, vec![Span([2, 30])]);

        // Multiple spans
        let s = r#"[[0, 10], [20, 35]]"#;
        let spans: Vec<Span> = serde_json::from_str(s).unwrap();
        assert_eq!(spans, vec![Span([0, 10]), Span([20, 35])]);

        // Negative number must NOT parse
        let s = r#"[[-1, 30]]"#;
        assert!(serde_json::from_str::<Vec<Span>>(s).is_err());

        // Float must NOT parse
        let s = r#"[[1.5, 30]]"#;
        assert!(serde_json::from_str::<Vec<Span>>(s).is_err());

        // Too few elements must NOT parse
        let s = r#"[[30]]"#;
        assert!(serde_json::from_str::<Vec<Span>>(s).is_err());

        // Non-numeric string must NOT parse
        let s = r#"[["hello", "30"]]"#;
        assert!(serde_json::from_str::<Vec<Span>>(s).is_err());
    }

    #[tokio::test]
    async fn test_mock_tokenize_async() {
        let result = mock_tokenize_async("Bonjour monde").await;
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(tokens.fr_tokens.len(), 2);
        assert_eq!(tokens.en_tokens.len(), 2);
        assert_eq!(tokens.fr_tokens[0].text, "Bonjour");
        assert_eq!(tokens.en_tokens[0].text, "Bonjour ");
    }
}
