//! Unicode-aware lexical analysis for Qi language

pub mod keywords;
pub mod tokens;
pub mod unicode;

pub use tokens::{Token, TokenKind, Span};
pub use unicode::UnicodeHandler;


/// Qi language lexical analyzer
pub struct Lexer {
    source: String,
    position: usize,
    line: usize,
    column: usize,
    unicode_handler: UnicodeHandler,
}

impl Lexer {
    /// Create a new lexer for the given source code
    pub fn new(source: String) -> Self {
        Self {
            source,
            position: 0,
            line: 1,
            column: 1,
            unicode_handler: UnicodeHandler::new(),
        }
    }

    /// Tokenize the entire source code
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexicalError> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            // Skip whitespace
            self.skip_whitespace();

            if self.is_at_end() {
                break;
            }

            let token = self.next_token()?;
            tokens.push(token);
        }

        // Add EOF token
        tokens.push(Token {
            kind: TokenKind::文件结束,
            text: String::new(),
            span: tokens::Span::new(self.position, self.position),
            line: self.line,
            column: self.column,
        });

        Ok(tokens)
    }

    /// Get the next token
    fn next_token(&mut self) -> Result<Token, LexicalError> {
        let start_pos = self.position;
        let start_line = self.line;
        let start_column = self.column;

        let c = self.current_char().ok_or(LexicalError::UnexpectedEof)?;

        match c {
            // Single-character tokens
            '(' => Ok(self.make_single_char_token(TokenKind::左括号, start_pos, start_line, start_column)),
            ')' => Ok(self.make_single_char_token(TokenKind::右括号, start_pos, start_line, start_column)),
            '[' => Ok(self.make_single_char_token(TokenKind::左方括号, start_pos, start_line, start_column)),
            ']' => Ok(self.make_single_char_token(TokenKind::右方括号, start_pos, start_line, start_column)),
            '{' => Ok(self.make_single_char_token(TokenKind::左大括号, start_pos, start_line, start_column)),
            '}' => Ok(self.make_single_char_token(TokenKind::右大括号, start_pos, start_line, start_column)),
            ';' => Ok(self.make_single_char_token(TokenKind::分号, start_pos, start_line, start_column)),
            ',' => Ok(self.make_single_char_token(TokenKind::逗号, start_pos, start_line, start_column)),
            ':' => Ok(self.make_single_char_token(TokenKind::冒号, start_pos, start_line, start_column)),
            '.' => Ok(self.make_single_char_token(TokenKind::点, start_pos, start_line, start_column)),

            // Operators and comments
            '+' => Ok(self.make_single_char_token(TokenKind::加, start_pos, start_line, start_column)),
            '*' => Ok(self.make_single_char_token(TokenKind::乘, start_pos, start_line, start_column)),
            '/' => {
                if self.peek_char() == Some('/') {
                    // Check if it's a doc comment (///)
                    if self.peek_char_at_offset(2) == Some('/') {
                        // Doc line comment - skip and return next token
                        self.skip_line_comment();
                        return self.next_token();
                    } else {
                        // Regular line comment - skip to end of line and return next token
                        self.skip_line_comment();
                        return self.next_token();
                    }
                } else if self.peek_char() == Some('*') {
                    // Check if it's a doc block comment (/**)
                    if self.peek_char_at_offset(2) == Some('*') {
                        // Doc block comment - skip and return next token
                        self.skip_doc_block_comment();
                        return self.next_token();
                    } else {
                        // Regular block comment - skip to closing */
                        self.skip_block_comment();
                        return self.next_token();
                    }
                } else {
                    Ok(self.make_single_char_token(TokenKind::除, start_pos, start_line, start_column))
                }
            }

            // Assignment and comparison operators
            '=' => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    Ok(self.make_two_char_token(TokenKind::等于, start_pos, start_line, start_column))
                } else {
                    Ok(self.make_single_char_token(TokenKind::赋值, start_pos, start_line, start_column))
                }
            }
            '-' => {
                if self.peek_char() == Some('>') {
                    self.advance();
                    Ok(self.make_two_char_token(TokenKind::箭头, start_pos, start_line, start_column))
                } else {
                    Ok(self.make_single_char_token(TokenKind::减, start_pos, start_line, start_column))
                }
            }
            '!' => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    Ok(self.make_two_char_token(TokenKind::不等于, start_pos, start_line, start_column))
                } else {
                    Err(LexicalError::InvalidCharacter(c, start_line, start_column))
                }
            }
            '<' => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    Ok(self.make_two_char_token(TokenKind::小于等于, start_pos, start_line, start_column))
                } else {
                    Ok(self.make_single_char_token(TokenKind::小于, start_pos, start_line, start_column))
                }
            }
            '>' => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    Ok(self.make_two_char_token(TokenKind::大于等于, start_pos, start_line, start_column))
                } else {
                    Ok(self.make_single_char_token(TokenKind::大于, start_pos, start_line, start_column))
                }
            }

            // Character literals
            '\'' => self.scan_char_literal(start_pos, start_line, start_column),

            // String literals
            '"' => self.scan_string_literal(start_pos, start_line, start_column),

            // Numbers
            '0'..='9' => self.scan_number(start_pos, start_line, start_column),

            // Identifiers and keywords
            c if c.is_alphabetic() || c == '_' => {
                // Handle Chinese characters and standard identifiers
                if self.unicode_handler.is_chinese_char(c) {
                    Ok(self.scan_chinese_identifier(start_pos, start_line, start_column))
                } else {
                    Ok(self.scan_identifier(start_pos, start_line, start_column))
                }
            }

            // Chinese punctuation (treat as whitespace/end of statements)
            c if "，。！？；：".contains(c) ||
               c == '"' || c == '"' ||
               c == '（' || c == '）' || c == '【' || c == '】' ||
               c == '《' || c == '》' => {
                self.advance();
                return self.next_token(); // Skip Chinese punctuation
            }

            // Whitespace (should be skipped by skip_whitespace, but handle anyway)
            c if c.is_whitespace() => {
                self.advance();
                return self.next_token(); // Skip and get next token
            }

            _ => Err(LexicalError::InvalidCharacter(c, start_line, start_column)),
        }
    }

    /// Create a single character token
    fn make_single_char_token(&mut self, kind: TokenKind, start_pos: usize, start_line: usize, start_column: usize) -> Token {
        let text = self.current_char().unwrap().to_string();
        self.advance();
        Token {
            kind,
            text,
            span: tokens::Span::new(start_pos, self.position),
            line: start_line,
            column: start_column,
        }
    }

    /// Create a two character token
    fn make_two_char_token(&mut self, kind: TokenKind, start_pos: usize, start_line: usize, start_column: usize) -> Token {
        self.advance(); // Advance to include the second character
        Token {
            kind,
            text: self.source[start_pos..self.position].to_string(),
            span: tokens::Span::new(start_pos, self.position),
            line: start_line,
            column: start_column,
        }
    }

    /// Scan a string literal
    fn scan_string_literal(&mut self, start_pos: usize, start_line: usize, start_column: usize) -> Result<Token, LexicalError> {
        self.advance(); // Skip opening quote
        let start_content = self.position;

        while !self.is_at_end() && self.current_char() != Some('"') {
            if self.current_char() == Some('\\') {
                self.advance(); // Skip escape character
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(LexicalError::UnterminatedString(start_line, start_column));
        }

        let end_content = self.position;
        self.advance(); // Skip closing quote
        let end_pos = self.position;

        let content = self.source[start_content..end_content].to_string();

        Ok(Token {
            kind: TokenKind::字符串字面量(content),
            text: self.source[start_pos..end_pos].to_string(),
            span: tokens::Span::new(start_pos, end_pos),
            line: start_line,
            column: start_column,
        })
    }

    /// Scan a character literal
    fn scan_char_literal(&mut self, start_pos: usize, start_line: usize, start_column: usize) -> Result<Token, LexicalError> {
        self.advance(); // Skip opening quote

        if self.is_at_end() {
            return Err(LexicalError::UnterminatedString(start_line, start_column));
        }

        let char_content = self.current_char().unwrap();

        // Handle escape sequences
        let final_char = if char_content == '\\' {
            self.advance(); // Skip escape character
            if self.is_at_end() {
                return Err(LexicalError::UnterminatedString(start_line, start_column));
            }

            let escaped_char = self.current_char().unwrap();
            match escaped_char {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                '\\' => '\\',
                '\'' => '\'',
                '"' => '"',
                _ => escaped_char, // For other escape sequences, just use the character as-is
            }
        } else {
            char_content
        };

        self.advance(); // Skip the character

        // Expect closing quote
        if self.current_char() != Some('\'') {
            return Err(LexicalError::UnterminatedString(start_line, start_column));
        }

        self.advance(); // Skip closing quote
        let end_pos = self.position;

        Ok(Token {
            kind: TokenKind::字符字面量(final_char),
            text: self.source[start_pos..end_pos].to_string(),
            span: tokens::Span::new(start_pos, end_pos),
            line: start_line,
            column: start_column,
        })
    }

    /// Scan a number (integer or float)
    fn scan_number(&mut self, start_pos: usize, start_line: usize, start_column: usize) -> Result<Token, LexicalError> {
        while !self.is_at_end() && self.current_char().unwrap().is_ascii_digit() {
            self.advance();
        }

        // Check for float
        if self.current_char() == Some('.') {
            self.advance();
            while !self.is_at_end() && self.current_char().unwrap().is_ascii_digit() {
                self.advance();
            }

            let number_str = &self.source[start_pos..self.position];
            let value = number_str.parse::<f64>()
                .map_err(|_| LexicalError::InvalidNumber(start_line, start_column))?;

            Ok(Token {
                kind: TokenKind::浮点数字面量(value),
                text: number_str.to_string(),
                span: tokens::Span::new(start_pos, self.position),
                line: start_line,
                column: start_column,
            })
        } else {
            let number_str = &self.source[start_pos..self.position];
            let value = number_str.parse::<i64>()
                .map_err(|_| LexicalError::InvalidNumber(start_line, start_column))?;

            Ok(Token {
                kind: TokenKind::整数字面量(value),
                text: number_str.to_string(),
                span: tokens::Span::new(start_pos, self.position),
                line: start_line,
                column: start_column,
            })
        }
    }

    /// Scan a standard identifier
    fn scan_identifier(&mut self, start_pos: usize, start_line: usize, start_column: usize) -> Token {
        while !self.is_at_end() {
            let c = self.current_char().unwrap();
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let text = &self.source[start_pos..self.position];

        // Check if it's a keyword
        let kind = keywords::KEYWORDS.lookup(text)
            .unwrap_or(TokenKind::标识符(text.to_string()));

        Token {
            kind,
            text: text.to_string(),
            span: tokens::Span::new(start_pos, self.position),
            line: start_line,
            column: start_column,
        }
    }

    /// Scan a Chinese identifier or keyword
    fn scan_chinese_identifier(&mut self, start_pos: usize, start_line: usize, start_column: usize) -> Token {
        while !self.is_at_end() {
            let c = self.current_char().unwrap();
            if self.unicode_handler.is_chinese_char(c) || c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let text = &self.source[start_pos..self.position];

        // Check if it's a Chinese keyword
        let kind = keywords::KEYWORDS.lookup(text)
            .unwrap_or(TokenKind::标识符(text.to_string()));

        Token {
            kind,
            text: text.to_string(),
            span: tokens::Span::new(start_pos, self.position),
            line: start_line,
            column: start_column,
        }
    }

    /// Advance to the next character
    fn advance(&mut self) {
        if self.current_char() == Some('\n') {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += self.unicode_handler.char_width(self.current_char().unwrap_or('\0'));
        }
        self.position += self.current_char().unwrap_or('\0').len_utf8();
    }

    /// Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.current_char().unwrap().is_whitespace() {
            if self.current_char() == Some('\n') {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += self.unicode_handler.char_width(self.current_char().unwrap_or('\0'));
            }
            self.position += self.current_char().unwrap_or('\0').len_utf8();
        }
    }

    /// Check if we're at the end of the source
    fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }

    /// Get the current character
    fn current_char(&self) -> Option<char> {
        self.source[self.position..].chars().next()
    }

    /// Look ahead at the next character
    fn peek_char(&self) -> Option<char> {
        // Get current character
        let current_char = self.current_char()?;

        // Get current character length in bytes
        let char_len = current_char.len_utf8();

        // Look ahead from the position after current character
        self.source[self.position + char_len..].chars().next()
    }

    /// Look ahead at character at specific offset (character-based)
    fn peek_char_at_offset(&self, offset: usize) -> Option<char> {
        if offset == 0 {
            return self.current_char();
        }

        let mut chars = self.source[self.position..].chars();
        for _ in 0..offset {
            if chars.next().is_none() {
                return None;
            }
        }
        chars.next()
    }

    /// Skip line comment (// to end of line)
    fn skip_line_comment(&mut self) {
        // Skip both slashes
        self.advance(); // skip first '/'
        self.advance(); // skip second '/'

        // Skip until end of line or file
        while !self.is_at_end() && self.current_char() != Some('\n') {
            self.advance();
        }
    }

    /// Skip block comment (/* ... */)
    fn skip_block_comment(&mut self) {
        // Skip opening /*
        self.advance(); // skip '/'
        self.advance(); // skip '*'

        let mut depth = 1;

        while !self.is_at_end() && depth > 0 {
            match (self.current_char(), self.peek_char()) {
                (Some('/'), Some('*')) => {
                    // Found nested block comment start
                    self.advance(); // skip '/'
                    self.advance(); // skip '*'
                    depth += 1;
                }
                (Some('*'), Some('/')) => {
                    // Found block comment end
                    self.advance(); // skip '*'
                    self.advance(); // skip '/'
                    depth -= 1;
                }
                (Some('\n'), _) => {
                    // Handle line breaks properly for line counting
                    self.advance();
                }
                _ => {
                    self.advance();
                }
            }
        }

        if depth > 0 {
            // Unterminated block comment - this would be an error in a real implementation
            // For now, we'll just skip to end of file
        }
    }

    /// Skip doc block comment (/** ... */)
    fn skip_doc_block_comment(&mut self) {
        // Skip opening /**
        self.advance(); // skip '/'
        self.advance(); // skip '*'
        self.advance(); // skip '*'

        let mut depth = 1;

        while !self.is_at_end() && depth > 0 {
            match (self.current_char(), self.peek_char()) {
                (Some('/'), Some('*')) => {
                    // Found nested block comment start
                    self.advance(); // skip '/'
                    self.advance(); // skip '*'
                    depth += 1;
                }
                (Some('*'), Some('/')) => {
                    // Found block comment end
                    self.advance(); // skip '*'
                    self.advance(); // skip '/'
                    depth -= 1;
                }
                (Some('\n'), _) => {
                    // Handle line breaks properly for line counting
                    self.advance();
                }
                _ => {
                    self.advance();
                }
            }
        }

        if depth > 0 {
            // Unterminated doc block comment - this would be an error in a real implementation
            // For now, we'll just skip to end of file
        }
    }
}

/// Lexical analysis errors
#[derive(Debug, thiserror::Error)]
pub enum LexicalError {
    /// Invalid character
    #[error("无效字符: '{0}' 在第 {1} 行第 {2} 列")]
    InvalidCharacter(char, usize, usize),

    /// Unterminated string literal
    #[error("未终止的字符串字面量在第 {0} 行第 {1} 列")]
    UnterminatedString(usize, usize),

    /// Invalid number format
    #[error("无效的数字格式在第 {0} 行第 {1} 列")]
    InvalidNumber(usize, usize),

    /// Unexpected end of file
    #[error("意外的文件结束")]
    UnexpectedEof,
}