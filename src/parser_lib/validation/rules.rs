pub struct MessageRules {
    topic: TopicRules,
    scope: ScopeRules,
    description: DescriptionRules,
    body: BodyRules,
    footers: FooterRules,
}
pub struct TopicRules {
    allowed: Vec<String>,
}

pub struct ScopeRules {
    allowed: Vec<String>,
}

pub struct DescriptionRules {
    max_length: u32,
    space_afer_colon: bool,
}

pub struct BodyRules {
    max_length: u32,
}

pub struct FooterRules {
    allow_multi_line: bool,
}
