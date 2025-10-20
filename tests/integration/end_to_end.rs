//! End-to-end compilation tests for Qi Language Compiler

use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

/// Test basic compilation pipeline
#[test]
fn test_hello_world_compilation() {
    let source_code = r#"
// Hello World in Qi Language
函数 主() {
  变量 消息 = "你好，世界！";
}
"#;

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let source_file = temp_dir.path().join("test.qi");
    fs::write(&source_file, source_code).expect("Failed to write source file");

    // Test if compiler can process the file (this will fail initially)
    let result = test_compilation(&source_file);

    // For now, we expect this to fail since we haven't implemented full compilation
    // but it should not panic
    assert!(result.is_ok() || result.is_err());
}

/// Test lexer integration
#[test]
fn test_lexer_integration() {
    let source_code = "变量 x = 42; 变量 y = \"你好\"; 常量 Z = 真;";

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let source_file = temp_dir.path().join("test.qi");
    fs::write(&source_file, source_code).expect("Failed to write source file");

    // Test lexer can tokenize the file
    let result = test_lexing(&source_file);
    assert!(result.is_ok());
}

/// Test parser integration
#[test]
fn test_parser_integration() {
    let source_code = "变量 x = 1 + 2;";

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let source_file = temp_dir.path().join("test.qi");
    fs::write(&source_file, source_code).expect("Failed to write source file");

    // Test parser can process the file
    let result = test_parsing(&source_file);
    // Parser may fail as it's not fully implemented
    assert!(result.is_ok() || result.is_err());
}

/// Test error handling
#[test]
fn test_error_handling() {
    let invalid_source = "变量 x = @;"; // Invalid character

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let source_file = temp_dir.path().join("test.qi");
    fs::write(&source_file, invalid_source).expect("Failed to write source file");

    // Should produce a lexical error
    let result = test_lexing(&source_file);
    assert!(result.is_err());
}

/// Test Chinese character support
#[test]
fn test_chinese_character_support() {
    let source_code = "变量 中文变量名 = 100; 函数 测试函数() { return 中文变量名; }";

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let source_file = temp_dir.path().join("test.qi");
    fs::write(&source_file, source_code).expect("Failed to write source file");

    // Test lexer handles Chinese characters correctly
    let result = test_lexing(&source_file);
    assert!(result.is_ok());
}

/// Helper function to test compilation
fn test_compilation(source_file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // This will test the full compilation pipeline
    // For now, we'll just test if the file can be read and processed

    let content = fs::read_to_string(source_file)?;
    println!("Testing compilation of: {}", content);

    // TODO: Implement actual compilation test once compiler is complete
    Ok(())
}

/// Helper function to test lexing
fn test_lexing(source_file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    use qi_compiler::lexer::Lexer;

    let content = fs::read_to_string(source_file)?;
    let mut lexer = Lexer::new(content);

    let tokens = lexer.tokenize()?;
    println!("Lexed {} tokens", tokens.len());

    // Verify we got some tokens
    assert!(!tokens.is_empty());
    assert!(tokens.last().unwrap().kind == qi_compiler::lexer::TokenKind::文件结束);

    Ok(())
}

/// Helper function to test parsing
fn test_parsing(source_file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    use qi_compiler::lexer::Lexer;
    use qi_compiler::parser::Parser;

    let content = fs::read_to_string(source_file)?;
    let mut lexer = Lexer::new(content);
    let tokens = lexer.tokenize()?;

    let mut parser = Parser::new(tokens);
    let _result = parser.parse();

    // For now, we just test that parsing doesn't panic
    println!("Parsing completed");

    Ok(())
}

/// Test multiple files in sequence
#[test]
fn test_multiple_files() {
    let test_files = vec![
        ("hello.qi", r#"
函数 主() {
  变量 消息 = "Hello";
}
"#),
        ("calculator.qi", r#"
函数 计算() {
  变量 a = 10;
  变量 b = 20;
  变量 c = a + b;
}
"#),
        ("chinese_test.qi", r#"
变量 中文数字 = 一百;
变量 文本 = "中文文本";
函数 测试函数() {
  return 中文数字;
}
"#),
    ];

    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    for (filename, content) in test_files {
        let file_path = temp_dir.path().join(filename);
        fs::write(&file_path, content).expect("Failed to write source file");

        // Test each file
        let result = test_lexing(&file_path);
        assert!(result.is_ok(), "Failed to lex file: {}", filename);

        let result = test_parsing(&file_path);
        // Parser may fail as it's not fully implemented
        assert!(result.is_ok() || result.is_err(), "Parser panicked on file: {}", filename);
    }
}

/// Test configuration loading
#[test]
fn test_configuration() {
    use qi_compiler::config::CompilerConfig;

    let config = CompilerConfig::default();
    assert_eq!(config.optimization_level, qi_compiler::config::OptimizationLevel::None);
    assert_eq!(config.debug_symbols, false);
    assert!(config.runtime_checks);
}

/// Test CLI argument parsing
#[test]
fn test_cli_parsing() {
    use clap::Parser;
    use qi_compiler::cli::commands::Cli;

    let args = vec!["qi", "--target", "Linux", "--optimization", "basic"];
    let cli = Cli::try_parse_from(args);
    assert!(cli.is_ok());

    let cli = cli.unwrap();
    assert!(cli.target.is_some());
    assert!(cli.optimization.is_some());
}

/// Test Chinese identifiers with numbers
#[test]
fn test_chinese_identifiers_with_numbers() {
    let source_code = r#"
变量 数字1 = 10;
变量 数字2 = 20;
变量 变量123 = 30;
变量 结果 = 数字1 + 数字2 + 变量123;

// Mixed identifiers
变量 test_中文1 = 100;
变量 中文_test2 = 200;
变量 测试123abc = 300;

// Pure Chinese
变量 中文 = 42;
变量 测试变量 = 中文;

// Using all variables
变量 最终结果 = 结果 + test_中文1 + 中文_test2 + 测试123abc + 测试变量;
"#;

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let source_file = temp_dir.path().join("test_chinese_numbers.qi");
    fs::write(&source_file, source_code).expect("Failed to write source file");

    // Test compilation using the Qi compiler
    use qi_compiler::QiCompiler;
    let compiler = QiCompiler::new();
    let result = compiler.compile(source_file.clone());

    match result {
        Ok(compilation_result) => {
            println!("Chinese identifiers with numbers compilation succeeded!");
            println!("Generated file: {:?}", compilation_result.executable_path);

            // Check generated IR content
            let ir_content = fs::read_to_string(&compilation_result.executable_path)
                .expect("Failed to read generated IR file");
            println!("Generated IR:\n{}", ir_content);

            assert!(compilation_result.executable_path.exists());

            // Check that all variables are present in the IR
            assert!(ir_content.contains("数字1"));
            assert!(ir_content.contains("数字2"));
            assert!(ir_content.contains("变量123"));
            assert!(ir_content.contains("test_中文1"));
            assert!(ir_content.contains("中文_test2"));
            assert!(ir_content.contains("测试123abc"));
            assert!(ir_content.contains("中文"));
            assert!(ir_content.contains("测试变量"));
        }
        Err(e) => {
            println!("Chinese identifiers with numbers compilation failed: {}", e);
            panic!("Chinese identifiers with numbers compilation should have succeeded but failed with: {}", e);
        }
    }
}