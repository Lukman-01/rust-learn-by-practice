// // Fill the blank
// struct Person {
//     name: String,
//     age: u8,
// }
// fn main() {
//     println!("Success!");
// } 

// fn build_person(name: String, age: u8) -> Person {
//     Person {
//         age,
//         __
//     }
// }



// Using field init shorthand syntax to reduce repetitions.
struct Person {
    name: String,
    age: u8,
}
fn main() {
    println!("Success!");
} 

fn build_person(name: String, age: u8) -> Person {
    Person {
        age,
        name,
    }
}
