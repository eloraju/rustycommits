use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BodyRules {
    max_length: u32,
    min_length: u32,
    required: bool,
}

impl Default for BodyRules {
    fn default() -> Self {
        Self {
            required: false,
            min_length: 20,
            max_length: 350,
        }
    }
}
