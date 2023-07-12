
    fn add(a:i32, b:i32) -> i32{
        a + b
    }

    fn display_first_name(){
        println!("Mustafa")
    }

    fn display_last_name(){
        println!("Badici")
    }

fn main() {

    let number = add(1,2);
    display_first_name();
    display_last_name();
    println!("the total is {}", number);
  }