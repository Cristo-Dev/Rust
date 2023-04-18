use std::sync::Mutex;
use std::thread;

fn main() {
    let data = Mutex::new(0);

    let handles = (0..10)
        .map(|_| {
            let data = data.clone();
            thread::spawn(move || {
                let mut val = data.lock().unwrap();
                *val += 1;
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final value: {}", *data.lock().unwrap());
}
