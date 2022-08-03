use std::vec;
use std::format;

#[derive(Copy, Clone, Debug)]
pub enum TokenType {
    WhiteSpace,
    Addition,
    Subtraction,
    Division,
    Multiplication,
    Number,
    LeftParen,
    RightParen
}

#[derive(Clone, Debug)]
pub struct Token {
   pub token_type: TokenType,
   pub value: String,
}

pub struct Lexer {
    pub input: String,
    tokens: Vec<Token>,
    index: usize,
    token_start_index: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input: input,
            tokens: vec![],
            index: 0,
            token_start_index: 0,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        println!("Lexer: {}", self.input);

        let mut token_start_index = 0;

        while self.index < self.input.len() {
            let ch: char = self.peek();

            if ch.is_whitespace() {
                self.add_token(TokenType::WhiteSpace);
                continue;
            }

            if ch == '+' {
                self.add_token(TokenType::Addition);
                continue;
            }

            if ch == '-' {
                self.add_token(TokenType::Subtraction);
                continue;
            }

            if ch == '/' {
                self.add_token(TokenType::Division);
                continue;
            }

            if ch == '*' {
                self.add_token(TokenType::Multiplication);
                continue;
            }

            if ch == '(' {
                self.add_token(TokenType::LeftParen);
                continue;
            }

            if ch == ')' {
                self.add_token(TokenType::RightParen);
                continue;
            }


            if ch.is_numeric() {
                self.begin_token();
                while self.peek().is_numeric() {
                    self.consume();
                }
                self.commit_token(TokenType::Number);
                continue;
            }

            println!("Unhandled Token: {}", ch);
            self.consume();

        }

        return self.tokens.clone();
    }

    pub fn print_tokens(tokens: Vec<Token>) {
        for x in &tokens {
            println!("{:?}: {}", x.token_type, x.value);
        }
    }

    fn peek(&mut self) -> char {
        return self.input.chars().nth(self.index).unwrap();
    }

    fn consume(&mut self) {
        self.index += 1;
    }

    fn add_token(&mut self, typ: TokenType) {
        let mut token = Token {
            token_type: typ,
            value: self.input.chars().nth(self.index).unwrap().to_string().clone()
        };
        self.tokens.push(token);
        self.index += 1;
    }

    fn begin_token(&mut self) {
        self.token_start_index = self.index;
    }

    fn commit_token(&mut self, typ: TokenType)  {
        let value: String = self.input.chars().skip(self.token_start_index).take(self.index - self.token_start_index).collect();
        let mut token = Token {
            token_type: typ,
            value: value.clone()
        };
        self.tokens.push(token);
    }
}