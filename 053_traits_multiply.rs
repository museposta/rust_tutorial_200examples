


use std::ops;


fn multiply<T>(a: T, b: T) -> T
where
    T: std::ops::Mul<Output = T>,
{
    a * b
}

fn main() {
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));

    println!("Success!");
}