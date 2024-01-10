/* 
* scalar types: integers, floating-point numbers, Booleans, and characters
* Default bit size for each inferred scalar type:
*   For integers: Usually 32 or 64 bits based on the platform
*   For floating-point numbers: Usually f64
*   For boolean values: Implementation-defined (commonly 8 bits)
*   For char: 32 bits (Unicode scalar value)
*/ 
fn main() {
    // signed 8-bit integer (u8) with value 5
    let s: i8 = 2;

    // Unsigned 8-bit integer (u8) with value 5
    let a: u8 = 5;

    // Unsigned 8-bit integer (u8) with value 6
    let b: u8 = 6;

    // 64-bit floating-point number (f64) with value 5.0
    let c: f32 = 5.0;

    // 32-bit floating-point number (f32) with value 6.0
    let d: f64 = 6.0;

    // sqaure of i8 value (s^2)
    let square: i8 = s.pow(2);
    println!("The value of power square = {:?}", square);

    // Addition of two u8 values (a + b)
    let add: u8 = a + b;
    println!("The value of addition = {:?}", add);

    // Subtraction of a f64 value by 1.0 (b - 1.0)
    let subtract: f64 = d - 1.0;
    println!("The value of subtraction = {:?}", subtract);

    // Multiplication of a u8 value by 4 (4 * a)
    let multiply: u8 = 4 * a;
    println!("The value of multiplication = {:?}", multiply);

    // Division of a f64 value by 2.0 (c / 2.0)
    let divider: f32 = c / 2.0;
    println!("The value of divider = {:?}", divider);

    // Remainder of the division of two u8 values (a % b)
    let remainder: u8 = a % b;
    println!("The value of remainder = {:?}", remainder);

    // Boolean values (true and false)
    let t: bool = true;
    let f: bool = false;
    println!("The value of t = {:?}", t);
    println!("The value of f = {:?}", f);

    // Character (char) with a smiley face emoji '😊'
    let c: char = '😊';
    println!("The value of c = {:?}", c);
}
