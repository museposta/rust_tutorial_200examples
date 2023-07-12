


//fn closure. readony by ref. fnmut takes a mutuble reference. fnonce takes the ownership. by value

// return a closure
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
   }

fn main() {
    let mut s = String::new();

    let update_string = |str| s.push_str(str);

    exec(update_string);

    println!("{:?}", s);
}

fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    // or: FnOnce(&'a str)
    f("hello")
}
