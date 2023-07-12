


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
        return &s[0..i];
      }
    }
    &s[..]
  }

  fn main() {
    let string = "This is the first word.";
    let first_word = first_word(string);
    println!("{}", first_word);
  }
