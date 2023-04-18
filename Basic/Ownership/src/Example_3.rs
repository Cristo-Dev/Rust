fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // `s1` moves to `s2` and `s1` becomes invalid

    println!("{}", s1); // ERROR: value borrowed here after move
}
