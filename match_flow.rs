fn main() {
    let number = 5;

    match number {
        1 | 2 => {
            println!("One or Two");  // Match either 1 or 2
            // Call a function
            call_function();
            // Perform multiple operations
            let result = add_numbers(3, 4);
            println!("Result: {}", result);
        },
        3..=5 => {
            println!("Three to Five"); // Match the range from 3 to 5 (inclusive)
            // Call another function
            another_function();
            // Perform other operations
            let value = number * 10;
            println!("Value: {}", value);
        },
        _ => {
            println!("Other");  // Match any other value
            // Call some other function
            some_other_function();
            // Perform more operations
            let value = number.pow(2);
            println!("Squared Value: {}", value);
        },
    }
}

// Define some example functions
fn call_function() {
    println!("Called call_function()");
}

fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
}

fn another_function() {
    println!("Called another_function()");
}

fn some_other_function() {
    println!("