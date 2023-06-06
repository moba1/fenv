#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct EnvVar {
    pub key: String,
    pub value: String,
}

#[derive(Debug, PartialEq, Eq)]
enum State {
    FoundSeparator,
    EndOfString,
    ParsingSymbol,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum ParseError {
    NotEnvVar,
}

pub fn parse(arg: &str) -> Result<EnvVar, ParseError> {
    let mut character_sequence = arg.chars();
    let mut key = String::new();

    let mut state = State::ParsingSymbol;
    while state == State::ParsingSymbol {
        state = match character_sequence.next() {
            Some('=') => State::FoundSeparator,
            Some(c) => {
                key.push(c);
                State::ParsingSymbol
            }
            None => State::EndOfString,
        }
    }
    match state {
        State::EndOfString => Err(ParseError::NotEnvVar),
        _ => Ok(EnvVar {
            key,
            value: String::from_iter(character_sequence),
        }),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_only_ascii_character() {
        assert_eq!(
            super::parse("test=1"),
            Ok(super::EnvVar {
                key: "test".to_string(),
                value: "1".to_string()
            })
        );
        assert_eq!(
            super::parse("test\0\n=1\r"),
            Ok(super::EnvVar {
                key: "test\0\n".to_string(),
                value: "1\r".to_string()
            })
        );
        assert_eq!(
            super::parse("test\"=1'"),
            Ok(super::EnvVar {
                key: "test\"".to_string(),
                value: "1'".to_string()
            })
        );
    }

    #[test]
    fn reject_non_env_var() {
        assert_eq!(super::parse("test"), Err(super::ParseError::NotEnvVar))
    }

    #[test]
    fn parse_non_ascii_character() {
        assert_eq!(
            super::parse("🚀=✨"),
            Ok(super::EnvVar {
                key: "🚀".to_string(),
                value: "✨".to_string()
            })
        )
    }
}
