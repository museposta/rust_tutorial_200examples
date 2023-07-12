// slices donot have ownership. they let
// you reference a contiguous sequence of elements in a collection rather
// than the whole collection

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn main() {
    let s = String::from("hello Mars");
    let word = first_word(&s); // word will get the value
    println!("{:?}", s.get(..word).unwrap());
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{:?}", slice);
   
}
