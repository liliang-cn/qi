//! C runtime library integration

pub mod memory;
pub mod strings;
pub mod errors;
pub mod io;

/// Runtime library interface
pub struct RuntimeLibrary {
    memory_interface: memory::MemoryInterface,
    string_interface: strings::StringInterface,
    error_interface: errors::ErrorInterface,
    io_interface: io::IoInterface,
}

impl RuntimeLibrary {
    /// Create a new runtime library interface
    pub fn new() -> Self {
        Self {
            memory_interface: memory::MemoryInterface::new(),
            string_interface: strings::StringInterface::new(),
            error_interface: errors::ErrorInterface::new(),
            io_interface: io::IoInterface::new(),
        }
    }

    /// Initialize the runtime library
    pub fn initialize(&mut self) -> Result<(), RuntimeError> {
        self.memory_interface.initialize()?;
        self.string_interface.initialize()?;
        self.error_interface.initialize()?;
        self.io_interface.initialize()?;
        Ok(())
    }

    /// Get memory management interface
    pub fn memory(&self) -> &memory::MemoryInterface {
        &self.memory_interface
    }

    /// Get mutable memory management interface
    pub fn memory_mut(&mut self) -> &mut memory::MemoryInterface {
        &mut self.memory_interface
    }

    /// Get string operations interface
    pub fn strings(&self) -> &strings::StringInterface {
        &self.string_interface
    }

    /// Get error handling interface
    pub fn errors(&self) -> &errors::ErrorInterface {
        &self.error_interface
    }

    /// Get I/O operations interface
    pub fn io(&self) -> &io::IoInterface {
        &self.io_interface
    }
}

/// Runtime errors
#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    /// Memory allocation error
    #[error("内存分配错误: {0}")]
    MemoryAllocation(String),

    /// String operation error
    #[error("字符串操作错误: {0}")]
    StringOperation(String),

    /// Runtime panic
    #[error("运行时错误: {0}")]
    Panic(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_library_initialization() {
        let mut runtime = RuntimeLibrary::new();
        assert!(runtime.initialize().is_ok());
    }

    #[test]
    fn test_memory_operations() {
        let mut runtime = RuntimeLibrary::new();
        runtime.initialize().unwrap();

        let memory = runtime.memory();
        assert_eq!(memory.get_allocated_bytes(), 0);
    }

    #[test]
    fn test_string_operations() {
        let mut runtime = RuntimeLibrary::new();
        runtime.initialize().unwrap();

        let strings = runtime.strings();

        // Test string length
        assert_eq!(strings.length("你好").unwrap(), 2);
        assert_eq!(strings.length("Hello").unwrap(), 5);

        // Test string concatenation
        assert_eq!(strings.concat("你好", "世界").unwrap(), "你好世界");

        // Test string comparison
        assert_eq!(strings.compare("你好", "你好").unwrap(), 0);
        let result = strings.compare("你好", "世界").unwrap();
        assert!(result != 0, "Comparison should not be equal");
    }

    #[test]
    fn test_io_operations() {
        let mut runtime = RuntimeLibrary::new();
        runtime.initialize().unwrap();

        let io = runtime.io();

        // Test printing (should not panic)
        assert!(io.print("Hello").is_ok());
        assert!(io.println_int(42).is_ok());
        assert!(io.println_float(3.14).is_ok());
    }

    #[test]
    fn test_memory_allocation() {
        let mut runtime = RuntimeLibrary::new();
        runtime.initialize().unwrap();

        let memory = runtime.memory_mut();

        // Test allocation (using unsafe for testing)
        let ptr = memory.allocate(1024);
        assert!(ptr.is_ok());

        if let Ok(allocated_ptr) = ptr {
            assert_eq!(memory.get_allocated_bytes(), 1024);

            // Test deallocation
            assert!(memory.deallocate(allocated_ptr, 1024).is_ok());
            assert_eq!(memory.get_allocated_bytes(), 0);
        }
    }
}