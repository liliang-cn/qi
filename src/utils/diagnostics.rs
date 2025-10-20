//! Diagnostic and error reporting for Qi language

use std::path::PathBuf;

/// Diagnostic level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticLevel {
    错误,    // Error
    警告,    // Warning
    信息,    // Info
}

/// Diagnostic message
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub code: String,
    pub message: String,
    pub english_message: String,
    pub file_path: Option<PathBuf>,
    pub span: Option<crate::lexer::Span>,
    pub suggestion: Option<String>,
    pub related_code: Option<String>,
}

/// Diagnostic manager
pub struct DiagnosticManager {
    diagnostics: Vec<Diagnostic>,
    max_errors: usize,
    max_warnings: usize,
}

impl DiagnosticManager {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
            max_errors: 100,
            max_warnings: 100,
        }
    }

    /// Add syntax error with code and suggestion
    pub fn syntax_error(&mut self, span: crate::lexer::Span, expected: &str, found: &str, suggestion: Option<&str>) {
        self.add_diagnostic(Diagnostic {
            level: DiagnosticLevel::错误,
            code: "E001".to_string(),
            message: format!("语法错误: 期望 '{}', 找到 '{}'", expected, found),
            english_message: format!("Syntax error: expected '{}', found '{}'", expected, found),
            file_path: None,
            span: Some(span),
            suggestion: suggestion.map(|s| s.to_string()),
            related_code: None,
        });
    }

    /// Add type mismatch error with suggestion
    pub fn type_mismatch_error(&mut self, span: crate::lexer::Span, expected: &str, found: &str, suggestion: Option<&str>) {
        self.add_diagnostic(Diagnostic {
            level: DiagnosticLevel::错误,
            code: "E002".to_string(),
            message: format!("类型不匹配: 期望 '{}', 实际 '{}'", expected, found),
            english_message: format!("Type mismatch: expected '{}', found '{}'", expected, found),
            file_path: None,
            span: Some(span),
            suggestion: suggestion.map(|s| s.to_string()),
            related_code: None,
        });
    }

    /// Add undefined variable error with suggestion
    pub fn undefined_variable_error(&mut self, span: crate::lexer::Span, var_name: &str, suggestion: Option<&str>) {
        self.add_diagnostic(Diagnostic {
            level: DiagnosticLevel::错误,
            code: "E003".to_string(),
            message: format!("未定义的变量: '{}'", var_name),
            english_message: format!("Undefined variable: '{}'", var_name),
            file_path: None,
            span: Some(span),
            suggestion: suggestion.map(|s| s.to_string()),
            related_code: None,
        });
    }

    /// Add function call error with suggestion
    pub fn function_call_error(&mut self, span: crate::lexer::Span, message: &str, suggestion: Option<&str>) {
        self.add_diagnostic(Diagnostic {
            level: DiagnosticLevel::错误,
            code: "E004".to_string(),
            message: format!("函数调用错误: {}", message),
            english_message: format!("Function call error: {}", message),
            file_path: None,
            span: Some(span),
            suggestion: suggestion.map(|s| s.to_string()),
            related_code: None,
        });
    }

    /// Add invalid operation error with suggestion
    pub fn invalid_operation_error(&mut self, span: crate::lexer::Span, operation: &str, type_name: &str, suggestion: Option<&str>) {
        self.add_diagnostic(Diagnostic {
            level: DiagnosticLevel::错误,
            code: "E005".to_string(),
            message: format!("无效操作: '{}' 对于类型 '{}'", operation, type_name),
            english_message: format!("Invalid operation: '{}' for type '{}'", operation, type_name),
            file_path: None,
            span: Some(span),
            suggestion: suggestion.map(|s| s.to_string()),
            related_code: None,
        });
    }

    /// Add struct field error with suggestion
    pub fn struct_field_error(&mut self, span: crate::lexer::Span, struct_name: &str, field_name: &str, suggestion: Option<&str>) {
        self.add_diagnostic(Diagnostic {
            level: DiagnosticLevel::错误,
            code: "E006".to_string(),
            message: format!("结构体 '{}' 没有字段 '{}'", struct_name, field_name),
            english_message: format!("Struct '{}' has no field '{}'", struct_name, field_name),
            file_path: None,
            span: Some(span),
            suggestion: suggestion.map(|s| s.to_string()),
            related_code: None,
        });
    }

    /// Add array access error with suggestion
    pub fn array_access_error(&mut self, span: crate::lexer::Span, message: &str, suggestion: Option<&str>) {
        self.add_diagnostic(Diagnostic {
            level: DiagnosticLevel::错误,
            code: "E007".to_string(),
            message: format!("数组访问错误: {}", message),
            english_message: format!("Array access error: {}", message),
            file_path: None,
            span: Some(span),
            suggestion: suggestion.map(|s| s.to_string()),
            related_code: None,
        });
    }

    /// Add general warning with suggestion
    pub fn warning(&mut self, code: &str, message: &str, suggestion: Option<&str>) {
        self.add_diagnostic(Diagnostic {
            level: DiagnosticLevel::警告,
            code: code.to_string(),
            message: message.to_string(),
            english_message: message.to_string(),
            file_path: None,
            span: None,
            suggestion: suggestion.map(|s| s.to_string()),
            related_code: None,
        });
    }

    /// Add unused variable warning
    pub fn unused_variable_warning(&mut self, span: crate::lexer::Span, var_name: &str) {
        self.add_diagnostic(Diagnostic {
            level: DiagnosticLevel::警告,
            code: "W001".to_string(),
            message: format!("未使用的变量: '{}'", var_name),
            english_message: format!("Unused variable: '{}'", var_name),
            file_path: None,
            span: Some(span),
            suggestion: Some("如果不需要此变量，请考虑删除它或在变量名前添加下划线前缀".to_string()),
            related_code: None,
        });
    }

    /// Add unreachable code warning
    pub fn unreachable_code_warning(&mut self, span: crate::lexer::Span) {
        self.add_diagnostic(Diagnostic {
            level: DiagnosticLevel::警告,
            code: "W002".to_string(),
            message: "不可达的代码".to_string(),
            english_message: "Unreachable code".to_string(),
            file_path: None,
            span: Some(span),
            suggestion: Some("请删除这段不可达的代码".to_string()),
            related_code: None,
        });
    }

    pub fn add_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn add_error(&mut self, code: &str, message: &str, english_message: &str) {
        self.add_diagnostic(Diagnostic {
            level: DiagnosticLevel::错误,
            code: code.to_string(),
            message: message.to_string(),
            english_message: english_message.to_string(),
            file_path: None,
            span: None,
            suggestion: None,
            related_code: None,
        });
    }

    pub fn add_warning(&mut self, code: &str, message: &str, english_message: &str) {
        self.add_diagnostic(Diagnostic {
            level: DiagnosticLevel::警告,
            code: code.to_string(),
            message: message.to_string(),
            english_message: english_message.to_string(),
            file_path: None,
            span: None,
            suggestion: None,
            related_code: None,
        }
        );
    }

    pub fn add_info(&mut self, code: &str, message: &str, english_message: &str) {
        self.add_diagnostic(Diagnostic {
            level: DiagnosticLevel::信息,
            code: code.to_string(),
            message: message.to_string(),
            english_message: english_message.to_string(),
            file_path: None,
            span: None,
            suggestion: None,
            related_code: None,
        });
    }

    pub fn get_diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    pub fn get_errors(&self) -> Vec<&Diagnostic> {
        self.diagnostics.iter()
            .filter(|d| d.level == DiagnosticLevel::错误)
            .collect()
    }

    pub fn get_warnings(&self) -> Vec<&Diagnostic> {
        self.diagnostics.iter()
            .filter(|d| d.level == DiagnosticLevel::警告)
            .collect()
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.level == DiagnosticLevel::错误)
    }

    pub fn error_count(&self) -> usize {
        self.diagnostics.iter()
            .filter(|d| d.level == DiagnosticLevel::错误)
            .count()
    }

    pub fn warning_count(&self) -> usize {
        self.diagnostics.iter()
            .filter(|d| d.level == DiagnosticLevel::警告)
            .count()
    }

    pub fn clear(&mut self) {
        self.diagnostics.clear();
    }

    pub fn set_max_errors(&mut self, max: usize) {
        self.max_errors = max;
    }

    pub fn set_max_warnings(&mut self, max: usize) {
        self.max_warnings = max;
    }

    pub fn format_diagnostic(&self, diagnostic: &Diagnostic) -> String {
        let level_str = match diagnostic.level {
            DiagnosticLevel::错误 => "错误",
            DiagnosticLevel::警告 => "警告",
            DiagnosticLevel::信息 => "信息",
        };

        let mut result = format!("{}: {}", level_str, diagnostic.message);

        if let Some(file_path) = &diagnostic.file_path {
            result.push_str(&format!(" ({})", file_path.display()));
        }

        if let Some(span) = diagnostic.span {
            result.push_str(&format!(" at {}..{}", span.start, span.end));
        }

        if !diagnostic.code.is_empty() {
            result.push_str(&format!(" [{}]", diagnostic.code));
        }

        if let Some(suggestion) = &diagnostic.suggestion {
            result.push_str(&format!("\n  建议: {}", suggestion));
        }

        result
    }

    pub fn print_diagnostics(&self) {
        for diagnostic in &self.diagnostics {
            eprintln!("{}", self.format_diagnostic(diagnostic));
        }
    }
}

impl Default for DiagnosticManager {
    fn default() -> Self {
        Self::new()
    }
}