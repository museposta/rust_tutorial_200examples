

use std::collections::HashMap;


fn main() {
    let mut stock: HashMap<String, u32> = HashMap::new();

    stock.insert("Chairs".to_string(), 5);
    stock.insert("Beds".to_string(), 3);
    stock.insert("Tables".to_string(), 2);
    stock.insert("Couches".to_string(), 0);

    let number: u32 = 0;
    let reference_to_number: &u32 = &number;

    for (item, quantity) in stock.iter() {     
        let mut quantity_string = "out of stock".to_string();
       
         if quantity > reference_to_number {  quantity_string= format!("{}", quantity);}

        println!("{}: {}", item, quantity_string);
    };

    let total_items: u32 = stock.values().sum();
    println!("Total number of items in stock: {}", total_items);
}

