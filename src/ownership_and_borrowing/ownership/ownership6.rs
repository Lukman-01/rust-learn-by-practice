
// fn main() {
//     let s = String::from("hello, ");
    
//     // Modify this line only !
//     let s1 = s;

//     s1.push_str("world");

//     println!("Success!");
// }


fn main() {
    let s = String::from("hello, ");
    // Modify this line only !
    let mut s1 = s;
    s1.push_str("world");

    println!("Success!");
}
