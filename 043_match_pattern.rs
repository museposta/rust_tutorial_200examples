fn match_number(n: i32) {
    match n {
        // Match a single value
        1 => println!("One!"),
        2 | 3 | 4 | 5 => println!("match 2 -> 5"),
        6..=10 => {
            println!("match 6 -> 10")
        }
        _ => {
            println!("match -infinite -> 0 or 11 -> +infinite")
        }
    };
}

fn main() {

    match_number(4i32);
}