use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicRules {
    allowed: Vec<String>,
    required: bool,
}

impl Default for TopicRules {
    fn default() -> Self {
        Self {
            allowed: vec![
                "chore".to_string(),
                "feat".to_string(),
                "fix".to_string(),
                "refactor".to_string(),
                "test".to_string(),
                "docs".to_string(),
            ],
            required: true,
        }
    }
}
