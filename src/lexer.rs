use std::vec;

#[derive(Copy, Clone, Debug)]
pub enum TokenType {
    Addition,
    Subtraction,
    Division,
    Multiplication,
    Number,
    LeftParen,
    RightParen
}

#[derive(Clone, Debug)]
pub struct Position {
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug)]
pub struct Token {
   pub token_type: TokenType,
   pub value: String,
   position: Position,
}

pub struct Lexer {
    pub input: String,
    tokens: Vec<Token>,
    index: usize,
    token_start_index: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            tokens: vec![],
            index: 0,
            token_start_index: 0,
            line: 0,
            column: 0,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        println!("Lexer: {}", self.input);

        while self.index < self.input.len() {
            let ch: char = self.peek();

            if self.check_is_whitespace(ch) {
                // Ignore whitespace.
                self.consume();
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

            // TODO: Handle decimals
            if ch.is_numeric() {
                self.begin_token();
                while self.peek().is_numeric() {
                    self.consume();
                }
                self.commit_token(TokenType::Number);
                continue;
            }

            if ch == '\n' {
                self.consume();
                self.line += 1;
                self.column = 0;
                continue;
            }

            println!("Unhandled Token: {}", ch);
            self.consume();

        }

        return self.tokens.clone();
    }

    pub fn print_tokens(tokens: Vec<Token>) {
        for x in &tokens {
            println!("{:?}: {}:{}: {}", x.token_type, x.position.line, x.position.start, x.value);
        }
    }

    fn peek(&mut self) -> char {
        if self.index + 1 > self.input.len() {
            return '\0';
        }
        return self.input.chars().nth(self.index).unwrap();
    }

    fn consume(&mut self) {
        self.index += 1;
        self.column += 1;
    }

    fn add_token(&mut self, typ: TokenType) {
        let token = Token {
            token_type: typ,
            value: self.input.chars().nth(self.index).unwrap().to_string().clone(),
            position: Position { line:self.line, start: self.column, end: 0 }
        };
        self.tokens.push(token);
        self.consume();
    }

    fn begin_token(&mut self) {
        self.token_start_index = self.index;
    }

    fn commit_token(&mut self, typ: TokenType)  {
        let value: String = self.input.chars().skip(self.token_start_index).take(self.index - self.token_start_index).collect();
        let token = Token {
            token_type: typ,
            value: value.clone(),
            position: Position { line:self.line, start: self.column - value.len(), end: self.index }
        };
        self.tokens.push(token);
    }

    fn check_is_whitespace(&mut self, ch: char) -> bool {
        return ch == ' ' || ch == '\r' || ch == '\t';
    }
}
