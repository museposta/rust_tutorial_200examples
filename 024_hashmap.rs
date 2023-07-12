// key value pairs
// hashmap doesnot keep the order. it is in arbitrary order. vec keeps the order.
use std::collections::HashMap;

fn main() {
    let mut people = HashMap::new();
    people.insert("Alice", 30);
    people.insert("Bob", 40);
    people.insert("Carol", 50);

    for (name, age) in people.iter() {
        println!("{} is {} years old", name, age);
    }

    // Iterate over the keys.
    for key in people.keys() {
        println!("The key is {}", key);
    }

    // Iterate over the values.
    for value in people.values() {
        println!("The value is {}", value);
    }

    // Iterate over the key-value pairs.
    for (key, value) in people.iter() {
        println!("The key is {}, and the value is {}", key, value);
    }

    people.remove("Carol");

    match people.get("Bob") {
        Some(age) => println!("Bob is {} years old", age),
        None => println!("Bob has no age"),
    }

    // Check if Alice has an age.
    if let Some(age) = people.get("Alice") {
        println!("Alice is {} years old", age);
    } else {
        println!("Alice doesn't have an age");
    }

    // Check if Bob has an age.
    if let Some(age) = people.get("Bob") {
        println!("Bob is {} years old", age);
    } else {
        println!("Bob doesn't have an age");
    }

    // Check if Carol has an age.
    if let Some(age) = people.get("Carol") {
        println!("Carol is {} years old", age);
    } else {
        println!("Carol doesn't have an age");
    }

    let mut stocks = HashMap::new();
    stocks.insert("Apple", 100);
    stocks.insert("Microsoft", 200);
    stocks.insert("Amazon", 300);
    stocks.insert("Google", 0);

    // Print the stock price of Apple.
    let apple_stock = stocks.get("Apple");
    println!("The stock of Apple is {}", apple_stock.unwrap());

    let mut total_stock = 0;

    // Print the stock prices of all the companies.
    for (symbol, qty) in stocks.iter() {
        total_stock = total_stock + qty;
        let stock_count = if qty == &0 {
            "out of stock".to_owned()
        } else {
            format!("{:?}", qty)
        };
        println!("item {} stock {}", symbol, stock_count);
    }

    let teams = vec![String::from("Blue"), String::from("Yellow")];

    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("initial scores {:?} ", scores);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Only Insert If the Key Has No Value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    let text = "it may be a wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
    }
    println!("{:?}", map);








}

// give me a rust code example on hashmap.
// call it people and keep names as key and their ages as value.
// Check whether they have ages with match.
