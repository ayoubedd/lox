use std::collections::HashMap;
use std::collections::LinkedList;

#[derive(Debug, Clone)]
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
    Fn,
    For,
    If,
    Nil,
    Print,
    Return,
    Super,
    This,
    True,
    Let,
    While,

    Eof,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Token {
    token: TokenType,
    line: usize,
}

pub struct Tokenizer {
    current: usize,
    line: usize,
    keywords: HashMap<&'static str, TokenType>,
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            current: 0,
            line: 0,
            keywords: Self::build_keywords_hash_set(),
        }
    }
    pub fn tokenize(&mut self, input: &str, file_name: &str) -> Result<LinkedList<Token>, String> {
        let mut tokens: LinkedList<Token> = LinkedList::new();

        // let mut current: u64 = 0;
        let mut start: usize;
        let input = input.as_bytes();

        while self.current < input.len() {
            let c: char = input[self.current] as char;
            match c {
                '\n' => self.line += 1,

                '+' => tokens.push_back(Token {
                    token: TokenType::Plus,
                    line: self.line,
                }),

                '-' => tokens.push_back(Token {
                    token: TokenType::Minus,
                    line: self.line,
                }),

                ',' => tokens.push_back(Token {
                    token: TokenType::Comma,
                    line: self.line,
                }),

                ';' => tokens.push_back(Token {
                    token: TokenType::Semicolon,
                    line: self.line,
                }),

                '(' => tokens.push_back(Token {
                    token: TokenType::LeftParen,
                    line: self.line,
                }),

                ')' => tokens.push_back(Token {
                    token: TokenType::RightParen,
                    line: self.line,
                }),

                '{' => tokens.push_back(Token {
                    token: TokenType::LeftBrace,
                    line: self.line,
                }),

                '}' => tokens.push_back(Token {
                    token: TokenType::RightBrace,
                    line: self.line,
                }),

                '#' => tokens.push_back(Token {
                    token: TokenType::Bang,
                    line: self.line,
                }),

                '.' => tokens.push_back(Token {
                    token: TokenType::Dot,
                    line: self.line,
                }),

                '&' => tokens.push_back(Token {
                    token: TokenType::And,
                    line: self.line,
                }),

                '|' => {
                    if Self::match_str(&input[self.current + 1..], "|") {
                        println!("bro wtf");
                        tokens.push_back(Token {
                            token: TokenType::Or,
                            line: self.line,
                        });

                        self.current += 1;
                    } else {
                        return Err(format!(
                            "{}:{} Unexpected charater '{}'",
                            file_name.to_string(),
                            self.line,
                            c.to_string()
                        ));
                    };
                }

                '!' => {
                    if Self::match_str(&input[self.current + 1..], "=") {
                        tokens.push_back(Token {
                            token: TokenType::BangEqual,
                            line: self.line,
                        });
                        self.current += 1;
                    } else {
                        tokens.push_back(Token {
                            token: TokenType::Bang,
                            line: self.line,
                        });
                    }
                }

                '=' => {
                    if Self::match_str(&input[self.current + 1..], "=") {
                        tokens.push_back(Token {
                            token: TokenType::EqualEqual,
                            line: self.line,
                        });
                        self.current += 1;
                    } else {
                        tokens.push_back(Token {
                            token: TokenType::Equal,
                            line: self.line,
                        });
                    }
                }

                '>' => {
                    if Self::match_str(&input[self.current + 1..], "=") {
                        tokens.push_back(Token {
                            token: TokenType::GreaterEqual,
                            line: self.line,
                        });
                        self.current += 1;
                    } else {
                        tokens.push_back(Token {
                            token: TokenType::Greater,
                            line: self.line,
                        });
                    }
                }

                '<' => {
                    if Self::match_str(&input[self.current + 1..], "=") {
                        tokens.push_back(Token {
                            token: TokenType::LessEqual,
                            line: self.line,
                        });
                        self.current += 1;
                    } else {
                        tokens.push_back(Token {
                            token: TokenType::Less,
                            line: self.line,
                        });
                    }
                }

                '*' => {
                    tokens.push_back(Token {
                        token: TokenType::Star,
                        line: self.line,
                    });
                }

                '/' => {
                    if Self::match_str(&input[self.current + 1..], "/") {
                        if let Some(j) = Self::peek(&input[self.current..], '\n') {
                            self.current += j;
                        }
                    } else {
                        tokens.push_back(Token {
                            token: TokenType::Slash,
                            line: self.line,
                        });
                    }
                }

                // Liter string
                '"' => {
                    if let Some(j) = Self::peek(&input[self.current + 1..], '"') {
                        let string_literal =
                            std::str::from_utf8(&input[self.current + 1..self.current + 1 + j])
                                .unwrap();
                        tokens.push_back(Token {
                            token: TokenType::String(String::from(string_literal)),
                            line: self.line,
                        });

                        self.current += string_literal.len() + 1;
                    } else {
                        return Err(format!(
                            "{}:{} Unterminated string",
                            file_name.to_string(),
                            self.line
                        ));
                    }
                }

                // Literal number
                '0'..='9' => {
                    start = self.current;
                    while Self::is_digit(input[self.current] as char) {
                        self.current += 1;
                    }
                    if input[self.current] as char == '.' {
                        self.current += 1;
                        let before = self.current;
                        while Self::is_digit(input[self.current] as char) {
                            self.current += 1;
                        }

                        if before == self.current {
                            return Err(format!("{}:{} invalid number", file_name, self.line));
                        }
                        self.current -= 1;
                    }

                    let number: f64 = std::str::from_utf8(&input[start..self.current])
                        .unwrap()
                        .parse()
                        .unwrap();

                    tokens.push_back(Token {
                        token: TokenType::Number(number),
                        line: self.line,
                    })
                }

                // Identifier
                'a'..='z' | 'A'..='Z' | '_' => {
                    start = self.current;
                    while (input[self.current] as char).is_alphanumeric() {
                        self.current += 1;
                    }
                    let str = std::str::from_utf8(&input[start..self.current]).unwrap();
                    if let Some(keyword) = self.keywords.get(str) {
                        // is a reserved keyword
                        tokens.push_back(Token {
                            token: keyword.clone(),
                            line: self.line,
                        });
                    } else {
                        // is a identifier
                        tokens.push_back(Token {
                            token: TokenType::Identifier(String::from(str)),
                            line: self.line,
                        });
                    }
                }

                // Ignoing white spaces
                ' ' | '\r' | '\t' => {}

                _ => {
                    return Err(format!(
                        "{}:{} Unexpected charater '{}'",
                        file_name.to_string(),
                        self.line,
                        c.to_string()
                    ));
                }
            };
            self.current += 1;
        }

        tokens.push_back(Token {
            token: TokenType::Eof,
            line: self.line,
        });

        Result::Ok(tokens)
    }

    fn advance(&mut self) {}

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

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    pub fn reset(&mut self) {
        self.current = 0;
        self.line = 0;
    }

    fn build_keywords_hash_set() -> HashMap<&'static str, TokenType> {
        let mut set: HashMap<&'static str, TokenType> = HashMap::new();

        let keywords = [
            // ("and", TokenType),
            ("class", TokenType::Class),
            ("else", TokenType::Else),
            ("false", TokenType::False),
            ("for", TokenType::For),
            ("fn", TokenType::Fn),
            ("if", TokenType::If),
            ("nil", TokenType::Nil),
            ("or", TokenType::Or),
            ("print", TokenType::Print),
            ("return", TokenType::Return),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("true", TokenType::True),
            ("let", TokenType::Let),
            ("while", TokenType::While),
        ];

        for pair in keywords {
            set.insert(pair.0, pair.1);
        }

        set
    }
}
