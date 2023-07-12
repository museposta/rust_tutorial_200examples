

#[derive(PartialEq, Debug)]
struct Shoe {
 size: u32,
 style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
   

fn main(){
    let shoes = vec![
    Shoe { size: 12, style: String::from("sneaker") },
    Shoe { size: 13, style: String::from("nike") },
    Shoe { size: 12, style: String::from("sport") },
    ];
        let in_my_size = shoes_in_my_size(shoes, 12);
        println!("{:?}",in_my_size );

}


