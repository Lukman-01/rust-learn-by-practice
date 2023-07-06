
// fn main() {
//     let boolean = true;

//     // Fill the blank with a match expression:
//     //
//     // boolean = true => binary = 1
//     // boolean = false =>  binary = 0
//     let binary = __;

//     assert_eq!(binary, 1);

//     println!("Success!");
// }


// Match is an expression, so we can use it in assignments.
fn main() {
    let boolean = true;

    // Fill the blank with a match expression:
    //
    // boolean = true => binary = 1
    // boolean = false =>  binary = 0
    let binary = match boolean{
        true => 1,
        _ => 0,
    };

    assert_eq!(binary, 1);

    println!("Success!");
}
