use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn count_words_in_file(input_filename: &str, output_filename: &str) -> io::Result<()> {
    // Open the input file
    let input_file = File::open(input_filename)?;
    let input_reader = BufReader::new(input_file);

    // Count the number of words in the file
    let mut word_count = 0;
    for line in input_reader.lines() {
        let line = line?;
        word_count += line.split_whitespace().count();
    }

    // Write the number of words to the output file
    let mut output_file = File::create(output_filename)?;
    write!(output_file, "{}", word_count)?;

    Ok(())
}

fn main() {
    if let Err(error) = count_words_in_file("input.txt", "output.txt") {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}
