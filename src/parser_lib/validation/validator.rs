use crate::parser_lib::parsing::types::CommitMessage;

use super::rules::ValidationRules;

pub struct Validator {}

impl Validator {
    pub fn validate(msg: &CommitMessage, rules: ValidationRules) -> () {
        println!("Validating commit message: {:?}", msg);
        println!("Using rules: {:?}", rules);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate() {
        let msg = CommitMessage {
            topic: None,
            scope: None,
            description: None,
            body: None,
            footers: None,
        };
        let rules = ValidationRules::default();
        Validator::validate(&msg, rules);
    }
}
