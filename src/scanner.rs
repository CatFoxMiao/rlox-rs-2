use std::collections::HashMap;

use crate::token::*;

struct Scanner {
    source: String,
    current: usize,
    start: usize,
    tokens: Vec<Token>,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert(String::from("and"), TokenType::And);
        keywords.insert(String::from("class"), TokenType::Class);
        keywords.insert(String::from("else"), TokenType::Else);
        keywords.insert(String::from("false"), TokenType::False);
        keywords.insert(String::from("for"), TokenType::For);
        keywords.insert(String::from("fun"), TokenType::Fun);
        keywords.insert(String::from("if"), TokenType::If);
        keywords.insert(String::from("nil"), TokenType::Nil);
        keywords.insert(String::from("or"), TokenType::Or);
        keywords.insert(String::from("print"), TokenType::Print);
        keywords.insert(String::from("return"), TokenType::Return);
        keywords.insert(String::from("super"), TokenType::Super);
        keywords.insert(String::from("this"), TokenType::This);
        keywords.insert(String::from("true"), TokenType::True);
        keywords.insert(String::from("var"), TokenType::Var);
        keywords.insert(String::from("while"), TokenType::While);
        Scanner {
            source: source,
            start: 0,
            current: 0,
            tokens: vec![],
            line: 1,
            keywords: keywords,
        }
    }

    fn scan_token(&mut self) {
        self.start = self.current;
        match self.cur_char() {
            // 单字节符号
            '(' => self.add_token(TokenType::LeftParen, Literal::None),
            ')' => self.add_token(TokenType::RightParen, Literal::None),
            '{' => self.add_token(TokenType::LeftBrace, Literal::None),
            '}' => self.add_token(TokenType::RightBrace, Literal::None),
            ',' => self.add_token(TokenType::Comma, Literal::None),
            '.' => self.add_token(TokenType::Dot, Literal::None),
            '-' => self.add_token(TokenType::Minus, Literal::None),
            '+' => self.add_token(TokenType::Plus, Literal::None),
            ';' => self.add_token(TokenType::Semicolon, Literal::None),
            '/' => self.add_token(TokenType::Slash, Literal::None),
            '*' => self.add_token(TokenType::Star, Literal::None),

            // one char or two char
            '!' => match self.next_char_is_expected('=') {
                false => self.add_token(TokenType::Bang, Literal::None),
                true => self.add_token(TokenType::BangEqual, Literal::None),
            },

            '=' => match self.next_char_is_expected('=') {
                false => self.add_token(TokenType::Equal, Literal::None),
                true => self.add_token(TokenType::EqualEqual, Literal::None),
            },

            '>' => match self.next_char_is_expected('=') {
                false => self.add_token(TokenType::Greater, Literal::None),
                true => self.add_token(TokenType::GreaterEqual, Literal::None),
            },

            '<' => match self.next_char_is_expected('=') {
                false => self.add_token(TokenType::Less, Literal::None),
                true => self.add_token(TokenType::LessEqual, Literal::None),
            },

            ' ' => self.consume_char(),

            '"' => self.add_string(),

            // 单字节或双字节符号
            c if c.is_ascii_digit() => self.add_number(),
            c if c.is_alphabetic() => self.add_identifier(),
            _ => {}
        }
    }

    fn add_identifier(&mut self) {
        while self.cur_has_next_char()
            && (self.next_char().is_alphanumeric() || self.next_char() == '_')
        {
            self.consume_char();
        }

        let text = &self.source[self.start..self.current + 1];
        match self.keywords.get(text) {
            Some(token_type) => self.add_token(token_type.clone(), Literal::None),
            None => self.add_token(TokenType::Identifier, Literal::String(text.to_string())),
        };
    }
    fn add_string(&mut self) {
        while self.cur_has_next_char()
            && (self.next_char().is_alphanumeric() || self.next_char() == '_')
        {
            self.consume_char();
        }
        if self.next_char() == '"' {
            self.consume_char();
        }
        self.add_token(
            TokenType::String,
            Literal::String(self.source[(self.start + 1)..self.current].to_string()),
        );
    }
    fn add_number(&mut self) {
        while self.cur_has_next_char() && self.next_char().is_ascii_digit() {
            self.consume_char();
        }

        if self.cur_has_next_char() && self.next_char() == '.' {
            self.consume_char();
        }
        while self.cur_has_next_char() && self.next_char().is_ascii_digit() {
            self.consume_char();
        }
        self.add_token(
            TokenType::Number,
            Literal::Number(
                self.source[self.start..(self.current + 1)]
                    .parse::<f64>()
                    .unwrap(),
            ),
        );
    }
    fn next_char_is_expected(&mut self, expected: char) -> bool {
        if !self.cur_has_next_char() {
            return false;
        }
        match self.next_char() == expected {
            true => {
                self.consume_char();
                true
            }
            false => false,
        }
    }
    fn consume_char(&mut self) {
        self.current += 1;
    }
    fn cur_has_next_char(&self) -> bool {
        self.current < (self.source.len() - 1)
    }
    fn cur_is_eof(&self) -> bool {
        self.current >= self.source.len()
    }
    fn next_char(&self) -> char {
        self.source.as_bytes()[self.current + 1] as char
    }
    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        self.tokens.push(Token {
            token_type: token_type,
            literal: literal,
            lexeme: self.source[self.start..(self.current + 1)].to_string(),
            line: self.line,
        });
    }
    fn cur_char(&self) -> char {
        self.source.as_bytes()[self.current] as char
    }
}
/// 测试模块：将文本转为对应的token序列值
/// 目标：
/// 1. 能够加载文本
/// 2. 能够准确识别所有token类型(标识符|关键字|字面量|运算符)
/// 3. 能够正确识别单char符号
/// 4. 能够正确识别双char符号
/// 5. 能够正确识别关键词

/// 3. 正确的处理位置信息（行号追踪）
/// 4. 完整覆盖边界情况（空输入|超长输入|混合空白输入)
#[cfg(test)]
mod test_scanners {
    use super::*;

    //能够正确识别单char符号
    #[test]
    fn test_single_left_paren() {
        let mut scanner = Scanner::new("(".to_string());
        scanner.scan_token();
        assert_eq!(
            scanner.tokens[0],
            Token {
                token_type: TokenType::LeftParen,
                literal: Literal::None,
                lexeme: "(".to_string(),
                line: 1
            }
        )
    }
    #[test]
    fn test_single_right_paren() {
        let mut scanner = Scanner::new(")".to_string());
        scanner.scan_token();
        assert_eq!(
            scanner.tokens[0],
            Token {
                token_type: TokenType::RightParen,
                literal: Literal::None,
                lexeme: ")".to_string(),
                line: 1
            }
        )
    }
    #[test]
    fn test_single_left_brace() {
        let mut scanner = Scanner::new("{".to_string());
        scanner.scan_token();
        assert_eq!(
            scanner.tokens[0],
            Token {
                token_type: TokenType::LeftBrace,
                literal: Literal::None,
                lexeme: "{".to_string(),
                line: 1
            }
        )
    }

    //能够正确识别单char或者双char符号
    #[test]
    fn test_bang_and_bang_equal() {
        let mut scanner = Scanner::new("!".to_string());
        scanner.scan_token();
        scanner.scan_token();
        assert_eq!(
            scanner.tokens[0],
            Token {
                token_type: TokenType::Bang,
                literal: Literal::None,
                lexeme: "!".to_string(),
                line: 1
            }
        );

        let mut scanner = Scanner::new("!=".to_string());
        scanner.scan_token();
        assert_eq!(
            scanner.tokens[0],
            Token {
                token_type: TokenType::BangEqual,
                literal: Literal::None,
                lexeme: "!=".to_string(),
                line: 1
            }
        );
    }

    #[test]
    fn test_equal_and_equal_equal() {
        let mut scanner = Scanner::new("=".to_string());
        scanner.scan_token();
        assert_eq!(
            scanner.tokens[0],
            Token {
                token_type: TokenType::Equal,
                literal: Literal::None,
                lexeme: "=".to_string(),
                line: 1
            }
        );

        let mut scanner = Scanner::new("==".to_string());
        scanner.scan_token();
        assert_eq!(
            scanner.tokens[0],
            Token {
                token_type: TokenType::EqualEqual,
                literal: Literal::None,
                lexeme: "==".to_string(),
                line: 1
            }
        );
    }
    #[test]
    fn test_greater_and_greater_equal() {
        let mut scanner = Scanner::new(">".to_string());
        scanner.scan_token();
        assert_eq!(
            scanner.tokens[0],
            Token {
                token_type: TokenType::Greater,
                literal: Literal::None,
                lexeme: ">".to_string(),
                line: 1
            }
        );

        let mut scanner = Scanner::new(">=".to_string());
        scanner.scan_token();
        assert_eq!(
            scanner.tokens[0],
            Token {
                token_type: TokenType::GreaterEqual,
                literal: Literal::None,
                lexeme: ">=".to_string(),
                line: 1
            }
        );
    }
    #[test]
    fn test_less_and_less_equal() {
        let mut scanner = Scanner::new("<".to_string());
        scanner.scan_token();
        assert_eq!(
            scanner.tokens[0],
            Token {
                token_type: TokenType::Less,
                literal: Literal::None,
                lexeme: "<".to_string(),
                line: 1
            }
        );

        let mut scanner = Scanner::new("<=".to_string());
        scanner.scan_token();
        assert_eq!(
            scanner.tokens[0],
            Token {
                token_type: TokenType::LessEqual,
                literal: Literal::None,
                lexeme: "<=".to_string(),
                line: 1
            }
        );
    }

    #[test]
    fn test_add_number() {
        let mut scanner = Scanner::new("123.456".to_string());
        scanner.scan_token();
        assert_eq!(
            scanner.tokens[0],
            Token {
                token_type: TokenType::Number,
                literal: Literal::Number(123.456),
                lexeme: "123.456".to_string(),
                line: 1
            }
        );

        let mut scanner = Scanner::new("123".to_string());
        scanner.scan_token();
        assert_eq!(
            scanner.tokens[0],
            Token {
                token_type: TokenType::Number,
                literal: Literal::Number(123.0),
                lexeme: "123".to_string(),
                line: 1
            }
        );

        let mut scanner = Scanner::new("  123.567  ".to_string());
        scanner.scan_token();
        scanner.scan_token();
        scanner.scan_token();
        assert_eq!(
            scanner.tokens[0],
            Token {
                token_type: TokenType::Number,
                literal: Literal::Number(123.567),
                lexeme: "123.567".to_string(),
                line: 1
            }
        );
    }

    #[test]
    fn test_add_string() {
        let mut scanner = Scanner::new("\"hello\"".to_string());
        scanner.scan_token();
        assert_eq!(
            scanner.tokens[0],
            Token {
                token_type: TokenType::String,
                literal: Literal::String("hello".to_string()),
                lexeme: "\"hello\"".to_string(),
                line: 1
            }
        );

        let mut scanner = Scanner::new("\"he_llo\"".to_string());
        scanner.scan_token();
        assert_eq!(
            scanner.tokens[0],
            Token {
                token_type: TokenType::String,
                literal: Literal::String("he_llo".to_string()),
                lexeme: "\"he_llo\"".to_string(),
                line: 1
            }
        );

        let mut scanner = Scanner::new("  \"hello\"".to_string());
        scanner.scan_token();
        scanner.scan_token();
        scanner.scan_token();
        assert_eq!(
            scanner.tokens[0],
            Token {
                token_type: TokenType::String,
                literal: Literal::String("hello".to_string()),
                lexeme: "\"hello\"".to_string(),
                line: 1
            }
        );

        let mut scanner = Scanner::new("  \"hello\" ".to_string());
        scanner.scan_token();
        scanner.scan_token();
        scanner.scan_token();
        assert_eq!(
            scanner.tokens[0],
            Token {
                token_type: TokenType::String,
                literal: Literal::String("hello".to_string()),
                lexeme: "\"hello\"".to_string(),
                line: 1
            }
        );
    }

    #[test]
    fn test_identifier() {
        let mut scanner = Scanner::new("hell_o".to_string());
        scanner.scan_token();
        assert_eq!(
            scanner.tokens[0],
            Token {
                token_type: TokenType::Identifier,
                literal: Literal::String("hell_o".to_string()),
                lexeme: "hell_o".to_string(),
                line: 1
            }
        );
    }

    #[test]
    fn test_keyword() {
        let mut scanner = Scanner::new("fun".to_string());
        scanner.scan_token();
        assert_eq!(
            scanner.tokens[0],
            Token {
                token_type: TokenType::Fun,
                literal: Literal::None,
                lexeme: "fun".to_string(),
                line: 1
            }
        );
    }    
}
