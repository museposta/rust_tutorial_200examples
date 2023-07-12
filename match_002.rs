// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

// use std::io;
// use std::io::Read;
// use std::fs::File;

fn main() {
      let value = 2;

      match value {
        1 => println!("The value is 1."),
        2 => println!("The value is 2."),
        3 => println!("The value is 3."),
        _ => println!("The value is not 1, 2, or 3."),
      };

}