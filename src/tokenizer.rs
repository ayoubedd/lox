use std::collections::LinkedList;
use std::process::exit;
use std::str;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Token {
    token: TokenType,
    line: u64,
}

#[derive(Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,

    // literals.
    Identifier(String),
    String(String),
    Number(f64),

    // keywords.
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

fn match_str(input: &[u8], str: &str) -> bool {
    if input.len() < str.len() {
        return false;
    }

    let one = std::str::from_utf8(&input[0..str.len()]).unwrap();

    if one[..] != *str {
        return false;
    }

    true
}

fn peek(input: &[u8], target: char) -> Option<usize> {
    let mut i: usize = 0;

    for c in input {
        if *c as char == target {
            return Some(i);
        }
        i += 1;
    }

    None
}

pub fn tokenize(input: &str, file_name: &str) -> Result<LinkedList<Token>, String> {
    let mut tokens: LinkedList<Token> = LinkedList::new();

    let mut line: u64 = 1;
    // let mut start: u64 = 0;
    // let mut current: u64 = 0;

    let mut i: usize = 0;
    let input = input.as_bytes();

    while i < input.len() {
        let c: char = input[i] as char;
        match c {
            '\n' => line += 1,

            '+' => tokens.push_back(Token {
                token: TokenType::Plus,
                line,
            }),

            '-' => tokens.push_back(Token {
                token: TokenType::Minus,
                line,
            }),

            ',' => tokens.push_back(Token {
                token: TokenType::Comma,
                line,
            }),

            ';' => tokens.push_back(Token {
                token: TokenType::Semicolon,
                line,
            }),

            '(' => tokens.push_back(Token {
                token: TokenType::LeftParen,
                line,
            }),

            ')' => tokens.push_back(Token {
                token: TokenType::RightParen,
                line,
            }),

            '{' => tokens.push_back(Token {
                token: TokenType::LeftBrace,
                line,
            }),

            '}' => tokens.push_back(Token {
                token: TokenType::RightBrace,
                line,
            }),

            '#' => tokens.push_back(Token {
                token: TokenType::Bang,
                line,
            }),

            '.' => tokens.push_back(Token {
                token: TokenType::Dot,
                line,
            }),

            '&' => tokens.push_back(Token {
                token: TokenType::And,
                line,
            }),

            '|' => {
                if match_str(&input[i + 1..], "|") {
                    println!("bro wtf");
                    tokens.push_back(Token {
                        token: TokenType::Or,
                        line,
                    });

                    i += 1;
                } else {
                    return Err(format!("{}:{} Unexpected charater '{}'", file_name.to_string(), line, c.to_string()));
                };
            }

            '!' => {
                if match_str(&input[i + 1..], "=") {
                    tokens.push_back(Token {
                        token: TokenType::BangEqual,
                        line,
                    });
                    i += 1;
                } else {
                    tokens.push_back(Token {
                        token: TokenType::Bang,
                        line,
                    });
                }
            }

            '=' => {
                if match_str(&input[i + 1..], "=") {
                    tokens.push_back(Token {
                        token: TokenType::EqualEqual,
                        line,
                    });
                    i += 1;
                } else {
                    tokens.push_back(Token {
                        token: TokenType::Equal,
                        line,
                    });
                }
            }

            '>' => {
                if match_str(&input[i + 1..], "=") {
                    tokens.push_back(Token {
                        token: TokenType::GreaterEqual,
                        line,
                    });
                    i += 1;
                } else {
                    tokens.push_back(Token {
                        token: TokenType::Greater,
                        line,
                    });
                }
            }

            '<' => {
                if match_str(&input[i + 1..], "=") {
                    tokens.push_back(Token {
                        token: TokenType::LessEqual,
                        line,
                    });
                    i += 1;
                } else {
                    tokens.push_back(Token {
                        token: TokenType::Less,
                        line,
                    });
                }
            }

            '*' => {
                tokens.push_back(Token {
                    token: TokenType::Star,
                    line,
                });
            }

            '/' => {
                if match_str(&input[i + 1..], "/") {
                    if let Some(j) = peek(&input[i..], '\n') {
                        i += j;
                    }
                } else {
                    tokens.push_back(Token {
                        token: TokenType::Slash,
                        line,
                    });
                }
            }

            // Liter string
            '"' => {
                if let Some(j) = peek(&input[i + 1..], '"') {
                    let string_literal = std::str::from_utf8(&input[i + 1..i + 1 + j]).unwrap();
                    tokens.push_back(Token {
                        token: TokenType::String(String::from(string_literal)),
                        line,
                    });

                    i += string_literal.len() + 1;
                } else {
                    return Err(format!("{}:{} Unterminated string", file_name.to_string(), line));
                }
            }

            // Literal number
            '0'..='9' => {}

            ' ' | '\r' | '\t' => {}

            _ => {
                return Err(format!("{}:{} Unexpected charater '{}'", file_name.to_string(), line, c.to_string()));
            }
        };
        i += 1;
    }

    tokens.push_back(Token {
        token: TokenType::Eof,
        line: line - 1,
    });

    Result::Ok(tokens)
}
