fn main() {
    // 1. if Statement
    let number = 5;

    if number < 0 {
        println!("Number is negative");
    } else if number == 0 {
        println!("Number is zero");
    } else {
        println!("Number is positive");
    }

    // 2. match Statement
    let day = "Wednesday";

    match day {
        "Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday" => println!("It's a weekday!"),
        "Saturday" | "Sunday" => println!("It's the weekend!"),
        _ => println!("Invalid day"),
    }

    // 3. for Loop
    for number in 1..=5 {
        println!("Current number: {}", number);
    }

    // 4. while Loop
    let mut count = 0;

    while count < 5 {
        println!("Current count: {}", count);
        count += 1;
    }

    // 5. loop Statement with break
    let mut counter = 0;

    loop {
        println!("Counter: {}", counter);
        counter += 1;

        if counter == 5 {
            break; // Exit the loop when counter reaches 5
        }
    }

    // 6. Using continue in a Loop
    for number in 1..=5 {
        if number == 3 {
            continue; // Skip printing number 3
        }
        println!("Current number: {}", number);
    }
}
