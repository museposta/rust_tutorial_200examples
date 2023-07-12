
#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
  }

  impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
      match self {
        Color::Red => write!(f, "Red"),
        Color::Green => write!(f, "Green"),
        Color::Blue => write!(f, "Blue"),
      }
    }
  }
  
  struct Box {
    length: u32,
    width: u32,
    height: u32,
    color: Color,
  }
  
  impl Box {
    fn new(length: u32, width: u32, height: u32, color: Color) -> Box {
      Box {
        length,
        width,
        height,
        color,
      }
    }
  
    fn print_characteristics(&self) {
      println!(
        "The box is {}x{}x{} and is {}.",
        self.length, self.width, self.height, self.color
      );
    }
  }
  
  fn main() {
    let box1= Box::new(10, 20, 30, Color::Red);
    box1.print_characteristics();
  }


