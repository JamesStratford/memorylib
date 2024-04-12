// src/lib.rs

pub mod memory;

#[no_mangle]
pub extern "C" fn read_memory(address: *const u8, buffer: *mut u8, size: usize) {
    for i in 0..size {
        unsafe {
            *buffer.add(i) = match memory::read::<u8>(address.add(i)) {
                Ok(value) => value,
                Err(_) => 0,
            };
        }
    }
}

#[no_mangle]
pub extern "C" fn write_memory(address: *mut u8, buffer: *const u8, size: usize) {
    for i in 0..size {
        let _ = memory::write::<u8>(unsafe { address.add(i) }, unsafe { *buffer.add(i) });
    }
}
