

// difference
use std::collections::HashSet;

fn main() {
    let hashset1 = HashSet::from([1, 2, 3, 4]);
    let hashset2 = HashSet::from([4, 3, 2]);
    
    // Difference between hashsets
    let result: HashSet<_> = hashset1.difference(&hashset2).collect();
    
    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("difference = {:?}", result);

    let hashset1 = HashSet::from([2, 7, 8]);
    let hashset2 = HashSet::from([1, 2, 7, 9]);
    
    // Symmetric difference of hashsets
    let result: HashSet<_> = hashset1.symmetric_difference(&hashset2).collect();
    
    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("symmetric difference = {:?}", result);


}
