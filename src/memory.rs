// src/memory_access.rs
use std::ptr::{read_volatile, write_volatile};

/// Wrapper for reading from a raw pointer.
pub fn read<T: Copy>(address: *const T) -> Result<T, &'static str> {
    if address.is_null() {
        Err("Null pointer dereference")
    } else {
        unsafe { Ok(read_volatile(address)) }
    }
}

/// Wrapper for writing to a raw pointer.
pub fn write<T: Copy>(address: *mut T, value: T) -> Result<(), &'static str> {
    if address.is_null() {
        Err("Null pointer dereference")
    } else {
        unsafe { write_volatile(address, value); }
        Ok(())
    }
}
