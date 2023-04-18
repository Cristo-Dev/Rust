fn main() {
    let s = String::from("hello");
    print_string(s); // `s` is moved to `print_string` function
}

fn print_string(s: String) { // `s` takes ownership of the passed string
    println!("{}", s);
}
