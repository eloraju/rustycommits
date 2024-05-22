use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FooterRules {
    allow_multi_line: bool,
}

impl Default for FooterRules {
    fn default() -> Self {
        Self {
            allow_multi_line: true,
        }
    }
}
