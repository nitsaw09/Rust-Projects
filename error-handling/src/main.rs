use std::fs::File;
use std::io::{self, Read};

// 1. Result Type
// The Result type is a powerful and widely used mechanism for handling errors in Rust.
// It represents either a successful result (Ok) or an error (Err).
fn read_file_contents(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

// 2. panic! Macro
// In situations where an unrecoverable error occurs, you can use the panic! macro
// to terminate the program with an error message.
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero!");
    }

    a / b
}

// 3. unwrap and expect Methods
// The unwrap method returns the value inside an Option or Result, panicking if it's None or Err.
// The expect method is similar but allows you to provide a custom error message.
fn result_option() {
    let result: Result<i32, &str> = Err("An error occurred");

    // Using unwrap
    // defualt error message
    // let unwrapped = result.unwrap(); // Uncommenting would cause a panic

    // Using expect
    // custom error message
    // let expected = result.expect("Custom error message"); // Uncommenting would cause a panic
}

// 4. ? Operator
// The ? operator is used to propagate errors up the call stack.
// It can only be used in functions that return a Result or Option.
fn process_file(file_path: &str) -> Result<(), io::Error> {
    let contents = read_file_contents(file_path)?;
    println!("File contents: {}", contents);

    Ok(())
}

fn main() {
    // 1. Result Type
    match read_file_contents("example.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(err) => eprintln!("Error reading file: {}", err),
    }

    // 2. panic! Macro
    let result = divide(10, 2);
    println!("Result: {}", result);

    // Uncommenting the line below would cause a panic
    // let invalid_result = divide(5, 0);

    // 3. unwrap and expect Methods
    // result_option();  // uncommenting will cause panic

    // 4. ? Operator
    match process_file("example.txt") {
        Ok(_) => println!("File processed successfully"),
        Err(err) => eprintln!("Error processing file: {}", err),
    }
}