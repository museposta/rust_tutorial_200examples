

use std::fmt;

struct Structure(i32);

struct Deep(Structure);

impl fmt::Debug for Deep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.0)
    }
}

fn main() {
    // with `derive` is there is no control over how to print  

    /* Now 7 will print */
    println!("Now {:?} will print!", Deep(Structure(7)));
}