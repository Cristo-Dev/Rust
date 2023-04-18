#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    // Create a new vector of Person structs
    let mut people = vec![
        Person { name: String::from("Alice"), age: 30 },
        Person { name: String::from("Bob"), age: 25 },
    ];

    // Add a new person to the vector
    let new_person = Person { name: String::from("Charlie"), age: 20 };
    people.push(new_person);

    // Print the vector of people
    println!("The vector of people is {:?}", people);
}
