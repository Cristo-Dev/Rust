enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let current_direction = Direction::Left;

    match current_direction {
        Direction::Up => println!("Moving up"),
        Direction::Down => println!("Moving down"),
        Direction::Left => println!("Moving left"),
        Direction::Right => println!("Moving right"),
    }
}
