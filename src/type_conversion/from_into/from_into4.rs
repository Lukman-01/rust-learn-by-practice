// // TryFrom and TryInto are included in `std::prelude`, so there is no need to introduce it into the current scope
// // use std::convert::TryInto;

// fn main() {
//     let n: i16 = 256;

//     // Into trait has a method `into`,
//     // hence TryInto has a method ?
//     let n: u8 = match n.__() {
//         Ok(n) => n,
//         Err(e) => {
//             println!("there is an error when converting: {:?}, but we catch it", e.to_string());
//             0
//         }
//     };

//     assert_eq!(n, __);

//     println!("Success!");
// }


// Similar to From and Into, TryFrom and TryInto are generic traits for converting between types.

// Unlike From/Into, TryFrom and TryInto are used for fallible conversions and 
// return a Result instead of a plain value.

fn main() {
    let n: i16 = 256;

    // Into trait has a method `into`,
    // hence TryInto has a method ?
    let n: u8 = match n.try_into() {
        // or: u8::try_from(n)
        Ok(n) => n,
        Err(e) => {
            println!(
                "there is an error when converting: {:?}, but we catch it",
                e.to_string()
            );
            0
        }
    };

    assert_eq!(n, 0);

    println!("Success!");
}