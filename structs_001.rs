
#[allow(dead_code)]
enum Flavor {
    Coke,
    Pepsi,
    Sprite,
    Fanta,
    DrPepper,
  }

  impl std::fmt::Display for Flavor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
      match self {
        Flavor::Coke => write!(f, "Coke"),
        Flavor::Pepsi => write!(f, "Pepsi"),
        Flavor::Sprite => write!(f, "Sprite"),
        Flavor::Fanta => write!(f, "Fanta"),
        Flavor::DrPepper => write!(f, "Dr. Pepper"),
      }
    }
  }

  
  struct Drink {
    flavor: Flavor,
    fluid_ounces: u32,
  }
  
  fn print_drink(drink: &Drink) {
    println!("The drink is {} and it contains {} fluid ounces.", drink.flavor, drink.fluid_ounces);
  }
  
  fn main() {
    let drink = Drink {
      flavor: Flavor::Coke,
      fluid_ounces: 12,
    };
  
    print_drink(&drink);
  
    match drink.flavor {
      Flavor::Coke => println!("This is a Coke."),
      Flavor::Pepsi => println!("This is a Pepsi."),
      Flavor::Sprite => println!("This is a Sprite."),
      Flavor::Fanta => println!("This is a Fanta."),
      Flavor::DrPepper => println!("This is a Dr. Pepper."),
    };
  }

