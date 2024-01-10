// Basic function with parameters and a return type
fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
}

// Function with an expression body (no need for braces)
fn multiply_numbers(x: i32, y: i32) -> i32 {
    x * y
}

// Function with no parameters
fn greet() {
    println!("Hello, world!");
}

// Function with no return type
fn print_message(message: &str) {
    println!("{}", message);
}

// Function with named parameters
fn greet_person(name: &str, greeting: &str) {
    println!("{} {}", greeting, name);
}

fn main() {
    // Calling a basic function
    let result = add_numbers(5, 3);
    println!("Result: {}", result);

    // Calling a function with an expression body
    let product = multiply_numbers(4, 6);
    println!("Product: {}", product);

    // Calling a function with no parameters
    greet();

    // Calling a function with named parameters
    greet_person("Alice", "Hi");

    // Calling a function with no return type
    print_message("This is a simple message.");
}
