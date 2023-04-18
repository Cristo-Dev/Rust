struct Rectangle {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { x: 10, y: 20, width: 100, height: 200 };
    println!("Area of rectangle is: {}", rect.area());
}
