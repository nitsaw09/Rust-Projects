/** 
    1. Tuple Struct
    Tuple structs are similar to tuples but have named fields. 
    They are useful when you want to give meaning to the individual components.
*/
struct Point(i32, i32);

/* 
    2. Classic Struct
    Classic structs have named fields, allowing you to access data using field names. 
    They are useful when you want to represent entities with multiple attributes.
*/
struct Rectangle {
    width: u32,
    height: u32,
}

/* 
    3. Unit-like Struct
    Unit-like structs have no fields and are similar to empty tuples ().
    They are useful when you need a named type but don't need to store any data.
*/
struct EmptyStruct;

/* 
    4. Struct with Methods
    Structs can have associated methods that operate on instances of the struct.
*/
struct Circle {
    radius: f64,
}
impl Circle {
    // Method to calculate the area of the circle
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

/* 
    5. Tuple Struct with Named Fields (Newtype Pattern)
    Tuple structs can also have named fields, combining the benefits of tuple structs and classic structs. 
    This pattern is often called the "newtype" pattern.
*/
struct Inches(i32);

fn main() {
    // Tuple Struct example
    let p = Point(10, 20);
    println!("Point coordinates: ({}, {})", p.0, p.1);

    // Classic Struct example
    let rect = Rectangle { width: 30, height: 50 };
    println!("Rectangle dimensions: {}x{}", rect.width, rect.height);

    // Unit-like Struct example
    let empty = EmptyStruct;
    
    // Struct with Methods example
    let circle = Circle { radius: 3.0 };
    println!("Circle area: {:.2}", circle.area());

    // Tuple Struct with Named Fields (Newtype Pattern) example
    let length = Inches(12);
    println!("Length in inches: {}", length.0);
}
