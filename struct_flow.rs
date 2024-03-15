// Define a struct representing a Rectangle
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method to calculate the area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // Create an instance of the Rectangle struct
    let rect = Rectangle {
        width: 10,
        height: 20,
    };

    // Accessing struct fields
    println!("Width: {}", rect.width);
    println!("Height: {}", rect.height);

    // Calling a method on the struct
    println!("Area: {}", rect.area());
}
