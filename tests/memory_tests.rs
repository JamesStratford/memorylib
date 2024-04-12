// tests/memory_access_tests.rs

#[cfg(test)]
mod tests {
    use memory_editor::memory;

    #[test]
    fn test_read_null() {
        let null_ptr: *const u8 = std::ptr::null();
        assert_eq!(memory::read::<u8>(null_ptr), Err("Null pointer dereference"));
    }

    #[test]
    fn test_write_null() {
        let null_ptr: *mut u8 = std::ptr::null_mut();
        assert_eq!(memory::write::<u8>(null_ptr, 255), Err("Null pointer dereference"));
    }

    #[test]
    fn test_read_write_valid_memory() {
        let mut value: u8 = 0;
        let ptr = &mut value as *mut u8;
        assert_eq!(memory::write::<u8>(ptr, 123), Ok(()));
        assert_eq!(memory::read::<u8>(ptr), Ok(123));
    }

    #[test]
    fn test_read_from_heap_allocated_variable() {
        // Allocate a value on the heap
        let heap_value: Box<u8> = Box::new(42); // Heap-allocated integer
        let heap_value_ptr = Box::into_raw(heap_value);

        // Read the value from the heap using the unsafe function
        let read_value: u8;
        read_value = memory::read::<u8>(heap_value_ptr).unwrap();

        // Check if the values match
        assert_eq!(read_value, 42);
        assert_eq!(unsafe { *heap_value_ptr }, 42);

        // Cleanup
        unsafe {
            drop(Box::from_raw(heap_value_ptr));
        }
    }

    #[test]
    fn test_write_to_heap_allocated_variable() {
        // Allocate a value on the heap
        let heap_value: Box<u8> = Box::new(0); // Heap-allocated integer
        let heap_value_ptr = Box::into_raw(heap_value);
        assert_eq!(unsafe { *heap_value_ptr }, 0);

        // Read the value from the heap using the unsafe function
        let success = memory::write::<u8>(heap_value_ptr, 8);

        // Check if the values match
        assert_eq!(success, Ok(()));
        assert_eq!(unsafe { *heap_value_ptr }, 8);

        // Cleanup
        unsafe {
            drop(Box::from_raw(heap_value_ptr));
        }
    }

    #[test]
    fn write_to_immutable_variable() {
        let value = 0;
        let ptr = &value as *const i32;
        let _ = memory::write::<i32>(ptr as *mut i32, 42);
        assert_eq!(value, 42);
    }
}
