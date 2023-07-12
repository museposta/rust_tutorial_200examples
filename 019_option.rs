

// data may or maynot at present. optional. not required.

#[derive(Debug)]
struct Person{
    age: Option<i32>,
    name: String,
}

// struct ShoppingList{
//     name: String,
//     amount: i32,
// }

// fn find_amount(name: &str) -> Option<i32>{
//     let shoppingList = vec![
//         ShoppingList{name: "apples".to_owned(), amount: 22},
//         ShoppingList{name: "milk".to_owned(), amount: 2},
//         ShoppingList{name: "sausages".to_owned(), amount: 1},
//     ];
//     for item in shoppingList {
//         if item.name == name {
//             return Some(item.amount);
//         }
//     }
//     None
// }

struct Survey{
    question1: Option<i32>,
    question2: Option<bool>,
    question3: Option<String>,
}

fn main() {

    let survey = Survey{
        // question1: Some(10),
        question1: None,
        question2: Some(false),
        question3: Some("Yes".to_owned()),
    };

    match survey.question1 {
        Some(answer) => println!("{:?} question1 ",answer),
        None => println!("question1 NO ANSWER "),
    }
    match survey.question2 {
        Some(answer) => println!("{:?} question2 ",answer),
        None => println!("question2 NO ANSWER "),
    }
    match survey.question3 {
        Some(answer) => println!("{:?} question3 ",answer),
        None => println!("question3 NO ANSWER "),
    }
    // let mut amount = find_amount((&"milk".to_owned()));
    // match amount {
    //     Some(qty)=>println!("amount of milk {}",amount),
    //     None=>println!("no amount of milk {}",amount),
    // };
    
    let some_number = Some(10);
    // let no_number = None;

    // Check if some_number has a value.
    if let Some(number) = some_number {
        println!("The number is {}", number);
    } else {
        println!("There is no number");
    }

    // // Check if no_number has a value.
    // if let Some(number) = no_number {
    //     println!("The number is {}", number);
    // } else {
    //     println!("There is no number");
    // }
    let jane = Person { age: None, name: "jane".to_owned()};
    match jane.age {
        Some(age) => println!("{:?} jane has age ",jane),
        None => println!("{:?} jane has no age ",jane),
    }


    let john = Person { age: Some(22), name: "John".to_owned()};
    println!("{:?}", john);


    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;


}