fn main() {
    // Define an array of integers
    let numbers = [1, 2, 3, 4, 5];
    println!("Array: {:?}", numbers);

    // Access and print individual elements of the array
    println!("First element: {}", numbers[0]);
    println!("Second element: {}", numbers[1]);

    // Calculate the sum of all elements in the array
    let sum: i32 = numbers.iter().sum();
    println!("Sum of array elements: {}", sum);

    // Calculate the sum of all elements in the array
    let length: usize = numbers.len();
    println!("Length of array: {}", length);

    // Define datatype and fixed length array of integers
    let fixed_length_array: [u32; 5] = [1, 2, 3, 4, 5];
    println!("Fixed length array: {:?}", fixed_length_array);

    // Define a tuple with mixed data types
    let person = ("Alice", 30, true);

    // Access and print individual elements of the tuple
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is Adult: {}", person.2);

    // Destructure the tuple for clearer variable names
    let (name, age, is_adult) = person;

    // Print using the destructured variables
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Is Adult: {}", is_adult);

    // Define a tuple with mixed data types
    let person2: (&str, i8, bool) = ("Michel", 25, true);

    // Access and print individual elements of the tuple
    println!("Name: {}", person2.0);
    println!("Age: {}", person2.1);
    println!("Is Adult: {}", person2.2);
}
