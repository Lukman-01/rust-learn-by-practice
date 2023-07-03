// // Fix the error
// struct Person {
//     name: String,
//     age: u8,
//     hobby: String
// }
// fn main() {
//     let age = 30;
//     let p = Person {
//         name: String::from("sunface"),
//         age,
//     };

//     println!("Success!");
// } 


// We must specify concrete values for each of the fields in struct
struct Person {
    name: String,
    age: u8,
    hobby: String
}
fn main() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age: age,
        hobby: String::from("Lukman"),
    };

    println!("Success!");
} 
