#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: "Mustafa".to_string(),
        age: 22,
    };

    /* will look like:
    Person {
        name: "Mustafa",
        age: 22,
    }
    use println!("{:#?}", person); instead of println!("{:?}", person);
    */
    println!("{:#?}", person);
}