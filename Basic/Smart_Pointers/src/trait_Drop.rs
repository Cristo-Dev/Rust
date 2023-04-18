use std::fs::File;
use std::io::prelude::*;

struct MyFile {
    file: File,
}

impl MyFile {
    fn new(filename: &str) -> Self {
        let file = File::create(filename).unwrap();
        MyFile { file }
    }

    fn write(&mut self, data: &[u8]) {
        self.file.write_all(data).unwrap();
    }
}

impl Drop for MyFile {
    fn drop(&mut self) {
        self.file.flush().unwrap();
    }
}

fn main() {
    let mut my_file = MyFile::new("test.txt");
    my_file.write(b"Hello, world!").unwrap();
}
