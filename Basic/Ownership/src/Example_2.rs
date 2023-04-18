fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // a copy of `s1` is created

    println!("s1 = {}, s2 = {}", s1, s2); // s1 is still valid and has not moved
}
