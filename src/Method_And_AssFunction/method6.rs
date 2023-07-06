
// #[derive(Debug)]
// enum TrafficLightColor {
//     Red,
//     Yellow,
//     Green,
// }

// // Implement TrafficLightColor with a method.
// impl TrafficLightColor {
    
// }

// fn main() {
//     let c = TrafficLightColor::Yellow;

//     assert_eq!(c.color(), "yellow");

//     println!("{:?}",c);
// }


// We can also implement methods for enums.
#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// Implement TrafficLightColor with a method.
impl TrafficLightColor {
    fn color(&self) -> &str{
        match self {
            Self::Yellow => "Yellow", // or TrafficLightColor::Yellow => "Yellow"
            Self::Green => "Green", // or TrafficLightColor::Green => "Green"
            _ => "Red",
        }
    }
}

fn main() {
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}",c);
}
