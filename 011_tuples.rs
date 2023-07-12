

// type of record.
// useful to return pairs of data from functions
// can be destructed easily into variables


fn return_three_values() -> (i32, i32, i32) {
    (10, 20, 30)
}

fn print_three_values(first_value: i32, second_value: i32, third_value: i32) {
    println!("The first value is {}", first_value);
    println!("The second value is {}", second_value);
    println!("The third value is {}", third_value);
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
   }


fn main() {
    let (first_value, second_value, third_value) = return_three_values();
    print_three_values(first_value, second_value, third_value);

    let coord = (5,6);
    println!("The first value is {}", coord.0);

    let (x,y) = coord;
    println!("The second value is {}", y);


    let rect1 = (30, 50);
    println!("The area of the rectangle is {} ",area(rect1));
    


}

// Write some code in rust. Please follow the instructions below.
// A function to return three values as tuple example.
// a print function to print those values.