


// automatically implement some kind of functionality. this case debug print.
// clone and copy will make sure to make another copy and ownership is not transferred
// copy and clone should be used only for small enums and structs.
#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}
#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i32,
}

fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40,
    };
    //// derive macro. handles this case
    // match me.position{
    //     Position::Manager => println!("manager"),
    //     Position::Supervisor => println!("Supervisor"),
    //     Position::Worker => println!("Worker"),
    // }
    println!("{:?}{:?}", me.position, me.work_hours);
    println!("{:?}", me);
}