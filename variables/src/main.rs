fn main() {
    // Immutable variable 'b' with type annotation 'i32'
    let a: i32 = 1;
    println!("The value of a = {}", a);
    
    // Immutable variable 'b' with type annotation 'bool'
    let b: bool = true;
    println!("The value of b = {}", b); 

    // Shadowing variable 'b' with type annotation 'bool'
    let b: bool = false;
    println!("The value of b = {}", b); 

    // mutable variable 'c' with type annotation 'i32'
    let mut c: i32 = 1;
    println!("The value of c = {}", c); 
    c = 3;
    println!("The value of c = {}", c); 

    // constant variable 'STRING' with type annotation '&str'
    const STRING: &str = "Hello";
    println!("The value of STRING = {}", STRING);
}
