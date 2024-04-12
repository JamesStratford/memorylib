# MemoryLib

MemoryLib is a Rust library designed to facilitate direct memory reading and writing operations. It provides low-level access to memory, making it suitable for tasks such as game hacking, reverse engineering, and any scenario where direct memory manipulation is needed.

## Features

- **Direct Memory Access**: Perform read and write operations directly on memory.
- **Safe Wrappers**: Exposes unsafe operations through safer interfaces where possible.
- **C Bindings**: Includes extern C functions for integration with other programming languages.

## Functions

MemoryLib includes the following key extern C functions:

- `read_memory`: Reads bytes from a specified memory address into a buffer.
- `write_memory`: Writes bytes from a buffer to a specified memory address.

```c
void read_memory(const uint8_t* address, uint8_t* buffer, size_t size);
void write_memory(uint8_t* address, const uint8_t* buffer, size_t size);
```

## Usage Example
```c
uint8_t data[] = {1, 2, 3, 4, 5};
write_memory((uint8_t*)0x0040F000, data, 5);

uint8_t buffer[5];
read_memory((const uint8_t*)0x0040F000, buffer, 5);
```
## Getting Started
To integrate MemoryLib into your own applications, follow these steps:

### Prerequisites
Ensure you have Rust and Cargo installed on your machine.

### Building
Clone the repository and build the library using Cargo:
```bash
git clone https://github.com/JamesStratford/memorylib.git
cd memorylib
cargo build --release
```

## License
MemoryLib is released under the MIT License. See the LICENSE file in the repository for more details.