fn main() {
    let mut s = String::from("hello");

    change(&mut s); // a mutable reference of `s` is passed to the function `change`

    println!("{}", s); // the string has been modified inside the function and is now "hello, world"
}

fn change(some_string: &mut String) { // `some_string` is a mutable reference to a string owned by another variable
    some_string.push_str(", world"); // `.push_str()` method is called on the mutable reference `some_string`
}
