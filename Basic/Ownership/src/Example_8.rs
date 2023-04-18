fn main() {
    let mut s = String::from("hello");
    change_string(&mut s); // `change_string` takes a mutable reference to `s`
    println!("{}", s); // `s` is modified and now prints "hello, world"
}

fn change_string(s: &mut String) { // `s` takes a mutable reference to the passed string
    s.push_str(", world");
}
