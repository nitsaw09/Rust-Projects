# Hello World Program
This is a simple "Hello, world!" program written in Rust. The purpose of this program is to demonstrate a basic Rust project structure and how to use Cargo, Rust's package manager and build system.

## Prerequisites
Before running the program, ensure that you have Rust and Cargo installed on your system. You can install them by following the instructions on the official Rust website.

## Project Structure
The project is structured as follows:

```hello_world
│
├── src
│   └── main.rs
│
└── Cargo.toml
```
```src```: This directory contains the source code of the program.
```main.rs```: The main Rust file that contains the fn main() function with the "Hello, world!" print statement.
```Cargo.toml```: The manifest file that defines the project's metadata and dependencies.

## How to Run
To run the program, follow these steps:
1. Open a terminal and navigate to the project's root directory.
2. Run the following command to build and run the program using Cargo:
```cargo run```
3. This command will automatically download and build any necessary dependencies and then execute the program.

You should see the output:
```Hello, world!```

Congratulations! You have successfully run the "Hello, world!" program in Rust.