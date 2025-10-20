//! Error handling interface for Qi runtime

use super::RuntimeError;

/// Error handling interface
pub struct ErrorInterface {
    initialized: bool,
    last_error: Option<String>,
}

impl ErrorInterface {
    pub fn new() -> Self {
        Self {
            initialized: false,
            last_error: None,
        }
    }

    pub fn initialize(&mut self) -> Result<(), RuntimeError> {
        if self.initialized {
            return Ok(());
        }

        // TODO: Initialize error handling
        self.initialized = true;
        self.last_error = None;
        Ok(())
    }

    pub fn set_error(&mut self, message: &str) -> Result<(), RuntimeError> {
        if !self.initialized {
            return Err(RuntimeError::Panic("错误处理未初始化".to_string()));
        }

        self.last_error = Some(message.to_string());
        Ok(())
    }

    pub fn get_last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    pub fn clear_error(&mut self) -> Result<(), RuntimeError> {
        if !self.initialized {
            return Err(RuntimeError::Panic("错误处理未初始化".to_string()));
        }

        self.last_error = None;
        Ok(())
    }

    pub fn panic(&self, message: &str) -> Result<(), RuntimeError> {
        if !self.initialized {
            return Err(RuntimeError::Panic("错误处理未初始化".to_string()));
        }

        // TODO: Implement panic handling
        Err(RuntimeError::Panic(message.to_string()))
    }

    pub fn assert(&self, condition: bool, message: &str) -> Result<(), RuntimeError> {
        if !condition {
            return self.panic(&format!("断言失败: {}", message));
        }
        Ok(())
    }

    pub fn unwrap<T>(&self, value: Option<T>, message: &str) -> Result<T, RuntimeError> {
        match value {
            Some(v) => Ok(v),
            None => {
                self.panic(&format!("unwrap 失败: {}", message))?;
                unreachable!()
            }
        }
    }

    pub fn expect<T>(&self, value: Option<T>, message: &str) -> Result<T, RuntimeError> {
        self.unwrap(value, message)
    }

    pub fn cleanup(&mut self) -> Result<(), RuntimeError> {
        if self.initialized {
            // TODO: Cleanup error handling
            self.last_error = None;
            self.initialized = false;
        }
        Ok(())
    }
}

impl Default for ErrorInterface {
    fn default() -> Self {
        Self::new()
    }
}