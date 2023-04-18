fn main() {
    // Create a new vector of strings
    let fruits = vec!["apple", "banana", "orange"];

    // Iterate over the vector and print each element
    for fruit in &fruits {
        println!("{}", fruit);
    }
}
