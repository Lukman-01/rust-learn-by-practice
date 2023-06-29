
// fn main() {
//     let t = (String::from("hello"), String::from("world"));
 
//     let _s = t.0;
 
//     // Modify this line only, don't use `_s`
//     println!("{:?}", t);
//  }
 


fn main() {
    let t = (String::from("hello"), String::from("world"));
    let _s = t.0;
    // you can use let ref _s = t.0; for t to still hold the first string
    // Modify this line only, don't use `_s`
    println!("{:?}", t.1);
 }
 