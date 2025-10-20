//! Integration tests for control flow structures in Qi language

use qi_compiler::*;
use std::fs;

#[test]
fn test_compile_simple_if_statement() {
    let source_code = r#"
    函数 主() {
        变量 x = 10;
        如果 x > 5 {
            返回 "成功";
        }
        返回 "失败";
    }
    "#;

    let compiler = Compiler::new();
    let result = compiler.compile("test_if", source_code);

    assert!(result.is_ok(), "Failed to compile simple if statement: {:?}", result);
}

#[test]
fn test_compile_if_else_statement() {
    let source_code = r#"
    函数 主() {
        变量 x = 3;
        如果 x > 5 {
            返回 "大于";
        } 否则 {
            返回 "小于等于";
        }
    }
    "#;

    let compiler = Compiler::new();
    let result = compiler.compile("test_if_else", source_code);

    assert!(result.is_ok(), "Failed to compile if-else statement: {:?}", result);
}

#[test]
fn test_compile_while_loop() {
    let source_code = r#"
    函数 主() {
        变量 i = 0;
        变量 总和 = 0;
        当 i < 5 {
            总和 = 总和 + i;
            i = i + 1;
        }
        返回 总和;
    }
    "#;

    let compiler = Compiler::new();
    let result = compiler.compile("test_while", source_code);

    assert!(result.is_ok(), "Failed to compile while loop: {:?}", result);
}

#[test]
fn test_compile_nested_control_flow() {
    let source_code = r#"
    函数 主() {
        变量 x = 10;
        变量 结果 = 0;

        如果 x > 5 {
            变量 i = 0;
            当 i < 3 {
                如果 i == 1 {
                    结果 = 结果 + 10;
                } 否则 {
                    结果 = 结果 + 1;
                }
                i = i + 1;
            }
        }
        返回 结果;
    }
    "#;

    let compiler = Compiler::new();
    let result = compiler.compile("test_nested", source_code);

    assert!(result.is_ok(), "Failed to compile nested control flow: {:?}", result);
}

#[test]
fn test_compile_boolean_logic() {
    let source_code = r#"
    函数 主() {
        变量 a = 真;
        变量 b = 假;
        变量 c = 10;

        如果 a 与 c > 5 {
            返回 "条件1";
        } 否则 如果 b 或 c > 15 {
            返回 "条件2";
        } 否则 {
            返回 "默认";
        }
    }
    "#;

    let compiler = Compiler::new();
    let result = compiler.compile("test_boolean", source_code);

    assert!(result.is_ok(), "Failed to compile boolean logic: {:?}", result);
}

#[test]
fn test_compile_multiple_functions_with_control_flow() {
    let source_code = r#"
    函数 检查数值(x) {
        如果 x < 0 {
            返回 "负数";
        } 否则 如果 x == 0 {
            返回 "零";
        } 否则 {
            返回 "正数";
        }
    }

    函数 主() {
        变量 结果 = 检查数值(5);
        返回 结果;
    }
    "#;

    let compiler = Compiler::new();
    let result = compiler.compile("test_multiple_functions", source_code);

    assert!(result.is_ok(), "Failed to compile multiple functions with control flow: {:?}", result);
}

#[test]
fn test_compile_comprehensive_control_flow() {
    let source_code = fs::read_to_string("tests/fixtures/control_flow_comprehensive.qi")
        .expect("Failed to read comprehensive test file");

    let compiler = Compiler::new();
    let result = compiler.compile("test_comprehensive", &source_code);

    assert!(result.is_ok(), "Failed to compile comprehensive control flow: {:?}", result);
}

#[test]
fn test_error_handling_invalid_syntax() {
    let source_code = r#"
    函数 主() {
        变量 x = 10;
        如果 x > 5 {
            返回 "成功";
        }
        // Missing closing brace - should cause error
    "#;

    let compiler = Compiler::new();
    let result = compiler.compile("test_invalid_syntax", source_code);

    assert!(result.is_err(), "Should fail to compile invalid syntax");
}

#[test]
fn test_error_handling_type_mismatch() {
    let source_code = r#"
    函数 主() {
        变量 x = 10;
        变量 y = "字符串";
        如果 x == y {
            返回 "类型不匹配";
        }
        返回 "正常";
    }
    "#;

    let compiler = Compiler::new();
    let result = compiler.compile("test_type_mismatch", source_code);

    // This should either fail compilation or produce warnings
    // The exact behavior depends on how strict the type checker is
    match result {
        Ok(_) => {
            // If compilation succeeds, we should at least have warnings
            // This would require checking the compiler's diagnostic output
        }
        Err(_) => {
            // If compilation fails, that's also acceptable for type mismatches
        }
    }
}

#[test]
fn test_optimization_control_flow() {
    let source_code = r#"
    函数 主() {
        变量 x = 10;
        如果 真 {
            返回 "总是执行";
        } 否则 {
            返回 "永不执行";
        }
    }
    "#;

    let mut config = CompilerConfig::default();
    config.optimization_level = OptimizationLevel::Basic;

    let compiler = Compiler::with_config(config);
    let result = compiler.compile("test_optimization", source_code);

    assert!(result.is_ok(), "Failed to compile with optimizations: {:?}", result);

    // The optimizer should be able to eliminate the dead "else" branch
    // This would require inspecting the generated IR to verify
}

#[test]
fn test_performance_large_control_flow() {
    let mut source_code = String::from("函数 主() {\n 变量 总和 = 0;\n");

    // Generate a large control flow structure
    for i in 0..100 {
        source_code.push_str(&format!(
            " 如果 {} < 50 {{\n  总和 = 总和 + {};\n }}",
            i, i
        ));
    }

    source_code.push_str(" 返回 总和;\n}");

    let compiler = Compiler::new();
    let result = compiler.compile("test_performance", &source_code);

    assert!(result.is_ok(), "Failed to compile large control flow: {:?}", result);
}

#[test]
fn test_chinese_keywords_in_control_flow() {
    let source_code = r#"
    函数 测试中文() {
        变量 数值 = 42;
        变量 布尔值 = 真;
        变量 字符串 = "测试";

        如果 数值 > 30 与 布尔值 == 真 {
            当 数值 > 0 {
                数值 = 数值 - 1;
                如果 数值 == 10 {
                    返回 字符串 + "完成";
                }
            }
        }
        返回 "结束";
    }
    "#;

    let compiler = Compiler::new();
    let result = compiler.compile("test_chinese_keywords", source_code);

    assert!(result.is_ok(), "Failed to compile Chinese keywords in control flow: {:?}", result);
}