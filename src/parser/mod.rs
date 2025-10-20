//! Chinese grammar parsing for Qi language using LALRPOP

pub mod ast;
pub mod error;

// Include the generated LALRPOP parser
include!(concat!(env!("OUT_DIR"), "/parser/grammar.rs"));

pub use ast::{
    AstNode, Program, TypeNode, BasicType, LiteralValue, LiteralExpression, IdentifierExpression,
    VariableDeclaration, FunctionDeclaration, ReturnStatement, ExpressionStatement,
    IfStatement, WhileStatement, LoopStatement, ForStatement, BinaryExpression, BinaryOperator,
    AssignmentExpression, FunctionCallExpression, Parameter, ArrayAccessExpression,
    ArrayLiteralExpression, StringConcatExpression, ArrayType, StructDeclaration, StructField,
    EnumDeclaration, EnumVariant, StructType, EnumType, StructLiteralExpression,
    StructFieldValue, FieldAccessExpression
};
pub use error::ParseError;

/// Qi language parser using LALRPOP-generated parser
pub struct Parser {
    _private: (),
}

impl Parser {
    /// Create a new parser
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// Parse source code directly into an AST
    pub fn parse_source(&self, source: &str) -> Result<Program, ParseError> {
        // Use LALRPOP-generated parser with string input
        use crate::parser::__parse__Program::ProgramParser;
        ProgramParser::new()
            .parse(source)
            .map_err(|_| ParseError::ParseFailed)
    }

    /// Parse tokens into an AST (legacy method - tokenizes first)
    pub fn parse(&self, tokens: Vec<crate::lexer::Token>) -> Result<Program, ParseError> {
        // Reconstruct source from tokens preserving original structure
        // Use the original span information to maintain proper spacing
        let mut source = String::new();
        let mut last_end = 0;

        for token in &tokens {
            // Preserve spacing between tokens based on original positions
            if token.span.start > last_end {
                // Add the original whitespace/newlines that were between tokens
                // For now, add a space if there was a gap
                source.push(' ');
            }
            source.push_str(&token.text);
            last_end = token.span.end;
        }

        self.parse_source(&source.trim())
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}