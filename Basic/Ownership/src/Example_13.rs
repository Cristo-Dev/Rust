fn main() {
    let mut numbers = [1, 2, 3, 4, 5];
    let slice = &mut numbers[1..3]; // slice contains elements 2 and 3

    slice[0] = 0;
    slice[1] = 0;

    println!("{:?}", numbers);
}
