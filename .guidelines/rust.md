# Rust Programming Guidelines

## Philosophy

Write idiomatic Rust for version 1.93+. Embrace modern patterns, leverage the type system, and follow zero-cost abstraction principles. Write code that is safe, concurrent, and performant.

---

## Project Setup

### Creating a New Project

```bash
# Binary project
cargo new my-project

# Library project
cargo new --lib my-library
```

### Cargo.toml Structure

```toml
[package]
name = "my-project"
version = "0.1.0"
edition = "2024"
rust-version = "1.93"

[dependencies]
# Use specific versions for stability
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.40", features = ["full"] }

[dev-dependencies]
# Test and benchmark dependencies
criterion = "0.5"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

---

## Module System

### File Organization

```rust
// src/lib.rs or src/main.rs
mod config;
mod database;
mod api;

pub use config::Config;
pub use api::start_server;

// src/config.rs
pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }
}

// src/database/mod.rs
mod connection;
mod query;

pub use connection::Connection;
pub use query::Query;
```

### Visibility

```rust
// Public
pub fn public_function() {}
pub struct PublicStruct;

// Public within crate
pub(crate) fn crate_visible() {}

// Public within module
pub(super) fn parent_visible() {}

// Private (default)
fn private_function() {}
```

---

## Naming Conventions

- **Crates**: lowercase with hyphens (`my-crate`)
- **Modules**: lowercase with underscores (`my_module`)
- **Types**: PascalCase (`MyStruct`, `MyEnum`)
- **Traits**: PascalCase (`MyTrait`, `Display`)
- **Functions**: snake_case (`my_function`)
- **Variables**: snake_case (`my_variable`)
- **Constants**: SCREAMING_SNAKE_CASE (`MAX_SIZE`)
- **Lifetimes**: short lowercase (`'a`, `'b`)
- **Type parameters**: single uppercase or PascalCase (`T`, `K`, `V`, `Item`)

---

## Types and Ownership

### Basic Types

```rust
// Integers
let x: i32 = 42;
let y: u64 = 100;

// Floats
let pi: f64 = 3.14159;

// Boolean
let is_active: bool = true;

// Characters (Unicode scalar)
let c: char = '🦀';

// Strings
let s: String = String::from("hello");
let slice: &str = "hello";

// Arrays (fixed size)
let arr: [i32; 3] = [1, 2, 3];

// Tuples
let tuple: (i32, &str, bool) = (42, "hello", true);
```

### Ownership Rules

```rust
// Ownership transfer (move)
let s1 = String::from("hello");
let s2 = s1;  // s1 is no longer valid

// Borrowing (immutable)
let s = String::from("hello");
let len = calculate_length(&s);  // s is still valid

// Mutable borrowing
let mut s = String::from("hello");
modify_string(&mut s);

// Multiple immutable borrows are allowed
let s = String::from("hello");
let r1 = &s;
let r2 = &s;

// But not with mutable borrow
let mut s = String::from("hello");
let r1 = &mut s;
// let r2 = &mut s;  // Error: cannot borrow mutably twice
```

### Lifetimes

```rust
// Explicit lifetime annotations
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Struct with lifetime
struct Excerpt<'a> {
    part: &'a str,
}

// Multiple lifetimes
fn complex<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}

// Static lifetime
const GLOBAL: &str = "global string";
```

---

## Structs and Enums

### Structs

```rust
// Named fields
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub username: String,
    pub email: String,
    active: bool,
}

impl User {
    // Associated function (constructor)
    pub fn new(username: String, email: String) -> Self {
        Self {
            username,
            email,
            active: true,
        }
    }
    
    // Method
    pub fn deactivate(&mut self) {
        self.active = false;
    }
    
    // Getter
    pub fn is_active(&self) -> bool {
        self.active
    }
}

// Tuple struct
struct Point(f64, f64, f64);

// Unit struct
struct Marker;
```

### Enums

```rust
// Simple enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Active,
    Inactive,
    Pending,
}

// Enum with data
#[derive(Debug, Clone)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    pub fn process(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
        }
    }
}

// Option and Result (built-in enums)
let maybe: Option<i32> = Some(42);
let result: Result<i32, String> = Ok(42);
```

---

## Pattern Matching

```rust
// Match expression
let value = match some_option {
    Some(x) => x,
    None => 0,
};

// Match with guards
match number {
    n if n < 0 => println!("Negative"),
    0 => println!("Zero"),
    n if n > 0 => println!("Positive: {}", n),
    _ => unreachable!(),
}

// Destructuring
match point {
    Point(x, 0, 0) => println!("On x axis at {}", x),
    Point(0, y, 0) => println!("On y axis at {}", y),
    Point(x, y, z) => println!("Point at ({}, {}, {})", x, y, z),
}

// If let (single pattern)
if let Some(value) = some_option {
    println!("Got: {}", value);
}

// While let
while let Some(value) = iterator.next() {
    println!("{}", value);
}

// Let else (Rust 1.65+)
let Some(value) = some_option else {
    return Err("No value");
};
```

---

## Error Handling

### Result Type

```rust
use std::fs::File;
use std::io::{self, Read};

// Return Result
fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Custom error type
#[derive(Debug)]
pub enum MyError {
    IoError(io::Error),
    ParseError(String),
    NotFound,
}

impl From<io::Error> for MyError {
    fn from(err: io::Error) -> Self {
        MyError::IoError(err)
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MyError::IoError(e) => write!(f, "IO error: {}", e),
            MyError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            MyError::NotFound => write!(f, "Not found"),
        }
    }
}

impl std::error::Error for MyError {}
```

### Using anyhow and thiserror

```rust
// anyhow for applications (flexible error handling)
use anyhow::{Context, Result};

fn process() -> Result<()> {
    let contents = std::fs::read_to_string("file.txt")
        .context("Failed to read file.txt")?;
    Ok(())
}

// thiserror for libraries (custom error types)
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("Data not found")]
    NotFound,
    #[error("Invalid data: {0}")]
    Invalid(String),
    #[error("IO error")]
    Io(#[from] std::io::Error),
}
```

---

## Traits

### Defining Traits

```rust
pub trait Summary {
    fn summarize(&self) -> String;
    
    // Default implementation
    fn summarize_author(&self) -> String {
        String::from("Unknown")
    }
}

// Implementing trait
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

// Trait bounds
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds
fn print_info<T: Summary + Display>(item: &T) {
    println!("{}", item);
}

// Where clause (cleaner for complex bounds)
fn complex_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
    42
}
```

### Common Traits to Derive

```rust
#[derive(Debug)]           // Debug formatting
#[derive(Clone)]           // Explicit cloning
#[derive(Copy)]            // Implicit copying (requires Clone)
#[derive(PartialEq, Eq)]   // Equality comparison
#[derive(PartialOrd, Ord)] // Ordering
#[derive(Hash)]            // Hashing
#[derive(Default)]         // Default values
struct MyStruct {
    field: i32,
}
```

---

## Generics

```rust
// Generic struct
pub struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
    
    pub fn get(&self) -> &T {
        &self.value
    }
}

// Generic function
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Associated types
pub trait Iterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
}

// Const generics
struct ArrayPair<T, const N: usize> {
    left: [T; N],
    right: [T; N],
}
```

---

## Collections

### Vec

```rust
// Create
let mut vec = Vec::new();
let vec = vec![1, 2, 3];

// Add elements
vec.push(4);
vec.extend([5, 6, 7]);

// Access
let first = vec[0];
let maybe_first = vec.get(0);

// Iterate
for item in &vec {
    println!("{}", item);
}

// Map and collect
let doubled: Vec<i32> = vec.iter().map(|x| x * 2).collect();

// Filter
let evens: Vec<i32> = vec.into_iter().filter(|x| x % 2 == 0).collect();
```

### HashMap

```rust
use std::collections::HashMap;

// Create
let mut map = HashMap::new();
map.insert("key", "value");

// Access
let value = map.get("key");

// Insert or update
map.entry("key").or_insert("default");
*map.entry("counter").or_insert(0) += 1;

// Iterate
for (key, value) in &map {
    println!("{}: {}", key, value);
}
```

### HashSet

```rust
use std::collections::HashSet;

let mut set = HashSet::new();
set.insert(1);
set.insert(2);

// Check membership
if set.contains(&1) {
    println!("Found");
}

// Set operations
let set1: HashSet<_> = [1, 2, 3].iter().collect();
let set2: HashSet<_> = [3, 4, 5].iter().collect();

let union: HashSet<_> = set1.union(&set2).collect();
let intersection: HashSet<_> = set1.intersection(&set2).collect();
```

---

## Iterators

```rust
// Creating iterators
let vec = vec![1, 2, 3];
let iter = vec.iter();        // Borrows elements
let iter = vec.iter_mut();    // Mutable borrow
let iter = vec.into_iter();   // Takes ownership

// Consuming adaptors
let sum: i32 = vec.iter().sum();
let product: i32 = vec.iter().product();
let max = vec.iter().max();

// Iterator adaptors
let doubled: Vec<i32> = vec.iter().map(|x| x * 2).collect();
let evens: Vec<i32> = vec.into_iter().filter(|x| x % 2 == 0).collect();

// Chaining
let result: i32 = vec.iter()
    .filter(|x| **x > 0)
    .map(|x| x * 2)
    .sum();

// Custom iterator
struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```

---

## Closures

```rust
// Basic closure
let add = |x, y| x + y;
let result = add(5, 3);

// Closure with type annotations
let multiply = |x: i32, y: i32| -> i32 { x * y };

// Capturing environment
let value = 42;
let closure = || println!("Captured: {}", value);

// Move closure (takes ownership)
let s = String::from("hello");
let closure = move || println!("{}", s);

// Closure as function parameter
fn apply<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

// FnOnce, FnMut, Fn traits
fn call_once<F: FnOnce()>(f: F) { f(); }
fn call_mut<F: FnMut()>(mut f: F) { f(); }
fn call_many<F: Fn()>(f: F) { f(); f(); }
```

---

## Smart Pointers

### Box

```rust
// Heap allocation
let b = Box::new(5);

// Recursive types
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

### Rc (Reference Counting)

```rust
use std::rc::Rc;

let a = Rc::new(5);
let b = Rc::clone(&a);
let c = Rc::clone(&a);

println!("Count: {}", Rc::strong_count(&a));  // 3
```

### Arc (Atomic Reference Counting - Thread-Safe)

```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3]);

for _ in 0..3 {
    let data = Arc::clone(&data);
    thread::spawn(move || {
        println!("{:?}", data);
    });
}
```

### RefCell (Interior Mutability)

```rust
use std::cell::RefCell;

let value = RefCell::new(5);

// Borrow
{
    let borrowed = value.borrow();
    println!("{}", borrowed);
}

// Mutable borrow
{
    let mut borrowed = value.borrow_mut();
    *borrowed += 1;
}
```

### Mutex (Thread-Safe Interior Mutability)

```rust
use std::sync::Mutex;

let counter = Mutex::new(0);

{
    let mut num = counter.lock().unwrap();
    *num += 1;
}
```

---

## Async/Await

### Async Functions

```rust
use tokio;

#[tokio::main]
async fn main() {
    let result = fetch_data().await;
    println!("{:?}", result);
}

async fn fetch_data() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://example.com").await?;
    let body = response.text().await?;
    Ok(body)
}

// Concurrent execution
async fn fetch_all() {
    let (result1, result2) = tokio::join!(
        fetch_data(),
        fetch_other_data()
    );
}

// Select first completion
async fn race() {
    tokio::select! {
        result1 = fetch_data() => println!("First: {:?}", result1),
        result2 = fetch_other_data() => println!("Second: {:?}", result2),
    }
}
```

### Async Traits (Rust 1.75+)

```rust
trait AsyncService {
    async fn process(&self) -> Result<String, Error>;
}

impl AsyncService for MyService {
    async fn process(&self) -> Result<String, Error> {
        // Async implementation
        Ok(String::from("done"))
    }
}
```

---

## Concurrency

### Threads

```rust
use std::thread;
use std::time::Duration;

// Spawn thread
let handle = thread::spawn(|| {
    for i in 1..10 {
        println!("Thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
});

// Wait for thread
handle.join().unwrap();

// Move closure
let v = vec![1, 2, 3];
let handle = thread::spawn(move || {
    println!("{:?}", v);
});
```

### Channels

```rust
use std::sync::mpsc;
use std::thread;

// Create channel
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send(String::from("hello")).unwrap();
});

let received = rx.recv().unwrap();
println!("{}", received);

// Multiple senders
let (tx, rx) = mpsc::channel();
let tx1 = tx.clone();

thread::spawn(move || tx.send(1).unwrap());
thread::spawn(move || tx1.send(2).unwrap());
```

---

## Macros

### Declarative Macros

```rust
// Simple macro
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

// Macro with arguments
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("Function {:?} called", stringify!($func_name));
        }
    };
}

// Repeating patterns
macro_rules! vec_of_strings {
    ($($element:expr),*) => {
        {
            let mut v = Vec::new();
            $(
                v.push($element.to_string());
            )*
            v
        }
    };
}
```

### Procedural Macros

```rust
// Custom derive macro (in separate crate)
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(MyDerive)]
pub fn my_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_my_derive(&ast)
}
```

---

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_with_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Math is broken"))
        }
    }

    #[test]
    #[should_panic(expected = "panic message")]
    fn test_panic() {
        panic!("panic message");
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // Run with: cargo test -- --ignored
    }
}
```

### Integration Tests

```rust
// tests/integration_test.rs
use my_crate;

#[test]
fn test_public_api() {
    let result = my_crate::public_function();
    assert_eq!(result, expected);
}
```

### Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Test helper
    fn setup() -> TestContext {
        TestContext::new()
    }

    #[test]
    fn test_with_setup() {
        let ctx = setup();
        // Use ctx
    }
}
```

---

## Documentation

```rust
/// Public function that does something useful.
///
/// # Arguments
///
/// * `x` - The first parameter
/// * `y` - The second parameter
///
/// # Returns
///
/// The sum of `x` and `y`
///
/// # Examples
///
/// ```
/// let result = my_crate::add(2, 3);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// This function will panic if both arguments are negative.
///
/// # Errors
///
/// Returns an error if the sum overflows.
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

/// Module-level documentation
///
/// This module contains utilities for processing data.

/// Struct documentation
///
/// Represents a user in the system.
#[derive(Debug)]
pub struct User {
    /// The user's unique identifier
    pub id: u64,
    /// The user's name
    pub name: String,
}
```

---

## Common Patterns

### Builder Pattern

```rust
#[derive(Debug)]
pub struct Config {
    host: String,
    port: u16,
    timeout: u64,
}

pub struct ConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    timeout: Option<u64>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            host: None,
            port: None,
            timeout: None,
        }
    }

    pub fn host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn build(self) -> Result<Config, &'static str> {
        Ok(Config {
            host: self.host.ok_or("host is required")?,
            port: self.port.unwrap_or(8080),
            timeout: self.timeout.unwrap_or(30),
        })
    }
}

// Usage
let config = ConfigBuilder::new()
    .host("localhost".to_string())
    .port(3000)
    .build()?;
```

### Newtype Pattern

```rust
pub struct UserId(u64);

impl UserId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}
```

### RAII (Resource Acquisition Is Initialization)

```rust
pub struct FileGuard {
    file: File,
}

impl FileGuard {
    pub fn new(path: &str) -> io::Result<Self> {
        Ok(Self {
            file: File::create(path)?,
        })
    }
}

impl Drop for FileGuard {
    fn drop(&mut self) {
        // Cleanup happens automatically
        println!("File closed");
    }
}
```

---

## Performance Tips

### Avoid Unnecessary Clones

```rust
// Bad
fn process(data: Vec<String>) {
    for item in data.clone() {  // Unnecessary clone
        println!("{}", item);
    }
}

// Good
fn process(data: &[String]) {
    for item in data {
        println!("{}", item);
    }
}
```

### Use Appropriate Collection Methods

```rust
// Preallocate capacity
let mut vec = Vec::with_capacity(1000);

// Extend instead of repeated push
vec.extend(other_vec);

// Reserve space
vec.reserve(100);
```

### Inline Critical Functions

```rust
#[inline]
fn hot_path_function(x: i32) -> i32 {
    x * 2 + 1
}

#[inline(always)]
fn always_inline() {
    // Force inlining
}
```

### Use References When Possible

```rust
// Bad
fn calculate(data: Vec<i32>) -> i32 {
    data.iter().sum()
}

// Good
fn calculate(data: &[i32]) -> i32 {
    data.iter().sum()
}
```

---

## Clippy and Rustfmt

### Clippy (Linter)

```bash
# Run clippy
cargo clippy

# Clippy with all lints
cargo clippy -- -W clippy::all

# Fix automatically
cargo clippy --fix
```

### Allow/Deny Specific Lints

```rust
#![warn(clippy::all)]
#![deny(clippy::correctness)]

#[allow(clippy::needless_return)]
fn my_function() -> i32 {
    return 42;
}
```

### Rustfmt (Formatter)

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

### rustfmt.toml

```toml
max_width = 100
tab_spaces = 4
edition = "2024"
```

---

## Best Practices

### Prefer Expressions Over Statements

```rust
// Good
let result = if condition {
    42
} else {
    0
};

// Good
let value = match option {
    Some(x) => x,
    None => 0,
};
```

### Use Type Inference

```rust
// Good
let numbers = vec![1, 2, 3];

// Verbose (unnecessary)
let numbers: Vec<i32> = vec![1, 2, 3];
```

### Avoid Unwrap in Production

```rust
// Bad
let value = some_option.unwrap();

// Good
let value = some_option.expect("Value should be present");

// Better
let value = match some_option {
    Some(v) => v,
    None => return Err("Value not found"),
};

// Best
let value = some_option.ok_or("Value not found")?;
```

### Use Descriptive Error Messages

```rust
// Bad
.expect("error");

// Good
.expect("Failed to read config file: config.toml not found");
```

### Implement Display for User-Facing Types

```rust
use std::fmt;

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}
```

---

## Common Cargo Commands

```bash
# Build project
cargo build
cargo build --release

# Run project
cargo run
cargo run --release

# Test
cargo test
cargo test -- --nocapture  # Show println output
cargo test test_name       # Run specific test

# Benchmark
cargo bench

# Documentation
cargo doc --open

# Check without building
cargo check

# Format code
cargo fmt

# Lint
cargo clippy

# Update dependencies
cargo update

# Add dependency
cargo add serde

# Show dependency tree
cargo tree

# Clean build artifacts
cargo clean
```

---

## Checklist

- [ ] Using Rust 1.93+ with edition 2024
- [ ] All functions have appropriate visibility
- [ ] Proper error handling (no unwrap in production)
- [ ] Variables are properly scoped (use `let`)
- [ ] Borrowed values are properly lifetime-annotated
- [ ] Using `&str` for string slices, `String` for owned
- [ ] Using `&[T]` for slices, `Vec<T>` for owned
- [ ] Derive Debug for all public types
- [ ] Public API is documented with examples
- [ ] Tests cover main functionality
- [ ] Code passes `cargo clippy`
- [ ] Code is formatted with `cargo fmt`
- [ ] No compiler warnings
- [ ] Using appropriate error handling (Result, anyhow, thiserror)
- [ ] Async code uses tokio or async-std consistently