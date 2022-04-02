use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// # File I/O
/// Read a text file and display it on the console
fn main() {
    
    // Create a new instance of the File type
    let mut file = match File::open("hello.txt") {
        // The Err variant is used when the file could not be opened
        Err(error) => panic!("Could not open file: {}", error),
        // The Ok variant is used when the file is successfully opened
        Ok(file) => file,
    };

    // Create a new instance of the String type
    let mut contents = String::new();

    // Read the file into the String instance
    match file.read_to_string(&mut contents) {
        // The Err variant is used when the file could not be read
        Err(error) => panic!("Could not read file: {}", error),
        // The Ok variant is used when the file is successfully read
        Ok(_) => {
            // Print the contents of the file
            println!("{} are belong to us.", contents);
        }
    }
}
