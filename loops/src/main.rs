// Function for loop example
fn loop_example() -> i32 {
    let mut i: i32 = 1;

    // The `loop` construct creates an infinite loop until a `break` statement is encountered.
    // In this example, it doubles the value of `i` until it exceeds 100.
    let result: i32 = loop {
        i *= 2;
        if i > 100 {
            break i; // The loop breaks with the value of `i` when it exceeds 100.
        }
    };

    // Advantage: Allows for more complex loop conditions and logic.
    // Disadvantage: Requires manual handling to exit the loop, making it prone to infinite loops.

    result
}

// Function for while loop example
fn while_example() {
    let mut counter: i32 = 0;

    // The `while` loop executes a block of code as long as the specified condition is true.
    // In this example, it prints the counter until it reaches 10.
    while counter < 10 {
        println!("The number count = {:?}", counter);
        counter += 1;
    }

    // Advantage: Suitable for scenarios where the condition is checked before each iteration.
    // Disadvantage: More verbose for complex conditions; may require manual counter updates.
}

// Function for loop example
fn for_example() {
    let items: [i32; 5] = [1, 2, 3, 4, 5];

    // The `for` loop iterates over each element in the specified range or collection.
    // In this example, it iterates over the array `items` and prints each item along with its index.
    for (index, &item) in items.iter().enumerate() {
        println!("The item[{}] = {}", index, item);
    }

    // Advantage: Concise syntax for iterating over collections with a known size.
    // Disadvantage: Limited to iterating over a predefined range or collection.
}

fn main() {
    // 1. loop
    let something = loop_example();
    assert_eq!(something, 128);

    // 2. while loop
    while_example();

    // 3. for loop 
    for_example();
}
