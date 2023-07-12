fn main() {
    let s1 = String::from("hello Mars!");
    // we borrow the string.
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}
fn calculate_length(s: &String) -> usize {
    // attempt to change the string will cause error
    // s.push_str(", and World");
    s.len()
}
