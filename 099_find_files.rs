

use glob::glob;
use std::error::Error;
use std::io::{self, ErrorKind};

fn main() -> Result<(), Box<dyn Error>> {
    for entry in glob("C:/Users/musep/OneDrive/Resimler/*.png")? {
        println!("{}", entry?.display());
    }

    Ok(())
}