enum Number {
    One,
    Two,
    Three,
}

fn main() {
    let current_number = Number::Two;

    match current_number {
        Number::One => println!("The number is one"),
        Number::Two => println!("The number is two"),
        Number::Three => println!("The number is three"),
    }
}
