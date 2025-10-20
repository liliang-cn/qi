//! User Story 2 Test Suite: Data Type and Variable Handling
//! Tests for variable declarations, type safety, and string handling

use qi_compiler::lexer::Lexer;
use qi_compiler::parser::Parser;
use qi_compiler::parser::ast::AstNode;
use qi_compiler::semantic::type_checker::{TypeChecker, TypeError};

/// Test variable declarations with type annotations
#[test]
fn test_variable_declaration_with_type_annotation() {
    let source_code = r#"
变量 x: 整数 = 42;
变量 message: 字符串 = "Hello";
变量 is_valid: 布尔 = 真;
变量 pi: 浮点数 = 3.14159;
"#;

    let mut lexer = Lexer::new(source_code.to_string());
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    let mut parser = Parser::new();
    let ast = parser.parse(tokens).expect("Parsing should succeed");

    let mut type_checker = TypeChecker::new();
    let program_ast = AstNode::程序(ast);
    let result = type_checker.check(&program_ast);

    assert!(result.is_ok(), "Type checking should succeed");
    assert!(type_checker.get_errors().is_empty(), "Should have no type errors");
}

/// Test variable declaration without type annotation (type inference)
#[test]
fn test_variable_declaration_type_inference() {
    let source_code = r#"
变量 x = 42;        // Should infer as 整数
变量 msg = "hello"; // Should infer as 字符串
变量 flag = 假;     // Should infer as 布尔
"#;

    let mut lexer = Lexer::new(source_code.to_string());
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    let mut parser = Parser::new();
    let ast = parser.parse(tokens).expect("Parsing should succeed");

    let mut type_checker = TypeChecker::new();
    let program_ast = AstNode::程序(ast);
    let result = type_checker.check(&program_ast);

    assert!(result.is_ok(), "Type checking should succeed");
    assert!(type_checker.get_errors().is_empty(), "Should have no type errors");
}

/// Test type mismatch errors
#[test]
fn test_type_mismatch_errors() {
    let source_code = r#"
变量 x: 整数 = "hello";  // Type mismatch: integer expected, string provided
变量 valid: 布尔 = 42;    // Type mismatch: boolean expected, integer provided
"#;

    let mut lexer = Lexer::new(source_code.to_string());
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    let mut parser = Parser::new();
    let ast = parser.parse(tokens).expect("Parsing should succeed");

    let mut type_checker = TypeChecker::new();
    let program_ast = AstNode::程序(ast);
    let _result = type_checker.check(&program_ast);

    let errors = type_checker.get_errors();
    assert!(!errors.is_empty(), "Should have type errors");

    // Check for type mismatch errors
    let has_type_mismatch = errors.iter().any(|e| {
        matches!(e, TypeError::TypeMismatch { .. })
    });
    assert!(has_type_mismatch, "Should have type mismatch errors");
}

/// Test undefined variable errors
#[test]
fn test_undefined_variable_errors() {
    let source_code = r#"
变量 x = y + 1;  // y is undefined
"#;

    let mut lexer = Lexer::new(source_code.to_string());
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    let mut parser = Parser::new();
    let ast = parser.parse(tokens).expect("Parsing should succeed");

    let mut type_checker = TypeChecker::new();
    let program_ast = AstNode::程序(ast);
    let _result = type_checker.check(&program_ast);

    let errors = type_checker.get_errors();
    assert!(!errors.is_empty(), "Should have type errors");

    // Check for undefined variable errors
    let has_undefined_var = errors.iter().any(|e| {
        matches!(e, TypeError::UndefinedVariable { .. })
    });
    assert!(has_undefined_var, "Should have undefined variable errors");
}

/// Test string type handling and concatenation
#[test]
fn test_string_type_handling() {
    let source_code = r#"
变量 greeting: 字符串 = "Hello";
变量 name: 字符串 = "World";
变量 message: 字符串 = greeting + ", " + name + "!";
"#;

    let mut lexer = Lexer::new(source_code.to_string());
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    let mut parser = Parser::new();
    let ast = parser.parse(tokens).expect("Parsing should succeed");

    let mut type_checker = TypeChecker::new();
    let program_ast = AstNode::程序(ast);
    let result = type_checker.check(&program_ast);

    assert!(result.is_ok(), "Type checking should succeed");
    assert!(type_checker.get_errors().is_empty(), "Should have no type errors");
}

/// Test string concatenation with non-string operands
#[test]
fn test_string_concatenation_type_errors() {
    let source_code = r#"
变量 message: 字符串 = "Hello" + 42;  // Can't concatenate string with integer
"#;

    let mut lexer = Lexer::new(source_code.to_string());
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    let mut parser = Parser::new();
    let ast = parser.parse(tokens).expect("Parsing should succeed");

    let mut type_checker = TypeChecker::new();
    let program_ast = AstNode::程序(ast);
    let _result = type_checker.check(&program_ast);

    let errors = type_checker.get_errors();
    assert!(!errors.is_empty(), "Should have type errors");

    // Check for invalid operation errors
    let has_invalid_op = errors.iter().any(|e| {
        matches!(e, TypeError::InvalidOperation { .. })
    });
    assert!(has_invalid_op, "Should have invalid operation errors");
}

/// Test type annotation validation in function parameters
#[test]
fn test_function_parameter_type_annotations() {
    let source_code = r#"
函数 add(a: 整数, b: 整数): 整数 {
    变量 result: 整数 = a + b;
    返回 result;
}

函数 greet(name: 字符串): 字符串 {
    变量 message: 字符串 = "Hello, " + name;
    返回 message;
}
"#;

    let mut lexer = Lexer::new(source_code.to_string());
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    let mut parser = Parser::new();
    let ast = parser.parse(tokens).expect("Parsing should succeed");

    let mut type_checker = TypeChecker::new();
    let program_ast = AstNode::程序(ast);
    let result = type_checker.check(&program_ast);

    assert!(result.is_ok(), "Type checking should succeed");
    assert!(type_checker.get_errors().is_empty(), "Should have no type errors");
}

/// Test variable mutability
#[test]
fn test_variable_mutability() {
    let source_code = r#"
变量 x = 10;
变量 y = 20;
x = 15;  // Should work (reassignment)
"#;

    let mut lexer = Lexer::new(source_code.to_string());
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    let mut parser = Parser::new();
    let ast = parser.parse(tokens).expect("Parsing should succeed");

    let mut type_checker = TypeChecker::new();
    let program_ast = AstNode::程序(ast);
    let result = type_checker.check(&program_ast);

    assert!(result.is_ok(), "Type checking should succeed");
    assert!(type_checker.get_errors().is_empty(), "Should have no type errors");
}

/// Test scope-based type checking
#[test]
fn test_scope_based_type_checking() {
    let source_code = r#"
变量 x: 整数 = 10;

函数 test() {
    变量 x: 字符串 = "local";  // Different type in local scope
    变量 y: 布尔 = 真;
    返回 y;
}

变量 z: 浮点数 = 3.14;
"#;

    let mut lexer = Lexer::new(source_code.to_string());
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    let mut parser = Parser::new();
    let ast = parser.parse(tokens).expect("Parsing should succeed");

    let mut type_checker = TypeChecker::new();
    let program_ast = AstNode::程序(ast);
    let result = type_checker.check(&program_ast);

    assert!(result.is_ok(), "Type checking should succeed");
    assert!(type_checker.get_errors().is_empty(), "Should have no type errors");
}

/// Test array type handling
#[test]
fn test_array_type_handling() {
    let source_code = r#"
变量 numbers: 数组<整数> = [1, 2, 3, 4, 5];
变量 names: 数组<字符串> = ["Alice", "Bob", "Charlie"];
变量 first_num: 整数 = numbers[0];
变量 first_name: 字符串 = names[0];
"#;

    let mut lexer = Lexer::new(source_code.to_string());
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    let mut parser = Parser::new();
    let ast = parser.parse(tokens).expect("Parsing should succeed");

    let mut type_checker = TypeChecker::new();
    let program_ast = AstNode::程序(ast);
    let result = type_checker.check(&program_ast);

    assert!(result.is_ok(), "Type checking should succeed");
    assert!(type_checker.get_errors().is_empty(), "Should have no type errors");
}

/// Test array type errors
#[test]
fn test_array_type_errors() {
    let source_code = r#"
变量 mixed = [1, "hello", 真];  // Mixed types in array
变量 invalid_index: 字符串 = numbers[0];  // Wrong index type
"#;

    let mut lexer = Lexer::new(source_code.to_string());
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    let mut parser = Parser::new();
    let ast = parser.parse(tokens).expect("Parsing should succeed");

    let mut type_checker = TypeChecker::new();
    let program_ast = AstNode::程序(ast);
    let _result = type_checker.check(&program_ast);

    let errors = type_checker.get_errors();
    assert!(!errors.is_empty(), "Should have type errors");
}

/// Test Chinese identifiers with type annotations
#[test]
fn test_chinese_identifiers_with_type_annotations() {
    let source_code = r#"
变量 数字: 整数 = 42;
变量 文本: 字符串 = "你好世界";
变量 标志: 布尔 = 真;
变量 浮点: 浮点数 = 3.14159;

函数 计算(值1: 整数, 值2: 整数): 整数 {
    变量 结果: 整数 = 值1 + 值2;
    返回 结果;
}

变量 最终结果: 整数 = 计算(数字, 10);
"#;

    let mut lexer = Lexer::new(source_code.to_string());
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    let mut parser = Parser::new();
    let ast = parser.parse(tokens).expect("Parsing should succeed");

    let mut type_checker = TypeChecker::new();
    let program_ast = AstNode::程序(ast);
    let result = type_checker.check(&program_ast);

    assert!(result.is_ok(), "Type checking should succeed");
    assert!(type_checker.get_errors().is_empty(), "Should have no type errors");
}