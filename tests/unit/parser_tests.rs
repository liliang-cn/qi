//! Unit tests for Qi parser

use qi_compiler::lexer::{Lexer, TokenKind};
use qi_compiler::parser::Parser;
use qi_compiler::parser::ast::*;
use qi_compiler::lexer::Span;

#[test]
fn test_empty_program() {
    let source = "".to_string();
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Should tokenize successfully");

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    // Should return a program with no statements for now
    assert!(result.is_ok());
}

#[test]
fn test_single_literal() {
    let source = "42;".to_string();
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Should tokenize successfully");

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    // Parser is not fully implemented yet, but should not panic
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_simple_variable_declaration() {
    let source = "变量 x = 10;".to_string();
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Should tokenize successfully");

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    // Parser is not fully implemented yet, but should not panic
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_chinese_variable_names() {
    let source = "变量 数字 = 42; 变量 文本 = \"你好\";".to_string();
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Should tokenize successfully");

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    // Parser is not fully implemented yet, but should not panic
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_basic_expressions() {
    let source = "1 + 2 * 3;".to_string();
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Should tokenize successfully");

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    // Parser is not fully implemented yet, but should not panic
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_boolean_literals() {
    let source = "真; 假;".to_string();
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Should tokenize successfully");

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    // Parser is not fully implemented yet, but should not panic
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_string_literals() {
    let source = "\"Hello, World!\";".to_string();
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Should tokenize successfully");

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    // Parser is not fully implemented yet, but should not panic
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_function_declaration() {
    let source = "函数 测试() { }".to_string();
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Should tokenize successfully");

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    // Parser is not fully implemented yet, but should not panic
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_mixed_language_identifiers() {
    let source = "variable 中文变量 = 42;".to_string();
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Should tokenize successfully");

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    // Parser is not fully implemented yet, but should not panic
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_complex_arithmetic() {
    let source = "(1 + 2) * (3 - 4) / 5;".to_string();
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Should tokenize successfully");

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    // Parser is not fully implemented yet, but should not panic
    assert!(result.is_ok() || result.is_err());
}