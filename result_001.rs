

// use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
struct Adult {
    name: String,
    age: u8,
}

impl Adult {
    fn new(name: String, age: u8) -> Result<Self, String> {
        if age < 21 {
            return Err("Adult must be 21 or older".to_string());
        }

        Ok(Adult { name, age })
    }
}

fn main() {
    let adult_under_21 = Adult::new("John Doe".to_string(), 20);
    let adult_over_21 = Adult::new("Jane Doe".to_string(), 22);

    match adult_under_21 {
        Ok(adult) => println!("Adult under 21: {:?}", adult),
        Err(error) => println!("Error: {}", error),
    };

    match adult_over_21 {
        Ok(adult) => println!("Adult over 21: {:?}", adult),
        Err(error) => println!("Error: {}", error),
    };
}

