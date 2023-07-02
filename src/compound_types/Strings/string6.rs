
// // Fix errors without removing any line
// fn main() {
//     let s1 = String::from("hello,");
//     let s2 = String::from("world!");
//     let s3 = s1 + s2; 
//     assert_eq!(s3, "hello,world!");
//     println!("{}", s1);
// }



// You can only concat a String with &str, and String's ownership can be moved to another variable.
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // or let s3 = s1 + s2 as_str(); 
    assert_eq!(s3, "hello,world!");
    println!("{}", s3);
}
