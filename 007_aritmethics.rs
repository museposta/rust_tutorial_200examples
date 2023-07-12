fn add_numbers(a: i32, b: i32) -> i32 {
    let sum = a + b;
    println!("The sum of {} and {} is {}", a, b, sum);
     sum
  }

  fn subtruct_numbers(a: i32, b: i32) -> i32 {
    let difference = a - b;
    println!("The difference of {} and {} is {}", a, b, difference);
    difference
  }

  fn divide_numbers(a: i32, b: i32) -> i32 {
    let division = b / a;
    println!("The division of {} and {} is {}", a, b, division);
    division
  }

  fn multiply_numbers(a: i32, b: i32) -> i32 {
    let multiply = b * a;
    println!("The multiply of {} and {} is {}", a, b, multiply);
    multiply
  }

  fn remainder_numbers(a: i32, b: i32) -> i32 {
    let remainder = b % a;
    println!("The remainder of {} and {} is {}", a, b, remainder);
    remainder
  }
  
  fn main() {
    let a = 10;
    let b = 20;
    let sum = add_numbers(a, b);
    println!("The sum is: {}", sum);
    let difference = subtruct_numbers(b,a);
    println!("The difference is: {}", difference);

    let division = divide_numbers(b,a);
    println!("The division is: {}", division);

    let multiply = multiply_numbers(b,a);
    println!("The multiply is: {}", multiply);
    let remainder: i32 = remainder_numbers(b,a);
    println!("The remainder is: {}", remainder);

  }