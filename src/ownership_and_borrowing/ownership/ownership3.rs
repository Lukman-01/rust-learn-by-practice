
// fn main() {
//     let s = give_ownership();
//     println!("{}", s);
// }

// // Only modify the code below!
// fn give_ownership() -> String {
//     let s = String::from("hello, world");
//     // Convert String to Vec
//     let _s = s.into_bytes();
//     s
// }


fn main() {
    let s = give_ownership();
    println!("{}", s);
}
// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // Convert String to Vec
    //let _s = s.into_bytes();
    let _s = s.as_bytes();
    // s.into_bytes() takes the ownership of s
    // s.as_bytes() reference to s without dropping s
    s
}
