use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("Alice", 100);
    scores.insert("Bob", 50);
    scores.insert("Charlie", 75);

    let alice_score = scores.get("Alice");

    match alice_score {
        Some(score) => println!("Alice's score is {}", score),
        None => println!("Alice's score is not available"),
    }
}
