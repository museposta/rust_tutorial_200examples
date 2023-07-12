#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn create_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user = create_user(String::from("mustafa@mustafa.com"), String::from("mustafa"));
    println!("{:?}", user);

    let user2 = User {
        email: String::from("another@another.com"),
        username: String::from("another"),
        ..user
    };

    println!("{:?}", user2);
}
