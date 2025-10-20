//! Memory management interface for Qi runtime

use super::RuntimeError;

/// Memory management interface
pub struct MemoryInterface {
    initialized: bool,
    allocated_bytes: usize,
}

impl MemoryInterface {
    pub fn new() -> Self {
        Self {
            initialized: false,
            allocated_bytes: 0,
        }
    }

    pub fn initialize(&mut self) -> Result<(), RuntimeError> {
        if self.initialized {
            return Ok(());
        }

        // TODO: Initialize memory management
        self.initialized = true;
        Ok(())
    }

    pub fn allocate(&mut self, size: usize) -> Result<*mut u8, RuntimeError> {
        if !self.initialized {
            return Err(RuntimeError::MemoryAllocation("内存管理未初始化".to_string()));
        }

        if size == 0 {
            return Err(RuntimeError::MemoryAllocation("无法分配零字节内存".to_string()));
        }

        unsafe {
            let ptr = std::alloc::alloc(std::alloc::Layout::from_size_align(size, 8).unwrap());
            if ptr.is_null() {
                Err(RuntimeError::MemoryAllocation("内存分配失败".to_string()))
            } else {
                self.allocated_bytes += size;
                Ok(ptr)
            }
        }
    }

    pub fn reallocate(&mut self, ptr: *mut u8, new_size: usize, old_size: usize) -> Result<*mut u8, RuntimeError> {
        if !self.initialized {
            return Err(RuntimeError::MemoryAllocation("内存管理未初始化".to_string()));
        }

        if new_size == 0 {
            return Err(RuntimeError::MemoryAllocation("无法重新分配零字节内存".to_string()));
        }

        unsafe {
            let new_ptr = std::alloc::realloc(
                ptr,
                std::alloc::Layout::from_size_align(old_size, 8).unwrap(),
                new_size
            );
            if new_ptr.is_null() {
                Err(RuntimeError::MemoryAllocation("内存重新分配失败".to_string()))
            } else {
                self.allocated_bytes = self.allocated_bytes - old_size + new_size;
                Ok(new_ptr)
            }
        }
    }

    pub fn deallocate(&mut self, ptr: *mut u8, size: usize) -> Result<(), RuntimeError> {
        if !self.initialized {
            return Err(RuntimeError::MemoryAllocation("内存管理未初始化".to_string()));
        }

        if ptr.is_null() {
            return Ok(());
        }

        unsafe {
            std::alloc::dealloc(ptr, std::alloc::Layout::from_size_align(size, 8).unwrap());
            self.allocated_bytes = self.allocated_bytes.saturating_sub(size);
        }
        Ok(())
    }

    pub fn get_allocated_bytes(&self) -> usize {
        self.allocated_bytes
    }

    pub fn cleanup(&mut self) -> Result<(), RuntimeError> {
        if self.initialized {
            // TODO: Cleanup memory management
            self.initialized = false;
        }
        Ok(())
    }
}

impl Default for MemoryInterface {
    fn default() -> Self {
        Self::new()
    }
}