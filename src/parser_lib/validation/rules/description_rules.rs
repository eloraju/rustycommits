use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DescriptionRules {
    max_length: u32,
    space_afer_colon: bool,
    required: bool,
}

impl Default for DescriptionRules {
    fn default() -> Self {
        Self {
            max_length: 120,
            space_afer_colon: true,
            required: true,
        }
    }
}
