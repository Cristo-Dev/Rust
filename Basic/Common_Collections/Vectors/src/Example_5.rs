#[derive(Debug)]
enum Pet {
    Cat(String),
    Dog(String),
}

fn main() {
    // Create a new vector of Pet enums
    let mut pets = vec![
        Pet::Cat(String::from("Whiskers")),
        Pet::Dog(String::from("Fido")),
    ];

    // Add a new pet to the vector
    let new_pet = Pet::Cat(String::from("Fluffy"));
    pets.push(new_pet);

    // Print the vector of pets
    println!("The vector of pets is {:?}", pets);
}
