

trait Print {
    fn print(&self) -> String;
}

impl Print for i32 {
    fn print(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

impl Print for f64 {
    fn print(&self) -> String {
        format!("eight-byte float {}", self)
    }
}

fn main() {
    let number = 22;
    let pi = 3.14;
    let s1 = number.print();
    let s2 = pi.print();
    println!("print {}", s1);
    println!("print {}", s2);
}