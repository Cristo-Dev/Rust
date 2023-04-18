struct Person {
    name: String,
    age: u8,
}

fn main() {
    let people = vec![
        Person { name: String::from("Alice"), age: 30 },
        Person { name: String::from("Bob"), age: 25 },
        Person { name: String::from("Charlie"), age: 35 },
    ];

    let slice = &people[1..3]; // slice contains Bob and Charlie

    for person in slice {
        println!("Name: {}, Age: {}", person.name, person.age);
    }
}
