
// fn main() {
//     let t = (String::from("hello"), String::from("world"));
 
//      // Fill the blanks
//      let (__, __) = __;
 
//      println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
//  }



fn main() {
    let t = (String::from("hello"), String::from("world"));
 
     // Fill the blanks
     let (ref s1, ref s2) = t;
     // you can also use let (s1, s2) = t.clone();
 
     println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
 }
 