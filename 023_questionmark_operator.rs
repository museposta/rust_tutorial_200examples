

// functions can be failed. handle with result.
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

// // instead of the below code, we can use question mark operator.
// match choice {
//     Ok(choice)=>print_menu(&choice),
//     Err(e) => println!(" there was an error {:?}", e),
// }                               if the result is ok, return nothing else returns error.
fn pick_menu(input: &str) -> Result<(), String>{
    let choice = get_menu_choice(input)?;
    print_menu(&choice);
    Ok(())
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
    pick_menu("start");
    let result = pick_menu("other");
    println!("{:?}", result);

}