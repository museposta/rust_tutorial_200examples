
struct GroceryItem {
    quantity: i32,
    id_number: i32,
  }
  
  fn print_quantity(grocery_item: &GroceryItem) {
    println!("The quantity of the grocery item is: {}", grocery_item.quantity);
  }
  
  fn print_id_number(grocery_item: &GroceryItem) {
    println!("The id number of the grocery item is: {}", grocery_item.id_number);
  }
  
  fn main() {
    let grocery_item = GroceryItem {
      quantity: 12,
      id_number: 100,
    };
  
    print_quantity(&grocery_item);
    print_id_number(&grocery_item);
  }


