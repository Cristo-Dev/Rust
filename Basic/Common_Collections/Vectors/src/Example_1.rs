fn main() {
    // Create a new vector of integers
    let mut numbers = vec![1, 2, 3];

    // Access the second element in the vector
    let second = numbers[1];
    println!("The second number is {}", second);

    // Change the third element in the vector
    numbers[2] = 5;
    println!("The new vector is {:?}", numbers);
}
