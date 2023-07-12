
fn get_y_value(x: i32, y: i32) -> (i32, i32) {
    if y > 5 {
      (y, 0.to_owned())
    } else if y < 5 {
      (y, 1.to_owned())
    } else {
      (y, 2.to_owned())
    }
  }
  
  fn main() {
    let (y, y_status) = get_y_value(10, 15);
  
    match y_status {
      0 => println!("The y-value is greater than 5: {}", y),
      1 => println!("The y-value is less than 5: {}", y),
      2 => println!("The y-value is equal to 5: {}", y),
      _ => println!("The y-value is equal to 5: {}", y),
    }
  }

