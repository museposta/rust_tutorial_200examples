// setup rust https://www.rust-lang.org/tools/install
// setup vscode https://code.visualstudio.com/download
// setup microsoft c++ build tools  https://visualstudio.microsoft.com/downloads/?q=build+tools   
//        make sure options/msvc and options/windows10sdk CHECKED.   

// cargo new hello-rust



// Why Rust?
//     Safety
//     Speed
//     Concurrency

// https://edu.anarcho-copy.org/Programming%20Languages/Rust/rust-programming-language-steve-klabnik.pdf


// cargo build
// Compiles the current project.
	
// cargo check
// Analyzes the current project and report errors, but don't build object files.

// cargo run
// Builds and executes src/main.rs.

// cargo clean
// Removes the target directory.

// cargo update
// Updates dependencies listed in Cargo.lock.

// cargo new
// Creates a new cargo project.


fn main() {
    println!("Hello Wrold");
}


// Create a binary 
// cargo new project_name --bin

// Create a library 
// cargo new project_name --lib

// To check the current version of cargo, execute the following command −
// cargo --version