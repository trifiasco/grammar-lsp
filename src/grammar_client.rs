use crate::ollama::{GrammarIssue, OllamaApiResponse, OllamaRequest, OllamaResponse};

/// Grammar checking provider that interfaces with Ollama HTTP API
#[derive(Debug)]
pub struct GrammarCheckProvider {
    http_client: reqwest::Client,
    model: String,
    api_url: String,
    timeout_secs: u64,
}

impl GrammarCheckProvider {
    /// Create a new grammar check provider with default settings
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            model: "gemma3:4b".to_string(),
            api_url: "http://localhost:11434/api/generate".to_string(),
            timeout_secs: 60,
        }
    }

    /// Check grammar and spelling in the provided text
    pub async fn check_grammar(&self, text: &str) -> Vec<GrammarIssue> {
        eprintln!("[Grammar Check] Starting check for {} bytes of text", text.len());

        let prompt = self.build_prompt(text);
        let request = self.build_request(prompt);

        match self.send_request(request).await {
            Ok(api_response) => {
                eprintln!("[Grammar Check] Response: {}", api_response.response);
                self.parse_response(&api_response.response)
            }
            Err(e) => {
                eprintln!("[ERROR] Grammar check failed: {}", e);
                vec![]
            }
        }
    }

    /// Build the prompt for the LLM
    fn build_prompt(&self, text: &str) -> String {
        format!(
            r#"You are a grammar and spelling checker. Your task is to find errors in the text below.

  IMPORTANT: You must respond with ONLY valid JSON in this exact format:
  {{
    "issues": [
      {{"line": 1, "column": 5, "message": "Spelling: 'teh' should be 'the'"}},
      {{"line": 2, "column": 10, "message": "Grammar: 'was went' should be 'went'"}}
    ]
  }}

  If there are no errors, return: {{"issues": []}}

  Rules:
  1. line numbers start at 1
  2. column numbers start at 0
  3. message format: "<error type>: '<incorrect>' should be '<correct>'"
  4. error types: "Spelling" or "Grammar"
  5. Do NOT include explanations, only the JSON object

  Text to analyze:
  {}"#
,
            text
        )
    }

    /// Build Ollama API request
    fn build_request(&self, prompt: String) -> OllamaRequest {
        OllamaRequest {
            model: self.model.clone(),
            prompt,
            format: "json".to_string(),
            stream: false,
        }
    }

    /// Send request to Ollama API with timeout
    async fn send_request(&self, request: OllamaRequest) -> Result<OllamaApiResponse, String> {
        eprintln!("[Grammar Check] Calling Ollama API...");

        let response = tokio::time::timeout(
            std::time::Duration::from_secs(self.timeout_secs),
            self.http_client
                .post(&self.api_url)
                .json(&request)
                .send(),
        )
        .await
        .map_err(|_| "Ollama timeout".to_string())?
        .map_err(|e| format!("Ollama request failed: {}", e))?;

        eprintln!("[Grammar Check] Ollama responded");

        response
            .json::<OllamaApiResponse>()
            .await
            .map_err(|e| format!("Failed to decode API response: {}", e))
    }

    /// Parse the JSON response from Ollama into grammar issues
    fn parse_response(&self, response_text: &str) -> Vec<GrammarIssue> {
        match serde_json::from_str::<OllamaResponse>(response_text) {
            Ok(grammar_response) => {
                eprintln!("[Grammar Check] Found {} issues", grammar_response.issues.len());
                grammar_response.issues
            }
            Err(e) => {
                eprintln!("[ERROR] Failed to parse JSON: {}", e);
                eprintln!("[ERROR] Raw response: {}", response_text);
                vec![]
            }
        }
    }
}

impl Default for GrammarCheckProvider {
    fn default() -> Self {
        Self::new()
    }
}
