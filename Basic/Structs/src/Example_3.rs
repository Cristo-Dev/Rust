struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    fn new() -> Self {
        Vector { data: Vec::new() }
    }

    fn push(&mut self, value: T) {
        self.data.push(value);
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
}

fn main() {
    let mut vec = Vector::new();
    vec.push(10);
    vec.push(20);
    vec.push(30);

    match vec.get(1) {
        Some(value) => println!("Value at index 1 is: {}", value),
        None => println!("Index out of bounds!"),
    }
}
