#[derive(Debug)]
pub struct EnvVar {
    key: String,
    value: String,
}

#[derive(Debug, PartialEq, Eq)]
enum State {
    FoundSeparator,
    EndOfString,
    ParsingSymbol,
}

#[derive(Debug)]
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
