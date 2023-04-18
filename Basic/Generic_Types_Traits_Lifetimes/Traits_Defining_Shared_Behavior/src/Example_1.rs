trait Printable {
    fn print(&self);
}

struct Person {
    name: String,
    age: u32,
}

impl Printable for Person {
    fn print(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }
}

fn main() {
    let person = Person { name: "Alice".to_string(), age: 25 };
    person.print();
}
