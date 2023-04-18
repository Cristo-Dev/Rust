use std::cell::RefCell;

struct MyStruct {
    data: RefCell<i32>,
}

impl MyStruct {
    fn increment_data(&self) {
        *self.data.borrow_mut() += 1;
    }
}

fn main() {
    let my_struct = MyStruct { data: RefCell::new(42) };
    println!("Data before: {}", *my_struct.data.borrow());
    my_struct.increment_data();
    println!("Data after: {}", *my_struct.data.borrow());
}
