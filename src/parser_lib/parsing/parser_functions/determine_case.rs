use crate::parser_lib::parsing::types::TextCase;

macro_rules! generate_match {
    ($init_state:ident => $(($state_predicate:ident => $(($predicate:expr, $result:expr)),*)),* ) => {
        match $init_state {
            $(
                TextCase::$state_predicate => {
                    // Dirty hack to generate the if else tree
                    if false {unreachable!()}
                    $(else if $predicate {
                        $result
                    })*
                    else {
                        TextCase::Unknown
                    }
                }
            )*
        }
    };
}

pub fn determine_text_case(content: &str) -> TextCase {
    let mut char_iter = content.chars();
    let mut found_case: TextCase = match char_iter.next() {
        Some(c) => {
            if c.is_ascii_uppercase() {
                TextCase::Upper
            } else if c.is_ascii_lowercase() {
                TextCase::Lower
            } else {
                panic!("Invalid character: Word can't begin with '{}'", c);
            }
        }
        None => panic!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"),
    };
    for char in char_iter {
        found_case = generate_match!(found_case =>
            (Upper =>
                (char.is_ascii_uppercase(), TextCase::Upper),
                (char.is_ascii_lowercase(), TextCase::Pascal),
                (char == '_', TextCase::Scream),
                (char == ' ', TextCase::Scream)
            ),
            (Lower =>
                (char.is_ascii_uppercase(), TextCase::Camel),
                (char.is_ascii_lowercase(), TextCase::Lower),
                (char == '_', TextCase::Snake),
                (char == '-', TextCase::Kebab)
            ),
            (Camel =>
                (char.is_ascii_uppercase(), TextCase::Camel),
                (char.is_ascii_lowercase(), TextCase::Camel),
                (char == '-', TextCase::Kebab),
                (char == '_', TextCase::Snake)
            ),
            (Kebab =>
                (char == '-', TextCase::Kebab),
                (char.is_ascii_lowercase(), TextCase::Kebab)
            ),
            (Pascal =>
                (char.is_ascii_uppercase(), TextCase::Pascal),
                (char.is_ascii_lowercase(), TextCase::Pascal),
                (char == '_', TextCase::Scream)
            ),
            (Scream =>
                (char.is_ascii_uppercase(), TextCase::Scream),
                (char == '_', TextCase::Scream),
                (char == ' ', TextCase::Scream)
            ),
            (Snake =>
                (char.is_ascii_lowercase(), TextCase::Snake),
                (char == '_', TextCase::Snake),
                (char.is_ascii_uppercase(), TextCase::Scream)
            ),
            (Unknown => (true, TextCase::Unknown))
        );
    }
    return found_case;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_happy_cases() {
        assert!(matches!(determine_text_case("testText"), TextCase::Camel));
        assert!(matches!(determine_text_case("TestText"), TextCase::Pascal));
        assert!(matches!(determine_text_case("test-text"), TextCase::Kebab));
        assert!(matches!(determine_text_case("test_text"), TextCase::Snake));
        assert!(matches!(determine_text_case("TEST_TEXT"), TextCase::Scream));
        assert!(matches!(determine_text_case("TEST TEXT"), TextCase::Scream));
        assert!(matches!(determine_text_case("TEST"), TextCase::Upper));
        assert!(matches!(determine_text_case("test"), TextCase::Lower));
        assert!(matches!(
            determine_text_case("TesT-TextT"),
            TextCase::Unknown
        ));
    }

    #[test]
    #[should_panic]
    fn test_determine_text_case_invalid() {
        determine_text_case("1testText");
    }
}
