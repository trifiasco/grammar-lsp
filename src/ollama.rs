use serde::{Deserialize, Serialize};

/// Ollama API request structure
#[derive(Debug, Serialize)]
pub struct OllamaRequest {
    pub model: String,
    pub prompt: String,
    pub format: String,
    pub stream: bool,
}

/// Ollama API response structure
#[derive(Debug, Deserialize)]
pub struct OllamaApiResponse {
    pub response: String,
}

/// Grammar issue detected by LLM
#[derive(Debug, Deserialize, Serialize)]
pub struct GrammarIssue {
    pub line: u32,
    pub column: u32,
    pub message: String,
}

/// Structured response from Ollama containing grammar issues
#[derive(Debug, Deserialize, Serialize)]
pub struct OllamaResponse {
    pub issues: Vec<GrammarIssue>,
}
