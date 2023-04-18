use std::collections::HashMap;

fn main() {
    // Creating an empty hash map where keys are strings and values are integers
    let mut scores = HashMap::new();

    // Inserting key-value pairs into the hash map
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    // Accessing values by key
    let blue_score = scores.get(&String::from("blue"));
    println!("The blue team's score is {:?}", blue_score);

    // Overwriting a value
    scores.insert(String::from("blue"), 25);

    // Inserting a value if the key does not exist
    scores.entry(String::from("red")).or_insert(50);

    // Updating a value based on the old value
    let text = "hello world wonderful world";
    let mut word_counts = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", word_counts);
}
