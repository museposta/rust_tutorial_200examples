// Traits: Defining Shared Behavior

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main() {
    let number_list = vec![45, 78, 99, 12, 22];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['5', 'm', 'w', 'r'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
