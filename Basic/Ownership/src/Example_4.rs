fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // a reference to `s1` is passed to the function `calculate_length`

    println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize { // `s` is a reference to a string owned by another variable
    s.len() // `.len()` method is called on the reference `s`
}
