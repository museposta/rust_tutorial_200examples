

// HashSet implements the set data structure in Rust. 
// Just like a set, it allows us to store values without duplicates.

// Method	        Description
// len()	        returns the length of a hashset
// is_empty()	    checks if the hashset is empty
// clear()	        removes all elements from the hashset
// drain()	        returns all the elements as an iterator and clears the hashset

// import HashSet from Rust standard collections library
use std::collections::HashSet;

fn main() {
    // create a new HashSet
    let mut colors: HashSet<&str> = HashSet::new();

    // insert elements to hashset
    colors.insert("Red");
    colors.insert("Yellow");
    colors.insert("Blue");
    
    println!("HashSet = {:?}", colors);

        // check for a value in a HashSet
        if colors.contains("Red") {
            println!("We have the color \"Red\" in the HashSet.")
        }
    // remove value from a HashSet
    colors.remove("Yellow");
    
    println!("colors after remove operation = {:?}", colors);

        // iterate over a hashset
        for color in colors {
            // print each value in the hashset
            println!("{}", color);
        }

        // Create HashSet with default set of values using from() method
        let numbers = HashSet::from([2, 7, 8, 10]);
        
        println!("numbers = {:?}", numbers);

        let hashset1 = HashSet::from([2, 7, 8]);
        let hashset2 = HashSet::from([1, 2, 7]);
        
        // Union of hashsets
        let result: HashSet<_> = hashset1.union(&hashset2).collect();
        
        println!("hashset1 = {:?}", hashset1);
        println!("hashset2 = {:?}", hashset2);
        println!("union = {:?}", result);

        

}