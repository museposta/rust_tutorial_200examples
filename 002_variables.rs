





// Variables are a fundamental part of any programming language. They are used to store data, 
// and they can be used to represent anything from numbers to strings to complex objects.
// In Rust, variables are declared using the let keyword. The let keyword is followed by the name of the variable, 
// and then the data type of the variable. Variables can be immutable or mutable, and they can be shadowed.


/// integers
/// i8, 16,32,64,128, u8, ... isize, usize (arch/itecture)
/// 
/// 
// Decimal 98_222
// Hex 0xff
// Octal 0o77
// Binary 0b1111_0000
// Byte (u8 only) b’A’

// The String data type in Rust can be classified into the following −
// String Literal(&str)
// String Object(String)



fn main() {
    let one = 1;
    let hello = "hello";
    let a = 'a';
    let half = 0.5;
    let mut your_name = "Jane";
    let is_lady = true;
    let _your_half = half;
    let (x,y) = (3,4);
    let s = String::from("hello");

    const USER_LIMIT:i32 = 100;    // Declare a integer constant
    const PI:f32 = 3.14;           //Declare a float constant
 
    println!("user limit is {}",USER_LIMIT);  //Display value of the constant
    println!("pi value is {}",PI);            //Display value of the constant


  }
