
// // Remove something to make it work
// fn main() {
//     let x: i32 = 5;
//     let mut y: u32 = 5;

//     y = x;
    
//     let z = 10; // Type of z ? 

//     println!("Success!");
// }


// Remove something to make it work
#[allow(unused_variables)]

fn main() {
    let x: i32 = 5;
    let mut _y = 5;

    _y = x;
    
    let z = 10; // Type of z ? 

    println!("Success!");
    alias();
}


// // Fill the blank
// fn main() {
//     let v: u16 = 38_u8 as __;

//     println!("Success!");
// }


// Fill the blank
fn alias() {
    let _v: u16 = 38_u8 as u16;

    println!("Success!");
}


// // Modify `assert_eq!` to make it work
// fn main() {
//     let x = 5;
//     assert_eq!("u32".to_string(), type_of(&x));

//     println!("Success!");
// }

// // Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }


// Modify `assert_eq!` to make it work
fn types() {
    let x = 5;
    assert_eq!("u32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}



