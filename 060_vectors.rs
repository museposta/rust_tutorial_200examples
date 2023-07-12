
fn main() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 {
        println!("Iteration #{i}: {:?}", v.get(i))
    }

    for i in 0..5 {
        match v.get(i) {
            Some(e) => v[i] = e + 1,
            None => v.push(i + 2),
        }
    }

    assert_eq!(format!("{:?}", v), format!("{:?}", vec![2, 3, 4, 5, 6]));

    println!("Success!");

    let mut v = vec![1, 2, 3];

    // let slice1 = &v[..];
    // // Out of bounds will cause a panic
    // // You must use `v.len` here
    let slice2 = &v[0..v.len()];

    // Öassert_eq!(slice1, slice2);

    // Slices are read only
    // Note: slice and &Vec are different
    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3 = &mut v[0..];

    assert_eq!(slice3, &[1, 2, 3, 4]);

    println!("Success!");


}