

use std::fmt::Display;

fn main(){
   print_pro(10 as u8);
   print_pro(20 as u16);
   print_pro("Hello Marsian Messi");
}

fn print_pro<T:Display>(t:T){
   println!("Inside print_pro function:");
   println!("{}",t);
}