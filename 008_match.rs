


fn main() {

    let can_vote = true;
    match can_vote {
        true => println!("Can vote"),
        false => println!("Cannot vote"),
    }

    let int = 2;
    match int {
        1 => println!("it is one"),
        2 => println!("it is two"),
        3 => println!("it is three"),
        _ => println!("it is some other number"),
    }

    let u8_value = Some(0u8);
    match u8_value {
    Some(0) => println!("zero"),
    _ => (),
    }


}