//! Unit tests for Qi lexer

use qi_compiler::lexer::{Lexer, LexicalError};
use qi_compiler::lexer::TokenKind;

#[test]
fn test_chinese_keywords() {
    let source = "如果 否则 当 对于 函数 返回 变量 常量 整数 字符串 布尔 浮点数".to_string();
    let mut lexer = Lexer::new(source);

    let tokens = lexer.tokenize().expect("Should tokenize successfully");

    let expected_keywords = vec![
        TokenKind::如果,
        TokenKind::否则,
        TokenKind::当,
        TokenKind::对于,
        TokenKind::函数,
        TokenKind::返回,
        TokenKind::变量,
        TokenKind::常量,
        TokenKind::整数,
        TokenKind::字符串,
        TokenKind::布尔,
        TokenKind::浮点数,
        TokenKind::文件结束,
    ];

    assert_eq!(tokens.len(), expected_keywords.len());

    for (token, expected) in tokens.iter().zip(expected_keywords.iter()) {
        assert_eq!(token.kind, *expected);
    }
}

#[test]
fn test_chinese_identifiers() {
    let source = "变量名 函数名 用户标识符".to_string();
    let mut lexer = Lexer::new(source);

    let tokens = lexer.tokenize().expect("Should tokenize successfully");

    // All Chinese characters should be recognized as identifiers
    for token in &tokens[..tokens.len()-1] { // Skip EOF token
        match &token.kind {
            TokenKind::标识符(_) => {},
            _ => panic!("Expected identifier token, got {:?}", token.kind),
        }
    }
}

#[test]
fn test_mixed_chinese_english() {
    let source = "变量 myVar = 42;".to_string();
    let mut lexer = Lexer::new(source);

    let tokens = lexer.tokenize().expect("Should tokenize successfully");

    let expected = vec![
        TokenKind::变量,
        TokenKind::标识符("myVar".to_string()),
        TokenKind::赋值,
        TokenKind::整数字面量(42),
        TokenKind::分号,
        TokenKind::文件结束,
    ];

    assert_eq!(tokens.len(), expected.len());

    for (token, expected) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.kind, *expected);
    }
}

#[test]
fn test_string_literals() {
    let source = r#"变量 消息 = "你好，世界！";"#.to_string();
    let mut lexer = Lexer::new(source);

    let tokens = lexer.tokenize().expect("Should tokenize successfully");

    assert_eq!(tokens[0].kind, TokenKind::变量);
    assert_eq!(tokens[1].kind, TokenKind::标识符("消息".to_string()));
    assert_eq!(tokens[2].kind, TokenKind::赋值);

    if let TokenKind::字符串字面量(content) = &tokens[3].kind {
        assert_eq!(content, "你好，世界！");
    } else {
        panic!("Expected string literal");
    }
}

#[test]
fn test_numeric_literals() {
    let source = "变量 整数 = 42; 变量 浮点数 = 3.14;".to_string();
    let mut lexer = Lexer::new(source);

    let tokens = lexer.tokenize().expect("Should tokenize successfully");

    assert_eq!(tokens[3].kind, TokenKind::整数字面量(42));
    assert_eq!(tokens[9].kind, TokenKind::浮点数字面量(3.14));
}

#[test]
fn test_boolean_literals() {
    let source = "变量 真 = 真; 变量 假 = 假;".to_string();
    let mut lexer = Lexer::new(source);

    let tokens = lexer.tokenize().expect("Should tokenize successfully");

    assert_eq!(tokens[3].kind, TokenKind::布尔字面量(true));
    assert_eq!(tokens[9].kind, TokenKind::布尔字面量(false));
}

#[test]
fn test_operators() {
    let source = "+ - * / == != < <= > >=".to_string();
    let mut lexer = Lexer::new(source);

    let tokens = lexer.tokenize().expect("Should tokenize successfully");

    let expected_operators = vec![
        TokenKind::加,
        TokenKind::减,
        TokenKind::乘,
        TokenKind::除,
        TokenKind::等于,
        TokenKind::不等于,
        TokenKind::小于,
        TokenKind::小于等于,
        TokenKind::大于,
        TokenKind::大于等于,
    ];

    for (token, expected) in tokens.iter().zip(expected_operators.iter()) {
        assert_eq!(token.kind, *expected);
    }
}

#[test]
fn test_empty_input() {
    let source = "".to_string();
    let mut lexer = Lexer::new(source);

    let tokens = lexer.tokenize().expect("Should tokenize successfully");

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::文件结束);
}

#[test]
fn test_whitespace_handling() {
    let source = "   \t\n  变量 x = 1;  \n\t".to_string();
    let mut lexer = Lexer::new(source);

    let tokens = lexer.tokenize().expect("Should tokenize successfully");

    // Should have: 变量, x, =, 1, ;, 文件结束
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].kind, TokenKind::变量);
    assert_eq!(tokens[1].kind, TokenKind::标识符("x".to_string()));
}

#[test]
fn test_invalid_character() {
    let source = "变量 x = @;".to_string();
    let mut lexer = Lexer::new(source);

    let result = lexer.tokenize();
    assert!(result.is_err());

    if let Err(LexicalError::InvalidCharacter(char, line, col)) = result {
        assert_eq!(char, '@');
        assert_eq!(line, 1);
        assert_eq!(col, 11);
    } else {
        panic!("Expected InvalidCharacter error");
    }
}

#[test]
fn test_unterminated_string() {
    let source = r#"变量 消息 = "未终止字符串"#.to_string();
    let mut lexer = Lexer::new(source);

    let result = lexer.tokenize();
    assert!(result.is_err());

    if let Err(LexicalError::UnterminatedString(line, col)) = result {
        assert_eq!(line, 1);
        assert_eq!(col, 7);
    } else {
        panic!("Expected UnterminatedString error");
    }
}