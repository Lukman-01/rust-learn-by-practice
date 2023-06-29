// // Make it work with two ways
// fn main() {
//     let v = {
//         let mut x = 1;
//         x += 2
//     };
 
//     assert_eq!(v, 3);
 
//     println!("Success!");
//  }
 

// Make it work with two ways: Method 1
fn main() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);
    //or using assert_eq!(v, ()); since x+=2 is a statement

    println!("Success!");
 }
 