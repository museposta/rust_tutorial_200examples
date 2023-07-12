

// Enumeration
enum Mouse{
    LeftClick,
    RightClick,
    MiddleClick,
    Scroll(i32),
    Move(i32,i32),
}
enum Direction{
    Up,
    Down,
    Left,
    Right
}

fn which_way_to_go(go: Direction) ->String{
    match go{
        Direction::Up => "up".to_string(),
        Direction::Down => "down".to_string(),
        Direction::Right => "right".to_string(),
        Direction::Left => "left".to_string(),
    }
}


fn print_which_way_to_go(which_way: String){
    println!("which way to go {}", which_way);
}

fn main(){
    let go = which_way_to_go(Direction::Down);
    print_which_way_to_go(go);
    
}