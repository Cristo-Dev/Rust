use std::sync::Arc;
use std::thread;

struct SharedData {
    data: Vec<i32>,
}

impl SharedData {
    fn new() -> Self {
        SharedData { data: vec![0, 0, 0] }
    }

    fn increment(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] += 1;
        }
    }

    fn get_data(&self) -> &[i32] {
        &self.data
    }
}

fn main() {
    let data = Arc::new(SharedData::new());

    let handles = (0..10)
        .map(|_| {
            let data = data.clone();
            thread::spawn(move || {
                let mut val = data.lock().unwrap();
                val.increment();
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final data: {:?}", data.get_data());
}
