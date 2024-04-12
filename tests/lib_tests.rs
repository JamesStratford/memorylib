//tests/lis_tests.rs

#[cfg(test)]
mod tests {
    use memorylib;

    #[test]
    fn test_read_memory() {
        let mut buffer = vec![0u8; 5]; // buffer to hold read values
        let expected_values = vec![1, 2, 3, 4, 5];
        
        // Read values from the memory
        memorylib::read_memory(expected_values.as_ptr(), buffer.as_mut_ptr(), 5);

        // Check if the values match
        assert_eq!(buffer, expected_values);
    }

    #[test]
    fn test_write_memory() {
        let buffer = vec![1, 2, 3, 4, 5]; // buffer to write to memory
        let mut values = vec![0u8; 5]; // buffer to hold read values
        
        // Write values to the memory
        memorylib::write_memory(values.as_mut_ptr(), buffer.as_ptr(), 5);

        // Check if the values match
        assert_eq!(values, buffer);
    }
}