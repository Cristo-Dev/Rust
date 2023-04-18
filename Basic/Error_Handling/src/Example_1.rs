use std::fs::File;

fn main() {
    let result = File::open("nonexistent.txt");
    match result {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(error) => println!("Error opening file: {:?}", error),
    }
}
