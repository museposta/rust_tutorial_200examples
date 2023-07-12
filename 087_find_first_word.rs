fn first_w(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    // create some strings
    let str1 = "Educative is the best platform!";
    let str2 = "Rust";
    let str3 = "Welcome to Edpresso";
    let str4 = "Programming";

    let first_word = str1.split_whitespace().next().unwrap_or("");

    println!("{}", first_word);

    // create the matches
    let match1 = "is";
    let match2 = 'R';
    let match3 = "to";
    let match4 = "23";

    // find the matches and print byte indices
    println!(" {:?}", str1.find(match1));
    println!(" {:?}", str2.find(match2));
    println!(" {:?}", str3.find(match3));
    println!(" {:?}", str4.find(match4));

    // let string: String = b"This is the first word.";
    // let first_word_index = string.find(b' ').unwrap();
    // let first_word = std::str::from_utf8_unchecked(string).unwrap()[..first_word_index];
    // println!("The first word is: {}", first_word);

    // first char
    let text = "hello Mars!";
    let ch = text.chars().next().unwrap();
    // let ch = text.chars().nth(0).unwrap(); // nth char
    // let ch = &text[0..1]; // this returns "h"
    println!(" the first char is  {:?}", ch);


    let mut s = String::from("hello world");
    let word = first_w(&s);
    // s.clear(); // error!
    println!("the first word is: {}", word);





}
