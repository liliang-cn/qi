# Qi Language Compiler - Data Model

**Date**: 2025-10-19
**Purpose**: Define data entities, relationships, and validation rules for the Qi compiler
**Scope**: Core compiler data structures, AST representation, and type system

## Core Data Entities

### 1. Source File Management

#### QiSourceFile
```rust
pub struct QiSourceFile {
    pub path: PathBuf,              // Absolute path to source file
    pub content: String,            // UTF-8 encoded source content
    pub encoding: Encoding,         // Always UTF-8, but track for validation
    pub line_offsets: Vec<usize>,   // Byte offsets for line numbers (0-based)
    pub last_modified: SystemTime,  // File modification time
    pub dependencies: Vec<PathBuf>,  // Import dependencies
}

impl QiSourceFile {
    pub fn new(path: PathBuf, content: String) -> Result<Self, SourceError> {
        // Validate UTF-8 encoding
        if !content.is_utf8() {
            return Err(SourceError::InvalidEncoding);
        }

        // Compute line offsets for efficient line/column lookup
        let line_offsets = Self::compute_line_offsets(&content);

        Ok(Self {
            path,
            content,
            encoding: Encoding::Utf8,
            line_offsets,
            last_modified: SystemTime::now(),
            dependencies: Vec::new(),
        })
    }

    pub fn get_position(&self, byte_offset: usize) -> Option<Position> {
        // Binary search to find line number
        let line = self.line_offsets.binary_search(&byte_offset)
            .unwrap_or_else(|idx| idx);

        let column = byte_offset - self.line_offsets[line];
        Some(Position { line: line + 1, column })
    }
}
```

**Validation Rules**:
- Content must be valid UTF-8
- File path must have `.qi` extension
- Content cannot exceed practical limits (10MB max)
- Dependencies must be valid Qi files

### 2. Lexical Analysis

#### Token
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,            // Raw text from source
    pub span: Span,              // Source location (start, end)
    pub line: usize,             // Line number (1-based)
    pub column: usize,           // Column number (1-based)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,            // Byte offset start
    pub end: usize,              // Byte offset end (exclusive)
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Chinese Keywords (multi-character)
    如果,      // if
    否则,      // else
    循环,      // loop
    当,        // while
    对于,      // for
    函数,      // function
    返回,      // return
    变量,      // variable
    常量,      // constant
    整数,      // integer
    字符串,    // string
    布尔,      // boolean
    浮点数,    // float

    // Single-character tokens
    加,        // +
    减,        // -
    乘,        // *
    除,        // /
    等于,      // ==
    不等于,    // !=
    大于,      // >
    小于,      // <
    分号,      // ;
    逗号,      // ,
    左括号,    // (
    右括号,    // )
    左大括号,  // {
    右大括号,  // }

    // Identifiers and literals
    标识符(String),      // Variable/function names
    字符串字面量(String), // String literals
    整数字面量(i64),     // Integer literals
    浮点数字面量(f64),   // Float literals
    布尔字面量(bool),    // Boolean literals (真, 假)

    // Special
    文件结束,
    错误(String),       // Lexical error
}
```

**Validation Rules**:
- Token text must match token kind semantics
- Unicode identifiers must be valid Chinese characters or underscores
- Numeric literals must be within reasonable bounds
- String literals must have proper escape sequences

### 3. Abstract Syntax Tree (AST)

#### AST Node Hierarchy
```rust
#[derive(Debug, Clone)]
pub enum AstNode {
    // Top-level program
    程序(Program),

    // Statements
    变量声明(VariableDeclaration),
    函数声明(FunctionDeclaration),
    如果语句(IfStatement),
    循环语句(LoopStatement),
    当语句(WhileStatement),
    对于语句(ForStatement),
    返回语句(ReturnStatement),
    表达式语句(ExpressionStatement),

    // Expressions
    字面量表达式(LiteralExpression),
    标识符表达式(IdentifierExpression),
    二元操作表达式(BinaryExpression),
    函数调用表达式(FunctionCallExpression),
    赋值表达式(AssignmentExpression),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub package_name: Option<String>,
    pub imports: Vec<ImportStatement>,
    pub statements: Vec<AstNode>,
    pub source_span: Span,
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub type_annotation: Option<TypeNode>,
    pub initializer: Option<Box<AstNode>>,
    pub is_mutable: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeNode>,
    pub body: Vec<AstNode>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: TypeNode,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Box<AstNode>,
    pub then_branch: Vec<AstNode>,
    pub else_branch: Option<Vec<AstNode>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub left: Box<AstNode>,
    pub operator: BinaryOperator,
    pub right: Box<AstNode>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    加, 减, 乘, 除, 取余,
    等于, 不等于, 大于, 小于, 大于等于, 小于等于,
    与, 或, 非,
}
```

**Validation Rules**:
- AST must be semantically valid (no undeclared variables, etc.)
- All control structures must have proper scope
- Function calls must match function signatures
- Type annotations must be consistent with usage

### 4. Type System

#### Type Nodes
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum TypeNode {
    基础类型(BasicType),
    函数类型(FunctionType),
    数组类型(ArrayType),
    未知,    // Inferred type during compilation
    错误,    // Type error
}

#[derive(Debug, Clone, PartialEq)]
pub enum BasicType {
    整数,
    长整数,
    短整数,
    字节,
    浮点数,
    布尔,
    字符,
    字符串,
    空,      // void/nil
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType {
    pub parameters: Vec<TypeNode>,
    pub return_type: Box<TypeNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayType {
    pub element_type: Box<TypeNode>,
    pub size: Option<usize>,    // None for dynamic arrays
}
```

#### Type Checking Rules
```rust
pub struct TypeChecker {
    symbol_table: SymbolTable,
    current_scope: Vec<SymbolTable>,
    errors: Vec<TypeError>,
}

impl TypeChecker {
    pub fn infer_type(&mut self, node: &AstNode) -> TypeNode {
        match node {
            AstNode::字面量表达式(literal) => self.infer_literal_type(literal),
            AstNode::二元操作表达式(binary) => self.infer_binary_type(binary),
            AstNode::标识符表达式(identifier) => self.lookup_identifier_type(identifier),
            AstNode::函数调用表达式(call) => self.infer_call_type(call),
            // ... other cases
        }
    }

    pub fn check_type_compatibility(&self, expected: &TypeNode, actual: &TypeNode) -> bool {
        match (expected, actual) {
            (TypeNode::基础类型(expected_basic), TypeNode::基础类型(actual_basic)) => {
                self.is_basic_type_compatible(expected_basic, actual_basic)
            },
            (TypeNode::函数类型(expected_func), TypeNode::函数类型(actual_func)) => {
                self.is_function_type_compatible(expected_func, actual_func)
            },
            _ => false,
        }
    }
}
```

### 5. Symbol Table and Scoping

#### Symbol Table
```rust
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub type_node: TypeNode,
    pub scope_level: usize,
    pub span: Span,
    pub is_mutable: bool,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    变量,
    函数(FunctionInfo),
    常量,
    类型(TypeInfo),
}

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub parameters: Vec<Parameter>,
    pub return_type: TypeNode,
    pub is_defined: bool,        // Declaration vs definition
}

#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub definition: TypeDefinition,
    pub is_builtin: bool,
}

pub struct SymbolTable {
    symbols: HashMap<String, Vec<Symbol>>,  // Name -> stack of symbols
    current_scope: usize,
    errors: Vec<ScopeError>,
}

impl SymbolTable {
    pub fn enter_scope(&mut self) {
        self.current_scope += 1;
    }

    pub fn exit_scope(&mut self) {
        // Remove symbols from current scope
        self.symbols.retain(|_, symbols| {
            symbols.last().map_or(true, |sym| sym.scope_level <= self.current_scope - 1)
        });
        self.current_scope -= 1;
    }

    pub fn define_symbol(&mut self, symbol: Symbol) -> Result<(), ScopeError> {
        // Check for name conflicts in current scope
        if let Some(existing_symbols) = self.symbols.get(&symbol.name) {
            if let Some(existing) = existing_symbols.iter()
                .find(|sym| sym.scope_level == self.current_scope) {
                return Err(ScopeError::NameConflict {
                    name: symbol.name.clone(),
                    existing_span: existing.span,
                    new_span: symbol.span,
                });
            }
        }

        self.symbols.entry(symbol.name.clone())
            .or_insert_with(Vec::new)
            .push(symbol);

        Ok(())
    }

    pub fn lookup_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
            .and_then(|symbols| symbols.iter()
                .filter(|sym| sym.scope_level <= self.current_scope)
                .max_by_key(|sym| sym.scope_level))
    }
}
```

### 6. Intermediate Representation (IR)

#### IR Instructions
```rust
#[derive(Debug, Clone)]
pub enum IrInstruction {
    // Memory operations
    分配(Allocate),
    存储(Store),
    加载(Load),

    // Arithmetic
    二元操作(BinaryOp),
    一元操作(UnaryOp),

    // Control flow
    跳转(Jump),
    条件跳转(ConditionalJump),
    函数调用(FunctionCall),
    返回(Return),

    // Special
    标签(Label),
    注释(String),
}

#[derive(Debug, Clone)]
pub struct Allocate {
    pub dest: String,          // Destination variable name
    pub type_node: TypeNode,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct BinaryOp {
    pub dest: String,
    pub left: String,
    pub operator: BinaryOperator,
    pub right: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub dest: Option<String>,   // None for void functions
    pub callee: String,
    pub arguments: Vec<String>,
    pub span: Span,
}
```

#### Basic Block
```rust
#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub label: String,
    pub instructions: Vec<IrInstruction>,
    pub terminator: Option<IrInstruction>,
}

#[derive(Debug)]
pub struct FunctionIr {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub blocks: HashMap<String, BasicBlock>,
    pub entry_block: String,
    pub return_type: TypeNode,
}
```

### 7. Error Reporting

#### Error Types
```rust
#[derive(Debug, Clone)]
pub enum CompilerError {
    语法错误(SyntaxError),
    语义错误(SemanticError),
    类型错误(TypeError),
    运行时错误(RuntimeError),
    输入输出错误(IOError),
}

#[derive(Debug, Clone)]
pub struct SyntaxError {
    pub message: String,
    pub span: Span,
    pub suggestion: Option<String>,
    pub error_code: String,
}

#[derive(Debug, Clone)]
pub struct TypeError {
    pub message: String,
    pub expected: TypeNode,
    pub actual: TypeNode,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ErrorDiagnostic {
    pub error: CompilerError,
    pub file_path: PathBuf,
    pub source_context: String,
    pub severity: ErrorSeverity,
}

#[derive(Debug, Clone)]
pub enum ErrorSeverity {
    错误,    // Error - compilation stops
    警告,    // Warning - compilation continues
    信息,    // Info - for debugging
}
```

### 8. Configuration and Settings

#### Compiler Configuration
```rust
#[derive(Debug, Clone)]
pub struct CompilerConfig {
    pub optimization_level: OptimizationLevel,
    pub target_platform: CompilationTarget,
    pub debug_symbols: bool,
    pub runtime_checks: bool,
    pub output_format: OutputFormat,
    pub source_paths: Vec<PathBuf>,
    pub library_paths: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum OptimizationLevel {
    无优化,    // O0 - No optimization
    基础优化,  // O1 - Basic optimization
    标准优化,  // O2 - Standard optimization
    最大优化,  // O3 - Maximum optimization
}

#[derive(Debug, Clone)]
pub enum OutputFormat {
    可执行文件,  // Executable binary
    目标文件,    // Object file
    汇编代码,    // Assembly file
    LLVM_IR,     // LLVM intermediate representation
}
```

## Data Relationships

### Entity Relationship Summary

```
QiSourceFile
    └── 1..* Token (via lexical analysis)
    └── 1..* AstNode (via parsing)

AstNode
    └── 1..1 TypeNode (via type checking)
    └── 0..1 Symbol (via symbol table)

Symbol
    └── 1..1 SymbolTable
    └── 1..1 TypeNode

TypeNode
    └── 0..* TypeNode (for complex types)

FunctionIr
    └── 1..* BasicBlock
    └── 1..* IrInstruction

ErrorDiagnostic
    └── 1..1 Span
    └── 1..1 CompilerError
```

### State Transitions

#### Compilation Pipeline
```
Source File → Tokens → AST → Typed AST → IR → Binary
```

#### Type Inference
```
Unknown Type → Literal Type → Inferred Type → Final Type
```

#### Symbol Resolution
```
Identifier → Symbol Lookup → Type Resolution → Validation
```

## Validation Rules Summary

### Source File Validation
- Must be valid UTF-8 encoding
- File extension must be `.qi`
- Content size must be reasonable (<10MB)
- Path must be accessible and readable

### Token Validation
- Chinese keywords must be exact matches
- Identifiers must start with Chinese character or underscore
- Numeric literals must be within language bounds
- String literals must have proper escape sequences

### AST Validation
- All expressions must be typeable
- Control structures must have proper scope
- Function calls must match declarations
- Variables must be declared before use

### Type Validation
- Type annotations must be consistent with usage
- Function return types must match actual returns
- Binary operations must have compatible types
- Array indexing must use integer types

### IR Validation
- All variables must be defined before use
- Control flow must be well-formed
- Function calls must match signatures
- Memory operations must be type-safe

## Performance Considerations

### Memory Usage
- Use arena allocation for AST nodes
- String interning for identifiers and keywords
- Lazy evaluation for large structures
- Efficient hash maps for symbol tables

### Compilation Speed
- Incremental parsing and type checking
- Parallel compilation of independent modules
- Caching of expensive operations
- Optimized Unicode handling

### Data Structure Choices
- `Vec<T>` for ordered collections
- `HashMap<String, V>` for symbol lookup
- `HashSet<T>` for uniqueness checks
- `Rc<T>` for shared ownership where needed

This data model provides a comprehensive foundation for implementing the Qi language compiler with proper separation of concerns, clear validation rules, and efficient data structures optimized for Chinese language processing.