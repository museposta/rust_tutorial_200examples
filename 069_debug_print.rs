

#[derive(Debug)]
struct Structure(i32);

fn main() {
    // Types in std and Rust have implemented the fmt::Debug trait
    println!("There are {} days in a week.", 7);

    println!(" {:?} will print!", Structure(3));
}