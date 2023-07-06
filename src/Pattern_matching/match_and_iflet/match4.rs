
// fn main() {
//     let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

//     // Fill the blank with `matches!` to make the code work
//     for ab in alphabets {
//         assert!(__)
//     }

//     println!("Success!");
// } 

// matches! looks like match, but can do something different.
fn main() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    // Fill the blank with `matches!` to make the code work
    for ab in alphabets {
        assert!(matches!(ab, 'a'..='z'| 'A'..='Z'| '0'..='9'))
        // or
        // assert!(matches!(ab, 'a'|'E'|'Z'|'0'|'x'|'9'|'Y'))
    }

    println!("Success!");
} 
