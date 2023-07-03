
// // Fix the errors without adding or removing lines
// fn main() {
//     let names = [String::from("liming"),String::from("hanmeimei")];
//     for name in names {
//         // Do something with name...
//     }

//     println!("{:?}", names);

//     let numbers = [1, 2, 3];
//     // The elements in numbers are Copy，so there is no move here
//     for n in numbers {
//         // Do something with name...
//     }
    
//     println!("{:?}", numbers);
// } 


// Fix the errors without adding or removing lines
fn main() {
    let names: [String; 2]= [String::from("liming"),String::from("hanmeimei")];
    for name in &names {
        // Do something with name...
        println!("{:?}", name);
        // println will dereference it for us
    }

    println!("{:?}", names);

    let numbers :[i32; 3]= [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for n in numbers {
        // Do something with name...
        println!("{:?}", n);
    }
    
    println!("{:?}", numbers);
} 
