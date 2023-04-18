struct MyStruct {
    // ...
}

fn my_function(some_data: Box<MyStruct>) {
    // ...
}

fn main() {
    let data = Box::new(MyStruct { /* ... */ });
    my_function(data);
}
