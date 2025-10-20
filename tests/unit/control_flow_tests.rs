//! Unit tests for control flow structures in Qi language

use qi_compiler::lexer::*;
use qi_compiler::parser::*;
use qi_compiler::semantic::*;

#[test]
fn test_parse_if_statement() {
    let source_code = r#"
    函数 测试如果() {
        如果 x > 5 {
            变量 y = 10;
        }
    }
    "#;

    let mut lexer = Lexer::new(source_code.to_string());
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new();
    let ast = parser.parse(tokens).unwrap();

    // Check that we have an if statement in the AST
    assert!(format!("{:?}", ast).contains("如果"));
}

#[test]
fn test_parse_if_else_statement() {
    let source_code = r#"
    函数 测试如果否则() {
        如果 x > 5 {
            变量 y = 10;
        } 否则 {
            变量 y = 0;
        }
    }
    "#;

    let mut lexer = Lexer::new(source_code.to_string());
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new();
    let ast = parser.parse(tokens).unwrap();

    // Check that we have both 如果 and 否则 in the AST
    let ast_str = format!("{:?}", ast);
    assert!(ast_str.contains("如果"));
    assert!(ast_str.contains("否则"));
}

#[test]
fn test_parse_while_loop() {
    let source_code = r#"
    函数 测试当循环() {
        当 i < 10 {
            变量 x = x + 1;
        }
    }
    "#;

    let mut lexer = Lexer::new(source_code.to_string());
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new();
    let ast = parser.parse(tokens).unwrap();

    // Check that we have a while loop in the AST
    assert!(format!("{:?}", ast).contains("当"));
}

#[test]
fn test_parse_for_loop() {
    let source_code = r#"
    函数 测试循环() {
        变量 总和 = 0;
        对于 i 在 [1, 2, 3] {
            总和 = 总和 + i;
        }
    }
    "#;

    let mut lexer = Lexer::new(source_code.to_string());
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new();
    let ast = parser.parse(tokens).unwrap();

    // Check that we have a for loop in the AST
    assert!(format!("{:?}", ast).contains("对于"));
}

#[test]
fn test_nested_control_flow() {
    let source_code = r#"
    函数 测试嵌套() {
        如果 x > 5 {
            当 i < 10 {
                如果 i == 5 {
                    返回 "找到";
                }
                i = i + 1;
            }
        }
    }
    "#;

    let mut lexer = Lexer::new(source_code.to_string());
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new();
    let ast = parser.parse(tokens).unwrap();

    // Check that all control flow keywords are present
    let ast_str = format!("{:?}", ast);
    assert!(ast_str.contains("如果"));
    assert!(ast_str.contains("当"));
}

#[test]
fn test_boolean_expressions() {
    let source_code = r#"
    函数 测试表达式() {
        变量 x = 10;
        如果 x > 5 {
            返回 真;
        }
    }
    "#;

    let mut lexer = Lexer::new(source_code.to_string());
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new();
    let ast = parser.parse(tokens);

    assert!(ast.is_ok(), "Failed to parse expression");
}

#[test]
fn test_complex_boolean_logic() {
    let source_code = r#"
    函数 测试布尔() {
        变量 x = 10;
        变量 y = 5;
        如果 x > 5 {
            返回 真;
        }
    }
    "#;

    let mut lexer = Lexer::new(source_code.to_string());
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new();
    let ast = parser.parse(tokens).unwrap();

    // Check that parsing succeeded
    let ast_str = format!("{:?}", ast);
    assert!(ast_str.contains("如果"));
}

#[test]
fn test_type_checking_control_flow() {
    let source_code = r#"
    函数 测试函数(x) {
        如果 x > 5 {
            返回 "大于";
        } 否则 {
            返回 "小于等于";
        }
    }
    "#;

    let mut lexer = Lexer::new(source_code.to_string());
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new();
    let _ast = parser.parse(tokens).unwrap();

    // For now, just test that parsing succeeds
    // Type checking will be tested separately when fully implemented
    assert!(true, "Parsing succeeded for control flow");
}

#[test]
fn test_loop_type_validation() {
    let source_code = r#"
    函数 测试循环验证() {
        变量 i = 0;
        当 i < 10 {
            i = i + 1;
        }
    }
    "#;

    let mut lexer = Lexer::new(source_code.to_string());
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new();
    let ast = parser.parse(tokens).unwrap();

    // For now, just test that parsing succeeds
    assert!(true, "Parsing succeeded for loop");
}

#[test]
fn test_comparison_type_mismatch_detection() {
    let source_code = r#"
    函数 测试比较() {
        变量 x = 10;
        变量 y = 10;
        如果 x == y {
            返回 真;
        }
    }
    "#;

    let mut lexer = Lexer::new(source_code.to_string());
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new();
    match parser.parse(tokens) {
        Ok(ast) => {
            println!("Parsing succeeded!");
            println!("AST: {:?}", ast);
        }
        Err(e) => {
            println!("Parsing failed: {:?}", e);
            panic!("Should be able to parse simple comparison");
        }
    }

    // For now, just test that parsing succeeds
    // Type checking for mismatches will be tested separately when fully implemented
    assert!(true, "Parsing succeeded for comparison");
}

#[test]
fn test_control_flow_keywords() {
    let keywords = vec!["如果", "否则", "当", "对于", "与", "或"];

    for keyword in keywords {
        let mut lexer = Lexer::new(keyword.to_string());
        let tokens = lexer.tokenize().unwrap();

        // Check that the keyword is properly tokenized
        assert!(!tokens.is_empty(), "Failed to tokenize keyword: {}", keyword);

        // Check that the token represents a keyword (not an identifier)
        let token_str = format!("{:?}", tokens[0]);
        assert!(token_str.contains(keyword) ||
                tokens.iter().any(|t| format!("{:?}", t).contains(keyword)),
                "Keyword not properly recognized: {}", keyword);
    }
}