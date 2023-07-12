enum Color{
    Red,
    Blue
}
// &Color=borrowind   Color=ownership
fn display_color(color: &Color){
    match color{
        Color::Red =>println!("The  color is Red"),
        Color::Blue =>println!("The  color is Blue"),
    }
}
fn borrow_data(){
    let color = Color::Blue;
    // &Color=borrowind   Color=ownership
    display_color(&color);
    display_color(&color);
}


fn ownership_example() {
    let string = "This is a string.".to_string();

    println!("The string is owned by the variable `string`: {}", string);

    let new_string = string;

    println!("The string is now owned by the variable `new_string`: {}", new_string);

    // // The variable `string` is no longer valid.
    // println!("The variable `string` is no longer valid: {}", string);
}

fn main() {
    ownership_example();
    borrow_data();
}


// Write some code in rust. Please follow the instructions below.
// a detailed sample to show ownership in rust.
