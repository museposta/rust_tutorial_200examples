

fn add_numbers(a: i32, b: i32) -> i32 {
    let sum = a + b;
    println!("The sum of {} and {} is {}", a, b, sum);
     sum
  }
  
  fn main() {
    let a = 10;
    let b = 20;
    let sum = add_numbers(a, b);
    println!("The sum is: {}", sum);
  }
