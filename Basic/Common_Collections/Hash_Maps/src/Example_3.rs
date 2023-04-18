use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("Alice", 100);
    scores.insert("Bob", 50);
    scores.insert("Charlie", 75);

    scores.remove("Bob");

    for (name, score) in &scores {
        println!("{} has a score of {}", name, score);
    }
}
