


// ansi_term

use ansi_term::Colour;
use ansi_term::Style;

fn main() {
    println!("This is {} in color, {} in color and {} in color",
             Colour::Red.paint("red"),
             Colour::Blue.paint("blue"),
             Colour::Green.paint("green"));

              println!("{} and this is not",
             Style::new().bold().paint("This is Bold"));

             println!("{}, {} and {}",
             Colour::Yellow.paint("This is colored"),
             Style::new().bold().paint("this is bold"),
             Colour::Yellow.bold().paint("this is bold and colored"));

}