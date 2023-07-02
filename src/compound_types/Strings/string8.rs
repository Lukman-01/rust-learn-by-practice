
// // Use two approaches to fix the error and without adding a new line
// fn main() {
//     let s = "hello, world".to_string();
//     let s1: &str = s;

//     println!("Success!");
// }



// Use two approaches to fix the error and without adding a new line
fn main() {
    let s = "hello, world".to_string();
    let s1: &str = &s;
    // or let s1: &str = s as_str();

    println!("Success!");
}
