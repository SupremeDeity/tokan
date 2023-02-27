use std::process::exit;

#[derive(Debug)]
enum Lexicons {
    Numeric(f32),
    String(String),
    ParenthesisOpen,
    ParenthesisClose,
    OperatorSlash,
    OperatorAsterisk,
    OperatorPlus,
    OperatorMinus,
    LineBreak,
    Print,
}
pub struct Lexer<'a> {
    pub content: &'a [char],
    cursor: usize,
    tokens: Vec<Lexicons>,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self {
            content: content,
            cursor: 0,
            tokens: Vec::new(),
        }
    }

    fn tokenize(&mut self) {
        let mut group = String::new();
        let current_char = self.content[self.cursor];
        if current_char.is_whitespace() {
            if current_char == '\n' {
                self.tokens.push(Lexicons::LineBreak);
            }
        } else if current_char == '*' {
            self.tokens.push(Lexicons::OperatorAsterisk);
        } else if current_char == '/' {
            self.tokens.push(Lexicons::OperatorSlash);
        } else if current_char == '+' {
            self.tokens.push(Lexicons::OperatorPlus);
        } else if current_char == '-' {
            self.tokens.push(Lexicons::OperatorMinus);
        } else if current_char == '(' {
            self.tokens.push(Lexicons::ParenthesisOpen);
        } else if current_char == ')' {
            self.tokens.push(Lexicons::ParenthesisClose);
        } else if current_char == '"' {
            while let Some(chr) = self.next() {
                if chr != '"' {
                    group.push(chr);
                    self.cursor += 1;
                } else {
                    self.tokens.push(Lexicons::String(group.to_string()));
                    group.clear();
                    break;
                }
            }
        } else if current_char.is_ascii_alphabetic() {
            group.push(current_char);
            while let Some(chr) = self.next() {
                if !chr.is_ascii_whitespace() {
                    group.push(chr);
                    self.cursor += 1;
                } else {
                    if group == "PRINT" {
                        self.tokens.push(Lexicons::Print);
                        group.clear();
                        break;
                    }
                }
            }
        } else if current_char.is_numeric() {
            group.push(current_char);
            while let Some(chr) = self.next() {
                if !chr.is_ascii_punctuation() && !chr.is_whitespace() {
                    if chr.is_numeric() {
                        group.push(chr);
                        self.cursor += 1;
                    } else {
                        eprintln!(
                            "[ERROR]: PARSING AT {cursor} => {chr}",
                            cursor = self.cursor
                        );
                        exit(1);
                    }
                } else {
                    if let Ok(parsed_num) = group.parse::<f32>() {
                        self.tokens.push(Lexicons::Numeric(parsed_num));
                        group.clear();
                        break;
                    } else {
                        eprintln!(
                            "[ERROR]: CONVERTING NUMERIC AT {cursor}",
                            cursor = self.cursor
                        );
                        exit(1);
                    }
                }
            }
        } else if current_char == '\n' {
            self.tokens.push(Lexicons::LineBreak);
        } else {
            eprintln!(
                "[ERROR]: PARSING AT {cursor} => {chr}",
                cursor = self.cursor,
                chr = current_char
            );
            exit(1);
        }
        self.cursor += 1;
        if let Some(_) = self.next() {
            self.parse();
        } else {
            println!("{tokens:?}", tokens = self.tokens);
        }
    }

    // TODO: Do AST generation
    pub fn parse(&mut self) {
        self.tokenize();
        
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor + 1 >= self.content.len() {
            return None;
        }
        Some(self.content[self.cursor + 1])
    }
}
