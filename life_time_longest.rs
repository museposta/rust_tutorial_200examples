fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string_one = String::from("full stack web developer");
    {
        let string_two = String::from("system analyst");
        let the_longest_string = longest(string_one.as_str(), string_two.as_str());
        println!("The longest string is {}", the_longest_string);
    }
}
