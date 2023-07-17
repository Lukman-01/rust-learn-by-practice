// // To use `from_str` method, you need to introduce this trait into the current scope.
// use std::str::FromStr;
// fn main() {
//     let parsed: i32 = "5".__.unwrap();
//     let turbo_parsed = "10".__.unwrap();
//     let from_str = __.unwrap();
//     let sum = parsed + turbo_parsed + from_str;
//     assert_eq!(sum, 35);

//     println!("Success!");
// }


// We can use parse method to convert a String into a i32 number, 
// this is because FromStr is implemented for i32 type in standard library: impl FromStr for i32

use std::str::FromStr;
fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let from_str = i32::from_str("20").unwrap();
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("Success!");
}