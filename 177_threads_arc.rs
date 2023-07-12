


// For threads, you need std::sync::Arc - 'Arc' stands for 'Atomic Reference Counting'
// . That is, it guarantees that the reference count will be modified in one logical operation. 
// To make this guarantee, it must ensure that the operation is locked so that only the current thread has access.
// clone is still much cheaper than actually making a copy however.

// t
use std::thread;
use std::sync::Arc;

struct MyString(String);

impl MyString {
    fn new(s: &str) -> MyString {
        MyString(s.to_string())
    }
}

fn main() {
    let mut threads = Vec::new();
    let name = Arc::new(MyString::new("dolly"));

    for i in 0..5 {
        let tname = name.clone();
        let t = thread::spawn(move || {
            println!("hello {} count {}", tname.0, i);
        });
        threads.push(t);
    }

    for t in threads {
        t.join().expect("thread failed");
    }
}