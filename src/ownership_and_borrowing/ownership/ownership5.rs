// // Don't use clone ,use copy instead
// fn main() {
//     let x = (1, 2, (), "hello".to_string());
//     let y = x.clone();
//     println!("{:?}, {:?}", x, y);
// }


fn main() {
    let x: (i32, i32, (), &str) = (1, 2, (), "hello");
    // x contains string that cannot be copied
    // string literals are fiexed type and stored on stack 
    let y: (i32, i32, (), &str)= x;
    // for types that are stored on stack, it implicitly copy when assigned
    println!("{:?}, {:?}", x, y);
}
