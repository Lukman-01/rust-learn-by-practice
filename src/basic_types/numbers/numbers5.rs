
// // Fix errors and panics to make it work
// fn main() {
//     let v1 = 251_u8 + 8;
//     let v2 = i8::checked_add(251, 8).unwrap();
//     println!("{},{}",v1,v2);
//  }

// Fix errors and panics to make it work
fn main() {
    let v1 = 251_u16 + 8;
    let v2 = u8::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);
 }
 
//  In the first line, v1 is assigned the result of adding 251_u8 
//  (an unsigned 8-bit integer literal) to 8. Since 8 is an i32 
//  (32-bit signed integer) literal, the Rust compiler automatically 
//  promotes 251_u8 to i32 before performing the addition. 
//  The resulting value is then stored in v1, which is an u8 (unsigned 8-bit integer) variable.

//  In the second line, v2 is assigned the result of using the checked_add 
//  method on the i8 (signed 8-bit integer) type. The checked_add method is 
//  used to add two values and return an Option type. If the addition overflows 
//  or underflows, it returns None. In this case, the 251 and 8 values can be 
//  safely added without overflow, so checked_add returns Some(value). 
//  By calling unwrap() on the Option value, we extract the inner value. 
//  If the Option was None, it would panic and crash the program. 
//  Since we know the addition is safe in this case, we use unwrap() to retrieve the value.