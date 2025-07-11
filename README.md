# Byte Lang

## Byte Lang currently supports only ARM64 on macOS. Support for x86 and other architectures will be implemented later.

**Byte Lang** is an emerging programming language designed to combine the low-level control of assembly language with the readability and structure of high-level languages. Currently in its early stages, Byte Lang offers a foundational set of commands and features aimed at high performance and clear, efficient programming.

## Features

- **Early Development**: Byte Lang is still evolving with a limited set of commands and features.
- **Low-Level Control**: Directly manipulate hardware and system resources with assembly-like syntax.
- **Readable Syntax**: A structured and human-friendly approach to low-level programming.

## Getting Started

### Installation

Since Byte Lang is in its early stages, the installation process involves cloning the repository and building from source:

1. **Download the compiler from our release page**:

2. **Navigate to the Directory**:
   ```bash
   cd byte-lang
   ```

3. **Run your first program**:
   ```bash
   ./byte-lang (command)
   ```

### Commands
* run (file location like example.txt)

### Example
```bash
number test_var = 0;
number test_var_2 = 10;

fn printstring() {
    compare(10, 10)
    .== {
        println("printing");
    };
}

loop(5) {
    printstring();
}

compare(10, test_var_2)
.== {
    println("equal");
}
.!= {
    println("not equal");
};

println("hello world");
term;
```
