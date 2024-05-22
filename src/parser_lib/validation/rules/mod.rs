mod body_rules;
mod description_rules;
mod footer_rules;
mod scope_rules;
mod topic_rules;

use serde::{Deserialize, Serialize};

use self::{
    body_rules::BodyRules, description_rules::DescriptionRules, footer_rules::FooterRules,
    scope_rules::ScopeRules, topic_rules::TopicRules,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationRules {
    topic: TopicRules,
    scope: ScopeRules,
    description: DescriptionRules,
    body: BodyRules,
    footers: FooterRules,
}

impl Default for ValidationRules {
    fn default() -> Self {
        Self {
            topic: TopicRules::default(),
            scope: ScopeRules::default(),
            description: DescriptionRules::default(),
            body: BodyRules::default(),
            footers: FooterRules::default(),
        }
    }
}
