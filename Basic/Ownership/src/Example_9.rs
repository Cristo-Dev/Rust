fn main() {
    let s1 = String::from("hello");
    let s2 = modify_string(s1); // `s1` is moved to `modify_string` function and ownership is returned
    println!("{}", s2); // `s2` owns the modified string and can use it
}

fn modify_string(mut s: String) -> String { // `s` takes ownership of the passed string and returns ownership
    s.push_str(", world");
    s // `s` is returned, transferring ownership back to the calling function
}
