//! Source file management for Qi language

use std::path::PathBuf;
use std::fs;
use std::time::SystemTime;

/// Source file representation
#[derive(Debug, Clone)]
pub struct SourceFile {
    pub path: PathBuf,
    pub content: String,
    pub encoding: Encoding,
    pub line_offsets: Vec<usize>,
    pub last_modified: SystemTime,
    pub dependencies: Vec<PathBuf>,
}

/// Text encoding
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Encoding {
    Utf8,
    Unknown,
}

/// Source file error
#[derive(Debug, thiserror::Error)]
pub enum SourceError {
    /// File not found
    #[error("文件未找到: {0}")]
    NotFound(PathBuf),

    /// I/O error
    #[error("I/O 错误: {0}")]
    Io(#[from] std::io::Error),

    /// Invalid encoding
    #[error("无效的编码: {0}")]
    InvalidEncoding(String),

    /// File too large
    #[error("文件过大: {0} 字节")]
    TooLarge(usize),
}

impl SourceFile {
    pub fn new(path: PathBuf) -> Result<Self, SourceError> {
        let content = fs::read_to_string(&path)?;
        let last_modified = fs::metadata(&path)?.modified()
            .unwrap_or_else(|_| SystemTime::now());

        Self::from_content(path, content, last_modified)
    }

    pub fn from_content(path: PathBuf, content: String, last_modified: SystemTime) -> Result<Self, SourceError> {
        // Validate UTF-8 encoding (String is always UTF-8 in Rust)
        // TODO: Add additional validation if needed

        // Check file size (10MB limit)
        if content.len() > 10 * 1024 * 1024 {
            return Err(SourceError::TooLarge(content.len()));
        }

        // Compute line offsets
        let line_offsets = Self::compute_line_offsets(&content);

        Ok(Self {
            path,
            content,
            encoding: Encoding::Utf8,
            line_offsets,
            last_modified,
            dependencies: Vec::new(),
        })
    }

    fn compute_line_offsets(content: &str) -> Vec<usize> {
        let mut offsets = vec![0]; // First line starts at byte 0

        for (byte_offset, c) in content.char_indices() {
            if c == '\n' {
                offsets.push(byte_offset + 1); // Next line starts after \n
            }
        }

        offsets
    }

    pub fn get_position(&self, byte_offset: usize) -> Option<Position> {
        if byte_offset >= self.content.len() {
            return None;
        }

        // Binary search to find line number
        let line = self.line_offsets.binary_search(&byte_offset)
            .unwrap_or_else(|idx| idx);

        let column = byte_offset - self.line_offsets[line];
        Some(Position {
            line: line + 1, // 1-based line numbers
            column: column + 1, // 1-based column numbers
        })
    }

    pub fn get_line(&self, line_number: usize) -> Option<&str> {
        if line_number == 0 || line_number > self.line_offsets.len() {
            return None;
        }

        let line_index = line_number - 1;
        let start = self.line_offsets[line_index];
        let end = if line_index + 1 < self.line_offsets.len() {
            self.line_offsets[line_index + 1]
        } else {
            self.content.len()
        };

        // Extract line without trailing newline
        let line = &self.content[start..end];
        let line = line.trim_end_matches('\n');
        Some(line)
    }

    pub fn get_line_range(&self, line_number: usize) -> Option<(usize, usize)> {
        if line_number == 0 || line_number > self.line_offsets.len() {
            return None;
        }

        let line_index = line_number - 1;
        let start = self.line_offsets[line_index];
        let end = if line_index + 1 < self.line_offsets.len() {
            self.line_offsets[line_index + 1]
        } else {
            self.content.len()
        };

        Some((start, end))
    }

    pub fn add_dependency(&mut self, path: PathBuf) {
        if !self.dependencies.contains(&path) {
            self.dependencies.push(path);
        }
    }

    pub fn get_dependencies(&self) -> &[PathBuf] {
        &self.dependencies
    }

    pub fn is_modified_since(&self, timestamp: SystemTime) -> bool {
        self.last_modified > timestamp
    }

    pub fn get_filename(&self) -> Option<&str> {
        self.path.file_name()?.to_str()
    }

    pub fn get_extension(&self) -> Option<&str> {
        self.path.extension()?.to_str()
    }

    pub fn is_qi_file(&self) -> bool {
        self.get_extension() == Some("qi")
    }
}

/// Source position (line and column)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

/// Source file manager
pub struct SourceManager {
    files: Vec<SourceFile>,
    max_files: usize,
}

impl SourceManager {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            max_files: 1000,
        }
    }

    pub fn load_file(&mut self, path: PathBuf) -> Result<&SourceFile, SourceError> {
        // Check if file is already loaded
        if let Some(index) = self.files.iter().position(|f| f.path == path) {
            return Ok(&self.files[index]);
        }

        // Load new file
        let source_file = SourceFile::new(path)?;
        self.files.push(source_file);

        // Return reference to the loaded file
        Ok(&self.files.last().unwrap())
    }

    pub fn get_file(&self, path: &PathBuf) -> Option<&SourceFile> {
        self.files.iter().find(|f| f.path == *path)
    }

    pub fn get_files(&self) -> &[SourceFile] {
        &self.files
    }

    pub fn clear(&mut self) {
        self.files.clear();
    }

    pub fn set_max_files(&mut self, max: usize) {
        self.max_files = max;
    }
}

impl Default for SourceManager {
    fn default() -> Self {
        Self::new()
    }
}