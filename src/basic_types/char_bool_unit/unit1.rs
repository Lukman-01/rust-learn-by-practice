
// // Make it work, don't modify `implicitly_ret_unit` !
// fn main() {
//     let _v: () = ();

//     let v = (2, 3);
//     assert_eq!(v, implicitly_ret_unit());

//     println!("Success!");
// }

// fn implicitly_ret_unit() {
//     println!("I will return a ()");
// }

// // Don't use this one
// fn explicitly_ret_unit() -> () {
//     println!("I will return a ()");
// }

// Make it work, don't modify `implicitly_ret_unit` !

// A unit type is a type in rust that does not hold any value i.e empty value ()
// When a function doesnot have a return statement, it return a unit type ()

fn main() {
    let _v: () = ();// declearing _v as a unit type

    let v = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success!");
}
fn implicitly_ret_unit() {
    println!("I will return a ()");
    // since it doesnt have a return statement, it return a unit type just as below function
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}
