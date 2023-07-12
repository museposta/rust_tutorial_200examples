




// &i32 // a reference
// &'a i32 // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime


fn main() {  
    {
        let outer;               
                            
        {                     
            let inner = 5;       
            outer = &inner;      
            // this will give compiler error: ^^^^^^ borrowed value does not live long enough     
        }                     
                              
        println!("outer: {}", outer);
    }                         
}
