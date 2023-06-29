
// fn main() {
//     // Use as many approaches as you can to make it work
//     let x = String::from("hello, world");
//     let y = x;
//     println!("{},{}",x,y);
// }



fn main() {
    // Use as many approaches as you can to make it work
    let x: String = String::from("hello, world");
    let y: String = x.clone();
    println!("{},{}",x,y);
}
