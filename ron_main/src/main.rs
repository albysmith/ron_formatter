use std::error::Error;
use std::fmt::Display;

mod logos_parse;

#[derive(Debug)]
enum RonError {
    Nothing,
    RonThing,
}
impl Error for RonError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
impl Display for RonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RonError::Nothing => write!(f, "Nothing"),
            RonError::RonThing => write!(f, "RonThing"),
        }
    }
}

const TEST: &str = "
#sick hash comment
/*
 * RON now has multi-line (C-style) block comments!
 * They can be freely nested:
 * /* This is a nested comment */
 * If you just want a single-line comment,
 * do it like here:
// Just put two slashes before the comment and the rest of the line
// can be used freely!
*/

// Note that block comments can not be started in a line comment
// (Putting a /* here will have no effect)

(
    boolean: true,
    float: 8.2,
    map: {
        1: '1',
        2: '4',
        3: '9',
        4: '1',
        5: '2',
        6: '3',
    },
    nested: Nested(
        a: \"Decode me!\",
        b: 'z',
    ),
    option: Some(\t  \"Weird formatting!\" \n\n ),
    tuple: (3 /*(2 + 1)*/, 7 /*(2 * 5 - 3)*/),
)";

struct State {
    mode: Modes,
    previous: char,
}

#[derive(Debug, PartialEq)]
enum Modes {
    Start,
    Space,
    Comment,
    QuoteStart,
    QuoteEnd,
    Slash,
    Star
}
impl State {
    fn change_state(&mut self, c: char) {
        match c {
            ' ' => {
                if self.skip_state() == false {
                    self.mode = Modes::Space
                }
            }
            '#' => {
                if self.skip_state() == false {
                    self.mode = Modes::Comment
                }
            }
            '/' => {
                if self.skip_state() == false {
                    if self.mode == Modes::Slash {
                        self.mode = Modes::Comment
                    } else {
                        self.mode = Modes::Slash
                    }
                }
            }
            '*' => {
                if self.skip_state() == false {
                    if self.mode == Modes::Slash {
                        self.mode = Modes::Comment
                    } else {
                        self.mode = Modes::Star
                    }
                } else {
                    self.mode = Modes::Star
                }
            }
            '"' | '\'' => {
                if self.skip_state() == false {
                    self.mode = Modes::QuoteStart
                } else {
                    self.mode = Modes::QuoteEnd
                }
            }
            _ => (),
        };
        self.previous = c;
    }
    fn apply_state(&self, c: char) -> Option<char> {
        match self.mode {
            Modes::Start => Some(c),
            Modes::Space => Some(c),
            Modes::Comment => Some(c),
            Modes::QuoteStart => Some(c),
            Modes::QuoteEnd => Some(c),
            Modes::Slash => Some(c),
            Modes::Star => Some(c),
        }
    }
    fn skip_state(&self) -> bool {
        if self.mode == Modes::QuoteStart || self.mode == Modes::Comment {
            true
        } else {
            false
        }
    }
}
fn main() {
    let mut state = State {
        mode: Modes::Start,
        previous: ' ',
    };
    let x = TEST
        .chars()
        .filter_map(|c| {
            state.change_state(c);
            state.apply_state(c)
        })
        .collect::<String>();
    println!("{}", x)
}

// fn main() {
//     println!("Hello, world!");
//     let string = String::from("test");
//     let test = test_error(string);
//     let q = question(test);
//     match q {
//         Ok(_) => {}
//         Err(e) => {println!("{}", e)}
//     }
// }

// fn question(result: Result<String, RonError>) -> Result<String, RonError> {
//     let x  = result?;

//     Ok(x)
// }

// fn test_error(string: String) -> Result<String, RonError> {
//     match string.as_str() {
//         "pizza" => Ok(string),
//         _ => Err(RonError::Nothing)
//     }
// }
