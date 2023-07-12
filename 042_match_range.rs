

fn main() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

   
    for item in alphabets {
        assert!(matches!(item, 'A'..='Z' | 'a'..='z' | '0'..='9'));
    }

    println!("Success!");
}
