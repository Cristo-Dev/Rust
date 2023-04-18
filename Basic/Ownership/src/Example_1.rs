fn main() {
    let s = String::from("hello"); // s comes into scope

    println!("{}", s); // s is still valid and has not moved
} // Here, s goes out of scope and `drop()` is called automatically, freeing the memory occupied by `s`
