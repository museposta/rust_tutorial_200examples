

use std::io::Write;
fn main() {
   let b1 = std::io::stdout().write("Christopher ".as_bytes()).unwrap();
   let b2 = std::io::stdout().write(String::from("Badici").as_bytes()).unwrap();
   std::io::stdout().write(format!("\nbytes written {}",(b1+b2)).as_bytes()).unwrap();
}