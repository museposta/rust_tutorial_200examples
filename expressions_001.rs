

fn print_line(message:String)-> String{
    println!("The result is {}",message);
    message
}

fn main() {
    let num = 101;
    let itsbig = num >= 100;
    if itsbig {
        print_line("its big".to_owned());
    } else {
        print_line("its small".to_owned());
    }
}
