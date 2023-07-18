// fn main() {
//     println!("{argument}", argument = "test"); // => "test"

//     /* Fill in the blanks */
//     assert_eq!(format!("{name}{}", 1, __), "21");
//     assert_eq!(format!(__,a = "a", b = 'b', c = 3 ), "a 3 b");
    
//     /* Fix the error */
//     // Named argument must be placed after other arguments
//     println!("{abc} {1}", abc = "def", 2);

//     println!("Success!");
// }


fn main() {
    println!("{argument}", argument = "test"); // => "test"

    /* Fill in the blanks */
    assert_eq!(format!("{name}{}", 1, name = 2), "21");
    assert_eq!(format!("{a} {c} {b}", a = "a", b = 'b', c = 3), "a 3 b");

    /* Fix the error */
    // Named argument must be placed after other arguments
    println!("{abc} {0}", 2, abc = "def");

    println!("Success!");
}