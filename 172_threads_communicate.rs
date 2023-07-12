


use std::sync::mpsc;
use std::thread;

fn main() {
    // main thread starts here
    // create a new channel
    let (sender, receiver) = mpsc::channel();

    // spawn a new thread
    let handle = thread::spawn(move || {
        // receive message from channel
        let message = receiver.recv().unwrap();

        println!("Received message: {}", message);
    });

    let message = String::from("Hello, Neptun!");
    // send message to channel
    sender.send(message).unwrap();

    // wait for spawned thread to finish
    handle.join().unwrap();
}