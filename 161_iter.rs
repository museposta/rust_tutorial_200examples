


fn main() {
    let a = [10,20,30,40,50];
    let iter = a.iter();
    for data in iter{
       print!("{}\t",data);
    }

    let names = vec!["Mustafa", "Christopher", "John"];
    let  mut names_iter_mut= vec!["Mustafa", "Christopher", "John"];
    for name in names.iter() {
       match name {
          &"Mustafa" => println!("There is a rustacean among us!"),
          _ => println!("Hello {}", name),
       }
    }
    println!("{:?}",names); 
    // reusing the collection after iteration

     for name in names_iter_mut.iter_mut() {
        match name {
           &mut "Mustafa" => println!("There is a rustacean among us!"),
           _ => println!("Hello {}", name),
        }
     }
     println!("{:?}",names_iter_mut);
     //// reusing the collection after iteration

    for name in names.into_iter() {
        match name {
           "Mustafa" => println!("There is a rustacean among us!"),
           _ => println!("Hello {}", name),
        }
     }
     // cannot reuse the collection after iteration
     //println!("{:?}",names); 
     //Error:Cannot access after ownership move




 }

 
// iter()
// gives an iterator over &T(reference to T)

// into_iter()
// gives an iterator over T
	
// iter_mut()
// gives an iterator over &mut T