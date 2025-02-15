# Rust Notes and Projects

## Projects
1. [Guessing Game](guessing_game/src/game.rs) - A simple fruit guessing game using random selection
2. [Tic Tac Toe](tic_tac_toe/src/game.rs) - A command-line implementation of Tic Tac Toe

## Table of Contents
- [Introduction & Setup](#introduction--setup)
- [Basic Concepts](#basic-concepts)
  - [Variables and Mutability](#variables-and-mutability)
  - [Shadowing](#shadowing)
  - [Data Types](#data-types)
  - [Control Flow](#control-flow)
  - [Functions](#functions)
- [Memory Management](#memory-management-fundamentals)
- [Ownership System](#ownership-system)
- [References & Borrowing](#references--borrowing)
- [Collections](#collections)
  - [Vectors](#vectors-vec)
  - [Strings](#strings)
  - [HashMaps](#hashmaps)
- [Compound Types](#compound-types)
  - [Structs](#structs)
  - [Enums](#enums)
- [Pattern Matching](#pattern-matching)
- [Error Handling](#error-handling)
- [Generic Types & Traits](#generic-types--traits)
- [Modules & Project Organization](#modules--project-organization)
- [Concurrency](#concurrency)
- [Testing](#testing)
- [Smart Pointers](#smart-pointers)
- [Macros](#macros)

---

## Introduction & Setup

### What is Rust?
Rust is a systems programming language that focuses on:
- Memory safety without garbage collection
- Concurrency without data races
- Zero-cost abstractions
- Performance

### Installation
```bash
# Unix-based systems
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

### Essential Cargo Commands
```bash
cargo new project_name    # Create new project
cargo build              # Compile project
cargo run               # Run project
cargo test              # Run tests
cargo check             # Check if code compiles
cargo doc --open        # Generate and view documentation
```

### Your First Program
```rust
// src/main.rs
fn main() {
    println!("Hello, Rust!");
}
```

---

## Basic Concepts

### Variables and Mutability
> Summary: Variables are immutable by default. Use `mut` to make them mutable.

```rust
fn main() {
    let x = 5;          // Immutable
    // x = 6;           // Error!
    
    let mut y = 5;      // Mutable
    y = 6;              // OK
    
    // Constants
    const MAX_POINTS: u32 = 100_000;
}
```

### Shadowing
> Summary: A new variable can be declared with the same name as a previous variable, effectively shadowing it.

```rust
fn main() {
    // Basic shadowing
    let x = 5;
    let x = x + 1;        // Creates new variable, shadows previous x
    let x = x * 2;        // Creates another new variable
    println!("x is: {}", x);  // Prints: x is: 12
    
    // Shadowing with different types
    let spaces = "   ";           // String type
    let spaces = spaces.len();    // Number type
    
    // Shadowing in different scopes
    let y = 5;
    {
        let y = 12;      // Only shadows within this scope
        println!("Inner y: {}", y);  // Prints: Inner y: 12
    }
    println!("Outer y: {}", y);  // Prints: Outer y: 5
    
    // Shadowing vs mut
    let mut z = 5;
    // let z = "hello";  // OK with shadowing
    // z = "hello";      // Error! Can't change type with mut
}
```

#### Key Differences from mut
1. Shadowing creates a new variable
2. Can change type
3. Maintains immutability
4. Scope-based

```rust
fn main() {
    // With shadowing - OK
    let data = "   ";
    let data = data.len();
    
    // With mut - Error!
    let mut value = "   ";
    // value = value.len();  // Type mismatch error
    
    // Shadowing preserves immutability
    let number = 5;
    let number = number + 5;  // Creates new immutable binding
    // number += 1;          // Error! number is immutable
}
```

### Data Types

#### Scalar Types
1. **Integers**
```rust
let i: i32 = -42;       // Signed 32-bit integer
let u: u32 = 42;        // Unsigned 32-bit integer
let b: i8 = -128;       // Signed 8-bit integer
let big: i128 = 123;    // Signed 128-bit integer
```

2. **Floating-Point**
```rust
let f: f64 = 3.14159;   // 64-bit float (default)
let f: f32 = 3.14;      // 32-bit float
```

3. **Boolean**
```rust
let t: bool = true;
let f: bool = false;
```

4. **Character**
```rust
let c: char = 'z';
let emoji: char = '😀';  // Unicode character
```

### Strings and String Slices
> Summary: Rust has two main string types: String (owned) and &str (borrowed string slice).

```rust
fn main() {
    // String type (owned, growable)
    let mut s = String::from("hello");  // Heap allocated
    s.push_str(", world!");            // Can modify
    println!("{}", s);
    
    // String literal (&str type)
    let s1: &str = "hello world";      // Fixed size, immutable
    
    // String slices
    let hello: &str = &s[0..5];        // Slice of String
    let world: &str = &s[7..12];       // Another slice
    
    // Converting between String and &str
    let owned: String = "hello".to_string();  // &str to String
    let owned: String = String::from("hello"); // Another way
    let borrowed: &str = &owned;              // String to &str
    
    // String concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // s1 is moved here, s2 is borrowed
    
    // Format macro
    let s = format!("{}-{}-{}", "Hello", "world", "!");
}
```

#### Key Differences
```rust
fn main() {
    // Memory allocation
    let s1 = String::from("hello");  // Heap allocated
    let s2 = "hello";                // Stored in binary
    
    // Mutability
    let mut s = String::from("hello");
    s.push_str(" world");           // OK with String
    // "hello".push_str(" world");  // Error with &str
    
    // Function parameters
    fn takes_slice(s: &str) {       // Can accept both String and &str
        println!("{}", s);
    }
    
    fn takes_string(s: String) {    // Only accepts String
        println!("{}", s);
    }
    
    // Common operations
    let len = "hello".len();        // Length
    let chars = "hello".chars();    // Iterator over chars
    let bytes = "hello".as_bytes(); // As byte slice
    
    // UTF-8
    let hello = String::from("Здравствуйте");
    let s = &hello[0..2];  // Takes 2 bytes (1 character)
}
```

#### Best Practices
```rust
// Use &str for function parameters
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Use String when you need to own and modify
fn build_greeting() -> String {
    let mut greeting = String::from("Hello");
    greeting.push_str(", world!");
    greeting
}

// String interning
fn main() {
    let s1: &str = "Hello";  // Single allocation in binary
    let s2: &str = "Hello";  // Points to same memory
    assert!(std::ptr::eq(s1, s2));
}
```

### Control Flow

#### Conditionals
```rust
fn main() {
    let number = 6;

    // If expression
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4 or 3");
    }

    // If in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
}
```

#### Loops
```rust
fn main() {
    // Loop (infinite)
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    // While loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    // For loop
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
```

### Functions
```rust
fn main() {
    println!("Sum: {}", add(5, 6));
}

fn add(x: i32, y: i32) -> i32 {
    x + y  // Implicit return (no semicolon)
}

// Function with multiple parameters
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
```

---

## Memory Management

### Stack vs Heap Memory

| Characteristic | Stack | Heap |
|---------------|--------|-------|
| Speed | Faster (push/pop) | Slower (allocation/deallocation) |
| Size | Limited, fixed-size data | Dynamic, flexible size data |
| Organization | LIFO (Last In, First Out) | Unordered, accessed via pointers |
| Memory Management | Automatic | Manual (handled by ownership in Rust) |
| Data Types | Known size at compile time | Unknown size at compile time |
| Access Pattern | Direct, sequential | Random access via pointers |
| Common Uses | - Local variables<br>- Function parameters<br>- Fixed-size arrays | - Strings<br>- Vectors<br>- Boxes<br>- Other dynamic data |

#### Example
```rust
fn main() {
    // Stack allocation
    let x = 42;                    // Integer on stack
    let y = true;                  // Boolean on stack
    
    // Heap allocation
    let s = String::from("hello"); // String data on heap, pointer on stack
    let v = vec![1, 2, 3];        // Vector data on heap, pointer on stack
}
```

---

## Ownership System
> Summary: Ownership is Rust's most unique feature, ensuring memory safety at compile time without garbage collection.

### Core Ownership Rules
1. Each value has exactly one owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value is dropped

### Basic Ownership Examples
```rust
fn main() {
    // Simple ownership
    let s1 = String::from("hello");
    let s2 = s1;                // s1 is moved to s2
    // println!("{}", s1);      // Error! s1 is no longer valid
    println!("{}", s2);         // Works fine

    // Clone for deep copy
    let s3 = String::from("hello");
    let s4 = s3.clone();       // Creates a deep copy
    println!("{} {}", s3, s4); // Both valid
}
```

### Ownership with Functions
```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);        // s is moved into the function
    // println!("{}", s);      // Error! s is no longer valid

    let x = 5;
    makes_copy(x);            // i32 is Copy, so x is still valid
    println!("{}", x);        // Works fine
}

fn takes_ownership(string: String) {
    println!("{}", string);
}   // string is dropped here

fn makes_copy(integer: i32) {
    println!("{}", integer);
}   // integer goes out of scope (copy is dropped)
```

### Return Values and Scope
```rust
fn main() {
    let s1 = gives_ownership();         // Move return value into s1
    
    let s2 = String::from("hello");     // s2 comes into scope
    
    let s3 = takes_and_gives_back(s2);  // s2 moved into function
                                        // return value moved into s3
}

fn gives_ownership() -> String {
    String::from("yours")               // Return value moves out
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string                            // Return value moves out
}
```

---

## References & Borrowing
> Summary: References allow you to refer to a value without taking ownership.

### Basic References
```rust
fn main() {
    let s1 = String::from("hello");
    
    let len = calculate_length(&s1);    // Pass reference to s1
    println!("Length of '{}' is {}.", s1, len);  // s1 is still valid
}

fn calculate_length(s: &String) -> usize {
    s.len()
}   // s goes out of scope, but doesn't drop the value it refers to
```

### Mutable References
```rust
fn main() {
    let mut s = String::from("hello");
    
    change(&mut s);                     // Pass mutable reference
    println!("{}", s);                  // Prints: hello world
}

fn change(some_string: &mut String) {
    some_string.push_str(" world");
}
```

### Reference Rules
1. You can have either:
   - One mutable reference
   - Any number of immutable references
2. References must always be valid (no dangling references)

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;         // OK - immutable reference
    let r2 = &s;         // OK - multiple immutable references allowed
    println!("{} {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s;     // OK - no other references are active
    println!("{}", r3);
}
```

### Slice Type
> Summary: Slices let you reference a contiguous sequence of elements rather than the whole collection.

```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];  // Slice from index 0 to 4
    let world = &s[6..11]; // Slice from index 6 to 10
    
    println!("{} {}", hello, world);

    // String slice parameter
    let word = first_word(&s);
    println!("First word: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

---

## Collections
> Summary: Collections are data structures that can contain multiple values. The most common are Vec, String, and HashMap.

### Vectors (Vec<T>)
> Summary: A vector is a growable array type that can hold any type T.

#### Creating and Updating Vectors
```rust
fn main() {
    // Creating vectors
    let mut v1: Vec<i32> = Vec::new();        // Empty vector
    let v2 = vec![1, 2, 3];                   // Using vec! macro
    
    // Adding elements
    v1.push(1);
    v1.push(2);
    v1.push(3);
    
    // Accessing elements
    let third: &i32 = &v2[2];                 // Direct indexing (may panic)
    let safe_third: Option<&i32> = v2.get(2); // Safe access with get()
    
    // Using match with safe access
    match v2.get(2) {
        Some(third) => println!("Third element is {}", third),
        None => println!("No third element"),
    }
}
```

#### Iterating Over Vectors
```rust
fn main() {
    let mut v = vec![100, 32, 57];
    
    // Immutable iteration
    for i in &v {
        println!("{}", i);
    }
    
    // Mutable iteration
    for i in &mut v {
        *i += 50;  // Dereference to modify
    }
}
```

#### Vector Memory Management
```rust
fn main() {
    // Preallocating space
    let mut v = Vec::with_capacity(10);
    println!("Length: {}, Capacity: {}", v.len(), v.capacity());
    
    // Growing beyond capacity
    for i in 0..16 {
        v.push(i);
        println!("Length: {}, Capacity: {}", v.len(), v.capacity());
    }
}
```

### Strings
> Summary: Strings in Rust come in two forms: String and &str.

#### String vs &str
```rust
fn main() {
    // String type (owned)
    let mut s = String::from("hello");
    s.push_str(" world");    // Can modify String
    
    // String slice (&str)
    let s1: &str = "hello world";  // String literal - immutable
    let s2: &str = &s[..];         // Whole slice of String
    let s3: &str = &s[0..5];       // Partial slice
}
```

#### String Operations
```rust
fn main() {
    // Concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // Note: s1 has been moved here
    
    // Format macro
    let s = format!("{}-{}-{}", "Hello", "world", "!");
    
    // Iteration
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    
    for b in "hello".bytes() {
        println!("{}", b);
    }
}
```

### HashMaps
> Summary: HashMap<K, V> stores key-value pairs with O(1) lookup.

#### Basic HashMap Usage
```rust
use std::collections::HashMap;

fn main() {
    // Creating
    let mut scores = HashMap::new();
    
    // Inserting
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    // Accessing
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    
    // Iterating
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
```

#### Updating HashMap Values
```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    
    // Overwriting
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);  // Value is 25
    
    // Insert if key has no value
    scores.entry(String::from("Yellow")).or_insert(50);
    
    // Update based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}
```

---

## Compound Types
> Summary: Compound types can group multiple values into one type. The main compound types are structs and enums.

### Structs
> Summary: Structs are custom data types that let you package related values together.

#### Defining and Instantiating Structs
```rust
// Define a struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // Create an instance
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Modify a field (if instance is mutable)
    user1.email = String::from("newemail@example.com");
    
    // Create instance from other instance
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1  // Use remaining fields from user1
    };
}
```

#### Tuple Structs
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    // Access tuple struct fields
    println!("First value: {}", black.0);
}
```

#### Methods and Associated Functions
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function (like a static method)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    
    // Method
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Method that takes ownership
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    // Call method
    println!("Area: {}", rect1.area());
    
    // Call associated function
    let square = Rectangle::square(20);
}
```

### Enums
> Summary: Enums allow you to define a type by enumerating its possible variants.

#### Basic Enum Definition
```rust
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
}
```

#### Complex Enum Example
```rust
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },   // Anonymous struct
    Write(String),             // Single value
    ChangeColor(i32, i32, i32),// Tuple
}

impl Message {
    fn call(&self) {
        // Method implementation
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to {}, {}", x, y),
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to: {}, {}, {}", r, g, b),
        }
    }
}

fn main() {
    let msg = Message::Write(String::from("hello"));
    msg.call();
}
```

#### Option Enum
```rust
fn main() {
    // Some value
    let some_number = Some(5);
    let some_string = Some("a string");
    
    // No value
    let absent_number: Option<i32> = None;
    
    // Working with Option
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    
    // Must handle the None case
    match y {
        None => println!("No value"),
        Some(i) => println!("Value is {}", i),
    }
    
    // Or use if let
    if let Some(value) = y {
        println!("Value is {}", value);
    }
}
```

---

## Pattern Matching
> Summary: Pattern matching is a powerful feature for checking and destructuring values.

### Match Expression
```rust
fn main() {
    let number = 13;
    
    match number {
        // Match a single value
        1 => println!("One!"),
        
        // Match multiple values
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),
        
        // Match a range
        13..=19 => println!("A teen"),
        
        // Default case
        _ => println!("Ain't special"),
    }
}
```

### Pattern Types
```rust
fn main() {
    // Matching literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("anything"),
    }
    
    // Matching named variables
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }
    
    // Matching multiple patterns
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
```

### Destructuring
```rust
struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

fn main() {
    // Destructuring structs
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    println!("a = {}, b = {}", a, b);
    
    // Shorter struct destructuring
    let Point { x, y } = p;
    println!("x = {}, y = {}", x, y);
    
    // Destructuring enums
    let color = Color::Rgb(0, 160, 255);
    match color {
        Color::Rgb(r, g, b) => {
            println!("Red: {}, green: {}, blue: {}", r, g, b);
        }
        Color::Hsv(h, s, v) => {
            println!("Hue: {}, saturation: {}, value: {}", h, s, v);
        }
    }
}
```

### Match Guards
```rust
fn main() {
    let num = Some(4);
    
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
    
    // Multiple conditions
    let x = 4;
    let y = false;
    
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}
```

### if let Expression
```rust
fn main() {
    let config_max = Some(3u8);
    
    // Using match
    match config_max {
        Some(max) => println!("Maximum is configured to be {}", max),
        _ => (),
    }
    
    // Same using if let (more concise)
    if let Some(max) = config_max {
        println!("Maximum is configured to be {}", max);
    }
    
    // if let with else
    let mut count = 0;
    if let Some(max) = config_max {
        println!("Maximum is {}", max);
    } else {
        count += 1;
    }
}
```

### while let Conditional Loop
```rust
fn main() {
    let mut stack = Vec::new();
    
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    // Pop values until stack is empty
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
```

---

## Error Handling
> Summary: Rust has no exceptions. Instead, it uses the Result enum for recoverable errors and panic! for unrecoverable errors.

### Unrecoverable Errors with panic!
```rust
fn main() {
    // Basic panic
    panic!("crash and burn");

    // Panic from array access
    let v = vec![1, 2, 3];
    v[99]; // This will panic!
}
```

### Recoverable Errors with Result
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // Basic Result handling
    let file_result = File::open("hello.txt");
    
    let file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => panic!("Problem opening file: {:?}", other_error),
        },
    };
}
```

### Shortcuts for Error Handling

#### The ? Operator
```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// Even shorter using fs::read_to_string
use std::fs;

fn read_username_from_file_short() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

#### unwrap and expect
```rust
fn main() {
    // unwrap will panic! if the Result is an Err
    let file = File::open("hello.txt").unwrap();
    
    // expect lets us choose the panic! message
    let file = File::open("hello.txt")
        .expect("Failed to open hello.txt");
}
```

### Custom Error Types
```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct AppError {
    kind: String,
    message: String,
}

// Implement Display for AppError
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.kind, self.message)
    }
}

// Implement Error for AppError
impl Error for AppError {}

fn main() -> Result<(), AppError> {
    Err(AppError {
        kind: String::from("IO"),
        message: String::from("Failed to read file"),
    })
}
```

### Propagating Errors
```rust
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

// Using Box<dyn Error> for different error types
fn read_and_process() -> Result<String, Box<dyn Error>> {
    let mut file = File::open("hello.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    Ok(content)
}

fn main() {
    match read_and_process() {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error: {}", e),
    }
}
```

### Error Handling Guidelines
```rust
fn main() {
    // Use Result when:
    // 1. The error is expected and recoverable
    // 2. You want to give the caller choice in handling the error
    let result = std::fs::read_to_string("config.txt");
    
    // Use panic! when:
    // 1. The error is unexpected or unrecoverable
    // 2. You're writing examples or tests
    // 3. You're prototyping
    assert!(result.is_ok(), "Config file must exist!");
    
    // Use expect when:
    // 1. You're sure the operation will succeed
    // 2. You want a better error message than unwrap()
    let config = result.expect("Config file must be readable");
}
```

---

## Generic Types & Traits
> Summary: Generics provide abstraction over types, while traits define shared behavior.

### Generic Types
> Summary: Generics allow you to write code that works with multiple types.

#### Generic Functions
```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&numbers));
    
    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char: {}", largest(&chars));
}
```

#### Generic Structs
```rust
struct Point<T> {
    x: T,
    y: T,
}

struct MixedPoint<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let mixed = MixedPoint { x: 5, y: 4.0 };
}
```

### Traits
> Summary: Traits define shared behavior between types.

#### Defining and Implementing Traits
```rust
trait Summary {
    // Default implementation
    fn summarize_author(&self) -> String;
    
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

#### Trait Bounds
```rust
// Single trait bound
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds
pub fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Where clauses for cleaner syntax
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    // Function body
    42
}
```

#### Trait Objects
```rust
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Implementing for different types
struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // Draw button
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // Draw select box
    }
}
```

### Generic Type Constraints
```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Implementation only for types that implement Display + PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

---

## Modules & Project Organization
> Summary: Modules help you organize code and control privacy. A package can contain multiple binary crates and optionally one library crate.

### Module System
```rust
// In src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

### Project Structure
```
my_project/
├── Cargo.toml
├── src/
│   ├── main.rs       // Binary crate root
│   ├── lib.rs        // Library crate root
│   └── bin/          // Additional binaries
│       └── another_binary.rs
```

### Modules in Separate Files
```rust
// src/lib.rs
mod front_of_house;  // Declares the module
pub use crate::front_of_house::hosting;  // Re-export

// src/front_of_house.rs
pub mod hosting;  // Declares the submodule

// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
```

### Visibility and Privacy
```rust
mod back_of_house {
    // Public struct with some private fields
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,  // Private field
    }
    
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    
    // Public enum with all variants public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    // meal.seasonal_fruit = String::from("blueberries"); // Error!
    
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

### Using External Packages
```toml
# Cargo.toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8.5"
serde = { version = "1.0", features = ["derive"] }
```

```rust
// src/main.rs
use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(1..=100);
    println!("Random number: {}", random_number);
}
```

### Workspaces
```toml
# Cargo.toml in workspace root
[workspace]
members = [
    "adder",
    "add_one",
]
```

```
workspace_root/
├── Cargo.toml
├── adder/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── add_one/
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

### Re-exporting Names
```rust
// In src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Re-export for easier access
pub use crate::front_of_house::hosting;

// Users can now use:
// restaurant::hosting::add_to_waitlist();
// Instead of:
// restaurant::front_of_house::hosting::add_to_waitlist();
```

---

## Concurrency
> Summary: Rust's ownership and type systems guarantee thread safety and prevent data races at compile time.

### Threads
```rust
use std::thread;
use std::time::Duration;

fn main() {
    // Spawn a new thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    // Main thread work
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    // Wait for spawned thread to finish
    handle.join().unwrap();
}
```

### Message Passing
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    // Create a channel
    let (tx, rx) = mpsc::channel();
    
    // Clone transmitter for multiple senders
    let tx1 = tx.clone();
    
    // Spawn thread with one sender
    thread::spawn(move || {
        tx.send("hello from first thread").unwrap();
    });
    
    // Spawn thread with cloned sender
    thread::spawn(move || {
        tx1.send("hello from second thread").unwrap();
    });
    
    // Receive messages
    for received in rx {
        println!("Got: {}", received);
    }
}
```

### Shared State
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create a mutex wrapped in an Arc (Atomic Reference Counting)
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Result: {}", *counter.lock().unwrap());
}
```

### Thread Safety with Send and Sync
```rust
// Send: Type can be transferred between threads
// Sync: Type can be shared between threads

use std::marker::{Send, Sync};

// Custom type that is both Send and Sync
#[derive(Debug)]
struct ThreadSafeCounter {
    count: u32,
}

// Implementing Send and Sync is unsafe
unsafe impl Send for ThreadSafeCounter {}
unsafe impl Sync for ThreadSafeCounter {}

fn main() {
    let counter = ThreadSafeCounter { count: 0 };
    
    // This works because ThreadSafeCounter is Send
    thread::spawn(move || {
        println!("Counter in thread: {:?}", counter);
    }).join().unwrap();
}
```

### Async/Await
```rust
use tokio;

#[tokio::main]
async fn main() {
    // Spawn async tasks
    let handle = tokio::spawn(async {
        // Async work here
        println!("Hello from async task!");
    });
    
    // Wait for task to complete
    handle.await.unwrap();
    
    // Multiple concurrent tasks
    let handles = vec![
        tokio::spawn(async { /* task 1 */ }),
        tokio::spawn(async { /* task 2 */ }),
    ];
    
    // Wait for all tasks
    for handle in handles {
        handle.await.unwrap();
    }
}
```

### Parallel Iterator Operations
```rust
use rayon::prelude::*;

fn main() {
    let numbers: Vec<i32> = (0..1000).collect();
    
    // Sequential sum
    let sum: i32 = numbers.iter().sum();
    
    // Parallel sum
    let parallel_sum: i32 = numbers.par_iter().sum();
    
    // Parallel filter and map
    let result: Vec<i32> = numbers
        .par_iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
}
```

---

## Testing
> Summary: Rust has built-in support for unit tests, integration tests, and documentation tests.

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        
        assert!(larger.can_hold(&smaller));
    }
    
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

### Test Organization
```rust
// src/lib.rs
pub fn add_two(a: i32) -> i32 {
    a + 2
}

// Unit tests in the same file
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}

// Integration tests in tests directory
// tests/integration_test.rs
use my_crate;

#[test]
fn it_adds_two() {
    assert_eq!(4, my_crate::add_two(2));
}
```

### Test Attributes
```rust
#[test]            // Marks function as a test
#[ignore]          // Marks test to be ignored unless specifically requested
#[should_panic]    // Test should panic
#[bench]           // Marks function as a benchmark (nightly only)

#[test]
#[ignore = "too slow"]
fn expensive_test() {
    // ...
}
```

### Assertions
```rust
fn main() {
    // Basic assertions
    assert!(true);
    assert_eq!(2 + 2, 4);
    assert_ne!(2 + 2, 5);
    
    // Custom messages
    assert!(
        true,
        "This is a custom error message: {} {}",
        "formatted", "arguments"
    );
    
    // Floating point comparisons
    assert!((0.1 + 0.2 - 0.3).abs() < f64::EPSILON);
}
```

### Running Tests
```bash
# Run all tests
cargo test

# Run specific test
cargo test it_works

# Run tests whose names contain a string
cargo test add

# Run ignored tests
cargo test -- --ignored

# Run tests with output
cargo test -- --nocapture

# Run tests in parallel or sequentially
cargo test -- --test-threads=1
```

### Documentation Tests
```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

### Test Driven Development Example
```rust
// First, write the test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_push() {
        let mut stack = Stack::new();
        stack.push(1);
        assert_eq!(stack.pop(), Some(1));
    }

    #[test]
    fn test_stack_pop_empty() {
        let mut stack = Stack::new();
        assert_eq!(stack.pop(), None);
    }
}

// Then implement the functionality
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}
```

### Integration Tests
```rust
// tests/integration_test.rs
use my_crate;

mod common;  // Load common test utilities

#[test]
fn integration_test() {
    common::setup();
    assert_eq!(my_crate::add_two(2), 4);
}

// tests/common/mod.rs
pub fn setup() {
    // Setup code used by multiple test files
}
```

---

## Smart Pointers
> Summary: Smart pointers are data structures that act like pointers but have additional metadata and capabilities.

### Box<T>
> Summary: Box<T> provides heap allocation and ownership of data.

```rust
fn main() {
    // Allocating on the heap
    let b = Box::new(5);
    println!("b = {}", b);
    
    // Recursive types with Box
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{:?}", list);
}
```

### Rc<T>
> Summary: Reference Counting allows multiple ownership in single-threaded scenarios.

```rust
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(List::Cons(5, Rc::new(List::Nil)));
    println!("Reference count after creating a = {}", Rc::strong_count(&a));
    
    let b = List::Cons(3, Rc::clone(&a));
    println!("Reference count after creating b = {}", Rc::strong_count(&a));
    
    {
        let c = List::Cons(4, Rc::clone(&a));
        println!("Reference count after creating c = {}", Rc::strong_count(&a));
    }
    
    println!("Reference count after c goes out of scope = {}", Rc::strong_count(&a));
}
```

### RefCell<T>
> Summary: Interior mutability pattern allows mutable borrowing in immutable contexts.

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    
    // Multiple mutable borrows (at different times)
    {
        let mut m1 = data.borrow_mut();
        *m1 += 1;
    }   // m1 is dropped here
    
    let mut m2 = data.borrow_mut();
    *m2 += 1;
    
    println!("Data = {:?}", data.borrow());
}
```

### Combining Rc<T> and RefCell<T>
```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });
    
    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    
    println!("leaf = {:?}", leaf);
    println!("branch = {:?}", branch);
}
```

### Weak<T>
> Summary: Weak references that don't prevent deallocation, useful for preventing reference cycles.

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```

### Custom Smart Pointer
```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y);  // Deref coercion
}
```

---

## Macros
> Summary: Macros are a way of writing code that writes other code, known as metaprogramming.

### Declarative Macros
```rust
// Simple macro
#[macro_export]
macro_rules! vec2 {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let v = vec2![1, 2, 3];  // Creates: let v = {let mut temp_vec = Vec::new(); temp_vec.push(1); temp_vec.push(2); temp_vec.push(3); temp_vec};
}
```

### Macro Patterns
```rust
macro_rules! my_macro {
    // Match zero arguments
    () => {
        println!("No arguments");
    };
    
    // Match one argument
    ($x:expr) => {
        println!("One argument: {}", $x);
    };
    
    // Match multiple arguments
    ($( $x:expr ),*) => {
        {
            $(
                println!("Multiple arguments: {}", $x);
            )*
        }
    };
    
    // Match specific patterns
    (key: $key:expr, value: $val:expr) => {
        println!("Key: {}, Value: {}", $key, $val);
    };
}
```

### Procedural Macros
```rust
use proc_macro;

// Derive macro
#[proc_macro_derive(MyTrait)]
pub fn my_trait_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Implementation
}

// Attribute macro
#[proc_macro_attribute]
pub fn route(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Implementation
}

// Function-like macro
#[proc_macro]
pub fn sql(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Implementation
}
```

### Common Macro Use Cases
```rust
// Debug printing
println!("Debug: {:?}", value);

// Format strings
format!("Hello, {}!", name);

// Vector creation
let v = vec![1, 2, 3];

// Error handling
panic!("Error message");

// Testing
assert!(condition);
assert_eq!(left, right);

// Custom error messages
assert!(
    condition,
    "Error occurred with value: {}",
    value
);
```

### Building Complex Macros
```rust
#[macro_export]
macro_rules! hash_map {
    // Empty map
    () => {
        {
            use std::collections::HashMap;
            HashMap::new()
        }
    };
    
    // Map with key-value pairs
    ( $($key:expr => $value:expr),* $(,)? ) => {
        {
            use std::collections::HashMap;
            let mut map = HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

fn main() {
    let map = hash_map! {
        "one" => 1,
        "two" => 2,
        "three" => 3,
    };
}
```

### Debugging Macros
```rust
// Use trace_macros! to see macro expansion
#![feature(trace_macros)]
trace_macros!(true);
my_macro!(some args);
trace_macros!(false);

// Use log_syntax! to see what tokens are passed
#![feature(log_syntax)]
log_syntax!(true);
my_macro!(some args);
log_syntax!(false);

// Expand macro without running
cargo expand
```

### Best Practices
```rust
// Document macros
/// Creates a vector containing the arguments.
///
/// # Examples
///
/// ```
/// let v = vec![1, 2, 3];
/// assert_eq!(v[0], 1);
/// ```
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// Use proper hygiene
macro_rules! hygienic {
    ($i:ident) => {
        let $i = 42;  // Won't conflict with external variables
    };
}
```

[Continue with Advanced Features section...]
