// Fix the warning below with :
// 🌟 Only one solution
// 🌟🌟 Two distinct solutions
// Note: none of the solutions is to remove the line let x = 1


// fn main() {
//     let x = 1; 
// }

// // Warning: unused variable: `x`
#[allow(unused_variables)]

fn main() {
    let x = 1; 
}

// solution1: put underscore infront of x, like _x
// solution2: add #[allow(unused_variables)] at the top of the code
