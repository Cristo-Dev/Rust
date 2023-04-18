enum Color {
    Red,
    Blue,
    Green,
}

fn main() {
    let current_color = Color::Green;

    match current_color {
        Color::Red => println!("The color is red"),
        Color::Blue => println!("The color is blue"),
        Color::Green => println!("The color is green"),
    }
}
