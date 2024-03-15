// Define a type alias for a function that takes an i32 parameter and returns an i32 value
type MyFunction = fn(i32) -> i32;

// Define a function that takes another function as a parameter
fn apply_function(f: MyFunction, value: i32) -> i32 {
    f(value) // Call the provided function f with the provided value and return the result
}

// Define a function that squares the input value
fn square(x: i32) -> i32 {
    x * x // Return the square of the input parameter
}

fn main() {
    let number = 5;

    // Pass the square function as a parameter to apply_function
    let result = apply_function(square, number);
    println!("The square of {} is {}", number, result);
}