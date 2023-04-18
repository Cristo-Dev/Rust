fn print_type<T>(x: T) {
    println!("The type of x is: {}", std::any::type_name::<T>());
}

fn main() {
    print_type("hello");
    print_type(42);
    print_type(3.14);
}
