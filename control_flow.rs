fn main() {
    let number = 7;

    // if-else statement
    if number % 2 == 0 {
        println!("{} is even.", number);
    } else {
        println!("{} is odd.", number);
    }

    // Match expression
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3..=5 => println!("Three to Five"),
        6..=9 => println!("Six to Nine"),
        _ => println!("Other"),
    }

    // Looping with for loop
    let fruits = ["apple", "banana", "orange", "grape"];
    for fruit in fruits.iter() {
        println!("I like {}", fruit);
    }

    // Looping with while loop
    let mut counter = 0;
    while counter < 5 {
        println!("Counter: {}", counter);
        counter += 1;
    }

    // Looping with loop and break
    let mut countdown = 3;
    loop {
        println!("Countdown: {}", countdown);
        if countdown == 0 {
            break;
        }
        countdown -= 1;
    }
}