

// array like. holds the same data type. vectors keeps the order.
struct Scores {
    score: i32,
}

enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}
fn is_vec(v: &Vec<u8>) {}
fn main() {


    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);

    let mut v2 = Vec::new();
    v2.extend(&v1);

    assert_eq!(v1, v2);

    println!("Success!");


    let mut vector = vec![1, 2, 3, 4, 5];
    // Add an item to the end of the vector.
    vector.push(6);
    // Insert an item at a specific index.
    vector.insert(2, 7);
    // Delete an item at a specific index.
    vector.remove(2);
    // Print the items in the vector.
    for item in vector {
        println!("{}", item);
    }

    let scores = vec![
        Scores {score: 95},
        Scores {score: 66},
        Scores {score: 74},
        Scores {score: 52},
        Scores {score: 99},
    ];

    for score in &scores {
        println!("{:?}", score.score);
    }
    println!("{:?}", scores.len());

    // tickets
    let tickets = vec![
         {Ticket::Backstage(100.0, String::from("John Doe"))},
         {Ticket::Standard(20.0)},
         {Ticket::Vip(50.0, String::from("Jane Doe"))},
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, name) => println!(" name {:?}  parice {:?}", name,price),
            Ticket::Standard(price) => println!("  parice {:?}", price),
            Ticket::Vip(price, name) => println!(" name {:?}  parice {:?}", name,price),
        }
    }

    let arr: [u8; 3] = [1, 2, 3];

    let v = Vec::from(arr);
    is_vec(&v);

    let v = vec![1, 2, 3];
    is_vec(&v);

    // vec!(..) and vec![..] are same macros, so
    let v = vec![1, 2, 3];
    is_vec(&v);

    // In code below, v is Vec<[u8; 3]> , not Vec<u8>
    // USE Vec::new and `for` to rewrite the below code
    let mut v1 = vec![];

    for i in &v {
        v1.push(*i);
    }
    is_vec(&v1);

    assert_eq!(v, v1);


    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
     .iter()
     .map(ToString::to_string)
     .collect();

     println!("list_of_strings {:?}", list_of_strings);

    println!("Success!");

}

// please write some rust code with the following instructions.
// give an example of vector. add, delete, insert, remove and print the items.