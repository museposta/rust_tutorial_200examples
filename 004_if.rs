fn main() {

    let age = 8;
    if age > 10 {
       println!("Your age is {:?}", age); 
    } else if age < 10 {
        println!("Your age is smallar than 10. Your age is {:?}", age); 
    } else  {
        println!("Your age is 10"); 
    }


    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

  }