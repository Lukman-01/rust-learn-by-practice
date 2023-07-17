
// use std::fs;
// use std::io;
// use std::num;

// enum CliError {
//     IoError(io::Error),
//     ParseError(num::ParseIntError),
// }

// impl From<io::Error> for CliError {
//     // IMPLEMENT from method
// }

// impl From<num::ParseIntError> for CliError {
//     // IMPLEMENT from method
// }

// fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
//     // ? automatically converts io::Error to CliError
//     let contents = fs::read_to_string(&file_name)?;
//     // num::ParseIntError -> CliError
//     let num: i32 = contents.trim().parse()?;
//     Ok(num)
// }

// fn main() {
//     println!("Success!");
// }

// When performing error handling it is often useful to 
// implement From trait for our own error type. Then we can use ? 
// to automatically convert the underlying error type to our own error type.

use core::num;
use std::fs;
use std::io;

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(e: io::Error) -> CliError {
        CliError::IoError(e)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(e: num::ParseIntError) -> CliError {
        CliError::ParseError(e)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    // ? automatically converts io::Error to CliError
    let contents = fs::read_to_string(&file_name)?;
    // num::ParseIntError -> CliError
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

fn main() {
    println!("Success!");
}