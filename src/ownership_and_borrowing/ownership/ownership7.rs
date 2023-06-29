
// fn main() {
//     let x = Box::new(5);
    
//     let ...      // Implement this line, dont change other lines!
    
//     *y = 4;
    
//     assert_eq!(*x, 5);

//     println!("Success!");
// }



fn main() {
    //Box is used to store variables directly to the heap.
    let x: Box<i32> = Box::new(5);
    // x holds the pointer of the heap that contains 5
    
    let mut y: Box<i32> = Box::new(1);     // Implement this line, dont change other lines!
    //since y holds the pointer address, to access the value of y, we have to direference it using * as shown below
    *y = 4;
    // that means we change the value 1 to 4
    assert_eq!(*x, 5);

    println!("Success!");
}
