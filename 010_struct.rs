


struct Person {
    name: String,
    age: u8,
}
struct Point {
    x: i32,
    y: i32,
}

enum Flavor {
    Vanilla,
    Chocolate,
    Strawberry,
    Mint,
}

struct Drink {
    name: String,
    flavor: Flavor,
}


fn main() {
    let person = Person {
        name: "John Doe".to_string(),
        age: 21,
    };

    println!("The person's name is {} and they are {} years old", person.name, person.age);

    let point = Point { x: 10, y: 20 };

    println!("The point's x-coordinate is {}", point.x);
    println!("The point's y-coordinate is {}", point.y);

    let drink = Drink {
        name: "Vanilla milkshake".to_string(),
        flavor: Flavor::Vanilla,
    };

    match drink.flavor {
        Flavor::Vanilla => println!("Vanilla milkshake"),
        Flavor::Chocolate => println!("Chocolate milkshake"),
        Flavor::Strawberry => println!("Strawberry milkshake"),
        Flavor::Mint => println!("Mint milkshake"),
    }


}