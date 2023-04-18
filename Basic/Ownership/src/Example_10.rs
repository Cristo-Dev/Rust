fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..3]; // slice contains elements 2 and 3

    println!("{:?}", slice);
}
