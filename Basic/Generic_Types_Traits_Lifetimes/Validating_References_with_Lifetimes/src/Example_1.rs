struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
    fn new(x: &'a i32) -> Foo<'a> {
        Foo { x: x }
    }

    fn print(&self) {
        println!("The value of x is: {}", self.x);
    }
}

fn main() {
    let x = 5;
    let foo = Foo::new(&x);
    foo.print();
}
