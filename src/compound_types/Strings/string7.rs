
// // Fix error with at least two solutions
// fn main() {
//     let s = "hello, world";
//     greetings(s)
// }

// fn greetings(s: String) {
//     println!("{}", s)
// }



// &str can be converted to String in three ways
fn main() {
    let s = "hello, world";
    //greetings(s.to_string());
    //greetings(String::from(s));
    greetings(s.to_owned())
}

fn greetings(s: String) {
    println!("{}", s)
}
