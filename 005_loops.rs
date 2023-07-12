// loop and while

fn main(){
    let mut limit = 10;
  
    loop{
        if limit == 280 {
            break;
        }
        println!(" the iterator is {}", limit);   
        limit  = limit + 1;

    }


    while limit < 5000 {
        println!(" the While iterator is {}", limit);   
        limit  = limit + 1;
    }

}