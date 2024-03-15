enum TrafficLight {
    Red,
    Yellow,
    Green,
}

let light = TrafficLight::Red;
match light {
    TrafficLight::Red => println!("Stop!"),
    TrafficLight::Yellow => println!("Slow down!"),
    TrafficLight::Green => println!("Go!"),
}

// Define a module
pub mod my_module {
    // Define a public enum
    #[derive(Debug)]
    pub enum MyEnum {
        Variant1,
        Variant2,
        Variant3,
    }
}

// Import the enum from another module
use my_module::MyEnum;

fn main() {
    // Use the enum
    let var1 = MyEnum::Variant1;
    let var2 = MyEnum::Variant2;

    println!("{:?}", var1);
    println!("{:?}", var2);
}