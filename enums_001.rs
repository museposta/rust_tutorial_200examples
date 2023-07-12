
#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Black,
    White,
  }
  
  fn main() {
    let color = Color::Red;
  
    match color {
      Color::Red => println!("The color is red."),
      Color::Green => println!("The color is green."),
      Color::Blue => println!("The color is blue."),
      Color::Yellow => println!("The color is yellow."),
      Color::Black => println!("The color is black."),
      Color::White => println!("The color is white."),
    };
  }