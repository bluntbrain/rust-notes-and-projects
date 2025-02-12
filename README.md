# Rust Language Notes
My notes on the Rust language.

---

## Cargo Commands

Cargo is Rust's build system and package manager. Here are some essential commands:

- **Create a new project:**
  ```bash
  cargo new my_project
  ```
- **Build the project:**
  ```bash
  cargo build
  ```
- **Run the project:**
  ```bash
  cargo run
  ```
- **Test the project:**
  ```bash
  cargo test
  ```
- **Generate documentation:**
  ```bash
  cargo doc --open
  ```
- **Format code:**
  ```bash
  cargo fmt
  ```
- **Lint the code:**
  ```bash
  cargo clippy
  ```
- **Add a dependency (requires Cargo-edit):**
  ```bash
  cargo add <dependency>
  ```

---

## Introduction

Rust is a modern, statically typed systems programming language that emphasizes performance, safety, and concurrency. Its unique ownership model ensures memory safety without a garbage collector, making it an excellent choice for systems-level and application programming.

---

## Getting Started

### Installation

- **Official Website:** [Rust Installation](https://www.rust-lang.org/tools/install)
- **Installation Command (Unix-based systems):**
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Your First Rust Program (Hello World)

Create a new project:
```bash
cargo new hello_world
cd hello_world
```

Then, edit the `src/main.rs` file to contain:
```rust
fn main() {
    println!("Hello, world!");
}
```

Run the project with:
```bash
cargo run
```

---

## Basic Concepts

### Variables and Mutability

By default, variables in Rust are immutable. Use the `mut` keyword to declare a mutable variable.
```rust
fn main() {
    let x = 5;
    // x = 6; // Error: cannot assign twice to immutable variable

    let mut y = 5;
    y = 6; // Correct: y is mutable
}
```

### Data Types

Rust is statically typed with powerful type inference. Key types include:

**Scalars:**

1. **Integers:**
   ```rust
   // Signed integers
   let i8_num: i8 = -128;            // -128 to 127
   let i16_num: i16 = 32_767;        // -32,768 to 32,767
   let i32_num: i32 = -2_147_483_648;// -2^31 to 2^31 - 1
   let i64_num: i64 = 9_223_372_036; // -2^63 to 2^63 - 1
   let i128_num: i128 = 170_141_183; // -2^127 to 2^127 - 1
   
   // Unsigned integers
   let u8_num: u8 = 255;             // 0 to 255
   let u16_num: u16 = 65_535;        // 0 to 65,535
   let u32_num: u32 = 4_294_967_295; // 0 to 2^32 - 1
   let u64_num: u64 = 18_446_744_073;// 0 to 2^64 - 1
   let u128_num: u128 = 340_282_366; // 0 to 2^128 - 1
   
   // Architecture-dependent integers
   let isize_num: isize = -123;      // Depends on system architecture
   let usize_num: usize = 123;       // Depends on system architecture
   ```

2. **Floating-point numbers:**
   ```rust
   let f32_num: f32 = 3.14159;       // Single precision
   let f64_num: f64 = 3.14159265359; // Double precision
   
   // Scientific notation
   let scientific1 = 2.5e-3;         // 0.0025
   let scientific2 = 1.23e4;         // 12300.0
   ```

3. **Booleans:**
   ```rust
   let is_active: bool = true;
   let is_greater = 10 > 5;          // Type inference
   
   // Boolean operations
   let and_result = true && false;   // false
   let or_result = true || false;    // true
   let not_result = !true;           // false
   ```

4. **Characters:**
   ```rust
   let letter: char = 'A';           // Single quotes for char
   let emoji: char = '😀';           // Unicode support
   let symbol: char = '∞';           // Special symbols
   let escape_char = '\n';           // Newline character
   
   // Unicode escape sequences
   let unicode_char = '\u{1F600}';   // 😀
   ```

**Compound Types:**
- **Tuples** and **Arrays**
```rust
fn main() {
    let tuple: (i32, f64) = (500, 6.4);
    let emp_info: (&str,u8) = ("John", 30);
    let emp_name = emp_info.0;
    let emp_age = emp_info.1;
    println!("Employee name: {}, age: {}", emp_name, emp_age);

    // destructuring 
    let (employee_name, employee_age) = emp_info;
    println!("Employee name: {}, age: {}", employee_name, employee_age);

    let array: [i32; 3] = [1, 2, 3];
}
```

### Strings

Rust has two main string types: `String` and `&str`.

1. **String Slice (`&str`):**
   ```rust
   // String literals are &str by default
   let hello: &str = "Hello, world!";
   
   // String slices are immutable references
   let slice: &str = &hello[0..5]; // "Hello"
   
   // Multi-line string literals
   let multi_line = "This is a
   multi-line string";
   
   // Raw string literals (no escape characters)
   let raw_string = r"C:\Program Files\Rust";
   ```

2. **String Type (`String`):**
   ```rust
   // Creating new String
   let mut string = String::new();
   let string_from = String::from("Hello");
   let to_string = "Hello".to_string();
   
   // Modifying String
   let mut s = String::from("Hello");
   s.push_str(", world!");    // Append string slice
   s.push('!');               // Append single character
   
   // Concatenation
   let s1 = String::from("Hello, ");
   let s2 = String::from("world!");
   let s3 = s1 + &s2;        // Note: s1 is moved here
   
   // Format macro
   let s = format!("{}-{}-{}", "Hello", "world", "!");
   ```

3. **Converting Between `String` and `&str`:**
   ```rust
   // &str to String
   let owned: String = "hello".to_string();
   let also_owned = String::from("hello");
   
   // String to &str
   let string = String::from("hello");
   let borrowed: &str = &string;
   let slice: &str = string.as_str();
   ```

4. **String Operations:**
   ```rust
   let mut s = String::from("Hello, world!");
   
   // Length
   let len = s.len();                // 13 bytes (not characters!)
   
   // Iteration
   for c in s.chars() {              // Iterate over characters
       println!("{}", c);
   }
   
   for b in s.bytes() {              // Iterate over bytes
       println!("{}", b);
   }
   
   // Substring
   let hello = &s[0..5];             // "Hello"
   
   // Clear
   s.clear();                        // Empty the string
   
   // Check if empty
   let is_empty = s.is_empty();      // true
   ```

5. **Unicode and UTF-8:**
   ```rust
   // UTF-8 encoded by default
   let hello = String::from("Hello, 世界");
   
   // Length in bytes vs chars
   println!("Bytes: {}", hello.len());           // 13
   println!("Chars: {}", hello.chars().count()); // 8
   
   // Individual Unicode characters
   let char_indices: Vec<(usize, char)> = hello.char_indices().collect();
   // [(0, 'H'), (1, 'e'), (2, 'l'), (3, 'l'), (4, 'o'), 
   //  (5, ','), (6, ' '), (7, '世'), (10, '界')]
   ```

6. **Common Methods:**
   ```rust
   let mut s = String::from("Hello, world!");
   
   // Transformations
   let uppercase = s.to_uppercase();
   let lowercase = s.to_lowercase();
   
   // Trimming
   let trimmed = s.trim();           // Remove whitespace
   let trim_start = s.trim_start();  // Remove leading whitespace
   let trim_end = s.trim_end();      // Remove trailing whitespace
   
   // Replace
   let replaced = s.replace("Hello", "Hi");
   
   // Split
   let words: Vec<&str> = s.split_whitespace().collect();
   let parts: Vec<&str> = s.split(',').collect();
   ```

**Note:** String operations in Rust are designed with UTF-8 encoding in mind, making them safe for international text but requiring careful handling when working with individual characters or slices.

### Functions

Define functions using the `fn` keyword. Rust allows implicit returns by omitting the semicolon on the last expression.
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Control Flow

#### Conditionals:
```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("number is less than 5");
    } else {
        println!("number is 5 or greater");
    }
}
```

#### Loops:

- **Infinite Loop using `loop`:**
  ```rust
  fn main() {
      let mut count = 0;
      loop {
          count += 1;
          if count == 5 {
              break;
          }
      }
  }
  ```

- **`while` Loop:**
  ```rust
  fn main() {
      let mut x = 5;
      while x > 0 {
          println!("{}", x);
          x -= 1;
      }
  }
  ```
  
- **`for` Loop:**
  ```rust
  fn main() {
      let a = [10, 20, 30];
      for element in a.iter() {
          println!("{}", element);
      }
  }
  ```

---

## Ownership, Borrowing, and Slices

Rust's ownership system enforces memory safety at compile time.

### Ownership
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // Ownership is moved from s1 to s2
    // s1 is no longer valid here
}
```

### Borrowing

Borrow data using references to avoid transferring ownership.
```rust
fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s); // Borrowing s
    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

### Slices

Slices allow you to reference a contiguous sequence of elements within a collection.
```rust
fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];  // Slices the string from index 0 to 4
    let world = &s[6..11]; // Slices the string from index 6 to 10

    println!("{} {}", hello, world);
}
```

---

## Structs and Enums

### Structs

Define custom data types using `struct` to group related data.
```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
        active: true,
    };
}
```

### Enums

Enums allow you to define a type by enumerating its possible variants.
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Write(String::from("Hello"));
}
```

---

## Pattern Matching

Use `match` to handle different values and control flow based on pattern matching.
```rust
fn main() {
    let number = 3;
    
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other"),
    }
}
```

---

## Modules and Crates

Modules help you organize code into namespaces, while crates are the compilation units (libraries or executables).

### Example Module
```rust
// In src/lib.rs
pub mod greetings {
    pub fn hello() {
        println!("Hello, world!");
    }
}
```

### Using the Module
```rust
// In src/main.rs
use mycrate::greetings;

fn main() {
    greetings::hello();
}
```

---

## Traits

Traits define shared behavior, similar to interfaces in other languages.
```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    headline: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.headline, &self.content[0..10])
    }
}
```

---

## Generics

Generics allow you to write flexible and reusable functions and types.
```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

---

## Collections

### Vectors

Vectors are dynamically growable arrays.
```rust
fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
}
```

### HashMaps

HashMaps store key-value pairs.
```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
```

---

## Error Handling

Rust uses `Result` and `Option` types for error management, promoting robust handling of potential errors.

### Handling Errors with `Result`
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}
```

### Using `Option`
```rust
fn main() {
    let some_number: Option<i32> = Some(5);
    if let Some(x) = some_number {
        println!("Number: {}", x);
    }
}
```

---

## Smart Pointers

Rust offers smart pointers that provide additional capabilities beyond regular references:

- **Box<T>:** For heap allocation.
- **Rc<T>:** For multiple ownership in single-threaded scenarios.
- **RefCell<T>:** For interior mutability.

Example using `Box<T>`:
```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

---

## Concurrency

Rust's concurrency model prevents data races by enforcing strict ownership rules, making it safe to run threads concurrently.
```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread!", i);
        }
    });

    for i in 1..5 {
        println!("hi number {} from main thread!", i);
    }

    handle.join().unwrap();
}
```

---

## Macros

Macros enable metaprogramming in Rust and can reduce repetitive code. Rust includes many built-in macros like `println!`, `vec!`, etc.
```rust
#[macro_export]
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!();
}
```

---

## Testing

Write tests in Rust using the `#[test]` attribute. Run tests with `cargo test`.
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

---

## Documentation and Comments

Document your code using triple-slash comments (`///`). These comments are included in the generated documentation.
```rust
/// Adds two numbers.
///
/// # Examples
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

---

## Debugging

- **Print Debug Information:**
  ```rust
  println!("{:?}", var);
  ```
- **Enable Backtraces:**
  ```bash
  export RUST_BACKTRACE=1
  ```

---

## Further Resources

- [The Rust Programming Language (The Book)](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Documentation](https://www.rust-lang.org/learn)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
- [Rust Playground](https://play.rust-lang.org/)

---
