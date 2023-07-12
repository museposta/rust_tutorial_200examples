
fn print_numbers_and_thirty() {
    let numbers = vec![10, 20, 30, 40];
    for number in numbers {
      if number == 30 {
        println!("thirty");
      } else {
        println!("{}", number);
      }
    }
  }
  
  fn print_number_of_elements_in_vector() {
    let numbers = vec![10, 20, 30, 40];
    let number_of_elements = numbers.len();
    println!("The number of elements in the vector is: {}", number_of_elements);
  }
  
  fn main() {
    print_numbers_and_thirty();
    print_number_of_elements_in_vector();
  }

