

struct Person {
    age: u8,
    name: String,
    favorite_color: String,
  }
  
  fn print_person_info(person: &Person) {
    println!("Name: {}", person.name);
    println!("Favorite color: {}", person.favorite_color);
  }
  
  fn main() {
    let people = vec![
      Person {
        age: 10,
        name: "John".to_string(),
        favorite_color: "red".to_string(),
      },
      Person {
        age: 12,
        name: "Jane".to_string(),
        favorite_color: "blue".to_string(),
      },
      Person {
        age: 15,
        name: "Mary".to_string(),
        favorite_color: "green".to_string(),
      },
    ];
  
    for person in people {
      if person.age <= 10 {
        print_person_info(&person);
      }
    }
  }


