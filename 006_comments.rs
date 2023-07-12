
// this is the main function of 
// the application 



// Line comment
//! Inner line doc comment
/// Outer line doc comment
/*...*/            /* Block comment  */ 
/*!...*/          /*   Inner block doc comment */ 
/**...*/         /*    Outer block doc comment */ 


fn main(){
    // limit variable will be used to exit the loop.
    let mut limit = 10;
  
    loop{
        if limit == 280 {
            break;
        }
        // prints the limit
        println!(" the iterator is {}", limit);   
        limit  = limit + 1;

    }

    /* whild is one of the loop and it always have a conditions */
    while limit < 5000 {
        println!(" the While iterator is {}", limit);   
        limit  = limit + 1;
    }

}