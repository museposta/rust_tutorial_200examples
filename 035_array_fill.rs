

fn main() {
    let mut array = [0; 10];
    array.fill(1);
    for i in 0..10 {
      println!("array[{}] = {}", i, array[i]);
    }


    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    for month in months{
      println!(" month : {}", month);
    }

  }