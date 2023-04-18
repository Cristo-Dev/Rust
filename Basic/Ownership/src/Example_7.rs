fn main() {
    let s1 = String::from("hello");
    let s2 = transfer_ownership(s1); // `s1` is moved to `transfer_ownership` function
    println!("{}", s2); // `s2` now owns the string and can use it

    let s3 = String::from("world");
    let len = calculate_length(&s3); // `calculate_length` borrows `s3` without taking ownership
    println!("The length of '{}' is {}.", s3, len);
}

fn transfer_ownership(s: String) -> String { // `s` takes ownership of the passed string and returns ownership
    s // `s` is returned, transferring ownership back to the calling function
}

fn calculate_length(s: &String) -> usize { // `s` borrows the passed string, and returns its length without taking ownership
    s.len()
}
