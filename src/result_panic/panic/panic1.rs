
// // FILL the blanks
// fn drink(beverage: &str) {
//     if beverage == "lemonade" {
//         println!("Success!");
//         // IMPLEMENT the below code
//         __
//      }

//     println!("Exercise Failed if printing out this line!");
// }

// fn main() {
//     drink(__);

//     println!("Exercise Failed if printing out this line!");
// }

// The simplest error handling mechanism is to use panic. It just prints an error message and starts unwinding the stack, finally exit the current thread:

// if panic occurred in main thread, then the program will be exited.
// if in spawned thread, then this thread will be terminated, but the program won't

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // IMPLEMENT the below code
        panic!()
    }

    println!("Exercise Failed if printing out this line!");
}

fn main() {
    drink("lemonade");

    println!("Exercise Failed if printing out this line!");
}