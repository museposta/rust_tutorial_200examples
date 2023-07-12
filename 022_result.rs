use std::num::ParseIntError;


fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1: i32 = n1_str.parse()?;
    let n2: i32 = n2_str.parse()?;

    Ok(n1 * n2)
}

// functions can be failed. handle with result.
#[derive(Debug)]
struct Adult{
    age: u8,
    name: String,
}
impl Adult {
    fn new(age: u8, name:&str)->Result<Self, &str>{
        if age > 21 {
            Ok(Self { age: age, name: name.to_string()  })
        } else {
            Err("Age must be at least 21.")
        }

    }
}






#[derive(Debug)]
enum Menu{
    MainMenu,
    Start,
    Quit,
}


fn get_menu_choice(var: &str) -> Result<Menu, String>{
    match var {
        "mainmenu"=>Ok(Menu::MainMenu),
        "start"=>Ok(Menu::Start),
        "quit"=>Ok(Menu::Quit),
        _ => Err("menu item is not found".to_owned()),
    }
}

fn print_menu(var: &Menu){
    println!("{:?}", var)
}

fn main() {
    let choice = get_menu_choice("mainmenu");
    match choice {
        Ok(choice)=>print_menu(&choice),
        Err(e) => println!(" there was an error {:?}", e),
    }

    let child = Adult::new(16, "Bonita");
    let adult = Adult::new(31, "Jane");
    match child{
        Ok(child)=>println!("{} is {} years old ", child.name, child.age),
        Err(e)=>println!("{e}"),
    }
    match adult{
        Ok(adult)=>println!("{} is {} years old ", adult.name, adult.age),
        Err(e)=>println!("{e}"),
    }

    assert_eq!(multiply("3", "4").unwrap(), 12);
    println!("Success!");


}