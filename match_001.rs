// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value
fn main() {
    let value = 2;
  
    match value {
      1 => println!("The value is One."),
      2 => println!("The value is Two."),
      3 => println!("The value is three."),
      _ => println!("The value is not 1, 2, or 3."),
    };
  }
