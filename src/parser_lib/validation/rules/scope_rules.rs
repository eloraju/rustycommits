use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScopeRules {
    allowed: Vec<String>,
    required: bool,
}

impl Default for ScopeRules {
    fn default() -> Self {
        Self {
            allowed: vec![
                "api".to_string(),
                "cli".to_string(),
                "rules".to_string(),
                "parsing".to_string(),
                "lexing".to_string(),
            ],
            required: false,
        }
    }
}
