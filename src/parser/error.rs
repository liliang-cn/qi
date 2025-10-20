//! Parsing error handling for Qi language

use crate::lexer::Span;

/// Parsing errors
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    /// Unexpected token
    #[error("意外的标记: {0:?} 在第 {1} 行第 {2} 列")]
    UnexpectedToken(crate::lexer::TokenKind, usize, usize),

    /// Expected token
    #[error("期望标记 {0:?} 但找到 {1:?} 在第 {2} 行第 {3} 列")]
    ExpectedToken(crate::lexer::TokenKind, crate::lexer::TokenKind, usize, usize),

    /// Invalid syntax
    #[error("语法错误: {0} 在第 {1} 行第 {2} 列")]
    InvalidSyntax(String, usize, usize),

    /// Unterminated expression
    #[error("未终止的表达式在第 {0} 行第 {1} 列")]
    UnterminatedExpression(usize, usize),

    /// Invalid function declaration
    #[error("无效的函数声明在第 {0} 行第 {1} 列")]
    InvalidFunctionDeclaration(usize, usize),

    /// Invalid variable declaration
    #[error("无效的变量声明在第 {0} 行第 {1} 列")]
    InvalidVariableDeclaration(usize, usize),

    /// Unexpected end of input
    #[error("意外的输入结束")]
    UnexpectedEof,

    /// Generic parsing error
    #[error("解析错误: {0}")]
    General(String),

    /// Parse failed
    #[error("解析失败")]
    ParseFailed,
}

impl ParseError {
    pub fn with_span(mut self, span: Span, source: &str) -> Self {
        // Calculate line and column from span
        let (line, column) = self.calculate_position(span.start, source);
        match self {
            ParseError::UnexpectedToken(_, ref mut l, ref mut c) => {
                *l = line;
                *c = column;
            }
            ParseError::ExpectedToken(_, _, ref mut l, ref mut c) => {
                *l = line;
                *c = column;
            }
            ParseError::InvalidSyntax(_, ref mut l, ref mut c) => {
                *l = line;
                *c = column;
            }
            ParseError::UnterminatedExpression(ref mut l, ref mut c) => {
                *l = line;
                *c = column;
            }
            ParseError::InvalidFunctionDeclaration(ref mut l, ref mut c) => {
                *l = line;
                *c = column;
            }
            ParseError::InvalidVariableDeclaration(ref mut l, ref mut c) => {
                *l = line;
                *c = column;
            }
            _ => {}
        }
        self
    }

    fn calculate_position(&self, offset: usize, source: &str) -> (usize, usize) {
        let mut line = 1;
        let mut column = 1;

        for (i, c) in source.chars().enumerate() {
            if i == offset {
                break;
            }
            if c == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }

        (line, column)
    }
}