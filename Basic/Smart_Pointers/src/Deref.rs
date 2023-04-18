use std::ops::Deref;

fn main() {
    let my_boxed_string = Box::new(String::from("Hello, world!"));
    let my_string_ref: &String = my_boxed_string.deref();
    println!("{}", my_string_ref);
}
