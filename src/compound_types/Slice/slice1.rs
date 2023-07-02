
// // Fix the errors, DON'T add new lines!
// fn main() {
//     let arr = [1, 2, 3];
//     let s1: [i32] = arr[0..2];

//     let s2: str = "hello, world" as str;

//     println!("Success!");
// }



// Here, both [i32] and str are slice types, but directly using it will cause errors. 
//You have to use the reference of the slice instead: &[i32], &str.

fn main() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    let s2: &str = "hello, world";

    println!("Success!");
}
