use std::fs::File;
use std::fs::read;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn read_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    println!("the contents of the file : {:?}", s);
    Ok(s)
}

fn main() {
    // let f = File::open("hello.txt").unwrap();
    // OR
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");
    // OR
    let contents = read_file();
    println!("the contents of the file : {:?}", contents.unwrap());

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => {
                panic!(
                    "Tried to create file but there
        was a problem: {:?}",
                    e
                )
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file:
        {:?}",
                error
            )
        }
    };
}
