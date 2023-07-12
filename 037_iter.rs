




// iter = readonly, into_iter = takes ownership, iter_mut()=editable iter.

fn main(){
    let a = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i, v) in a.iter().enumerate() {
        println!("The {}th element is {}", i + 1, v);
    }


    let arr = [0; 10];
    for i in arr {
        println!("{}", arr[i]);
    }

    let mut v = Vec::new();
    for n in 0..100 {
        v.push(n);
    }

    assert_eq!(v.len(), 100);


    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);

    println!("Success...");


}