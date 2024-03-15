// Define a trait representing a Shape
trait Shape {
    fn area(&self) -> u32;
}

// Implement the Shape trait for the Rectangle struct
impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 20,
    };

    // Call the area method using the Shape trait
    println!("Area: {}", rect.area());
}