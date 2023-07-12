


use std::fmt;

struct Student {
    name: String,
    locker_number: Option<i32>,
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(locker_number) = self.locker_number {
            write!(f, "Student {} (locker number: {})", self.name, locker_number)
        } else {
            write!(f, "Student {} (no locker assigned)", self.name)
        }
    }
}

fn main() {
    let student1 = Student {
        name: "John Doe".to_string(),
        locker_number: Some(123),
    };

    let student2 = Student {
        name: "Jane Doe".to_string(),
        locker_number: None,
    };

    println!("{}", student1);
    println!("{}", student2);
}

