

// for ... in used for collections.




fn main() {
    let numbers = [1, 2, 3, 4, 5];

    for number in numbers {
        println!("{}", number);
    }

    for n in 1..100 {
        // will never panic since 100 is not included
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }


    let a = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i, v) in a.iter().enumerate() {
        println!("The {}th element is {}", i + 1, v);
    }



    for c in "here you are some chars".chars() {
        println!(" {} ",c);
    }


}