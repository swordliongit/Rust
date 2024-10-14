use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// #[test]
fn main() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        tx.send("Hello from child!").unwrap();
    });

    // handle.join().unwrap();

    let msg = rx.recv().unwrap();
    println!("Main thread received: {msg}");
}
