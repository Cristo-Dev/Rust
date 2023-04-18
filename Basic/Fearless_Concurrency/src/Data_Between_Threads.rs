use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let message = String::from("Hello from another thread!");
        tx.send(message).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Received message: {}", received);
}
