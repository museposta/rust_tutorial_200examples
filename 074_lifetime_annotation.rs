

// lifetime annotatition = 'a
// without the lifetime annotation the code will fail to compile.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x: &str = "cat";
    let y: &str = "cattle";

    println!("{}", longest(x, y));
}