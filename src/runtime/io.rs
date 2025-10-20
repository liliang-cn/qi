//! I/O operations interface for Qi runtime

use super::RuntimeError;

/// I/O operations interface
pub struct IoInterface {
    initialized: bool,
}

impl IoInterface {
    pub fn new() -> Self {
        Self {
            initialized: false,
        }
    }

    pub fn initialize(&mut self) -> Result<(), RuntimeError> {
        if self.initialized {
            return Ok(());
        }

        // Initialize I/O operations
        self.initialized = true;
        Ok(())
    }

    /// Print a string to stdout
    pub fn print(&self, message: &str) -> Result<(), RuntimeError> {
        if !self.initialized {
            return Err(RuntimeError::Panic("I/O 操作未初始化".to_string()));
        }

        println!("{}", message);
        Ok(())
    }

    /// Print a string to stdout with newline
    pub fn println(&self, message: &str) -> Result<(), RuntimeError> {
        if !self.initialized {
            return Err(RuntimeError::Panic("I/O 操作未初始化".to_string()));
        }

        println!("{}", message);
        Ok(())
    }

    /// Print an integer to stdout
    pub fn print_int(&self, value: i64) -> Result<(), RuntimeError> {
        if !self.initialized {
            return Err(RuntimeError::Panic("I/O 操作未初始化".to_string()));
        }

        print!("{}", value);
        Ok(())
    }

    /// Print an integer to stdout with newline
    pub fn println_int(&self, value: i64) -> Result<(), RuntimeError> {
        if !self.initialized {
            return Err(RuntimeError::Panic("I/O 操作未初始化".to_string()));
        }

        println!("{}", value);
        Ok(())
    }

    /// Print a float to stdout
    pub fn print_float(&self, value: f64) -> Result<(), RuntimeError> {
        if !self.initialized {
            return Err(RuntimeError::Panic("I/O 操作未初始化".to_string()));
        }

        print!("{}", value);
        Ok(())
    }

    /// Print a float to stdout with newline
    pub fn println_float(&self, value: f64) -> Result<(), RuntimeError> {
        if !self.initialized {
            return Err(RuntimeError::Panic("I/O 操作未初始化".to_string()));
        }

        println!("{}", value);
        Ok(())
    }

    /// Read a line from stdin
    pub fn read_line(&self) -> Result<String, RuntimeError> {
        if !self.initialized {
            return Err(RuntimeError::Panic("I/O 操作未初始化".to_string()));
        }

        use std::io::{self, BufRead};
        let stdin = io::stdin();
        let mut line = String::new();

        stdin.lock().read_line(&mut line)
            .map_err(|e| RuntimeError::Panic(format!("读取输入失败: {}", e)))?;

        // Remove trailing newline
        if line.ends_with('\n') {
            line.pop();
            if line.ends_with('\r') {
                line.pop();
            }
        }

        Ok(line)
    }

    pub fn cleanup(&mut self) -> Result<(), RuntimeError> {
        if self.initialized {
            // Cleanup I/O operations
            self.initialized = false;
        }
        Ok(())
    }
}

impl Default for IoInterface {
    fn default() -> Self {
        Self::new()
    }
}