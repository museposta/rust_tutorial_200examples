

pub trait Computer{
    fn get_answer(&self)-> u8;
}

pub struct DeepThought;

impl Computer for DeepThought{
    fn get_answer(&self)-> u8{
        31
    }
}


fn main(){
    let e = DeepThought{};
    let _result = e.get_answer();
    println!("{:?}", _result);

}
