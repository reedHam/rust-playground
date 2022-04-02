use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn read_file(path: &str) -> Result<String, Error> {
    // Here the file open function returns a result enum.
    // The question mark operator will return the value of the Ok variant if the file is opened successfully.
    // If the file is not opened successfully, the Err variant will be returned.
    // This propagates the error to the calling function.
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .take_while(Result::is_ok)
        .map(Result::unwrap)
        .collect::<String>())
}

fn read_file_propagate_error(path: &str) -> Result<String, Error> {
    // Here we define a mutable error to store the result into.
    let mut err: Result<(), Error> = Ok(());
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    // using scan we can iterate over the lines of the file if an error occurs then we store the result into the error variable.
    let file_contents = reader
        .lines()
        .scan(&mut err, |err, line| match line {
            Ok(line) => Some(line),
            Err(e) => {
                **err = Err(e);
                None
            }
        })
        .collect();
    // Here we use the ? operator to propagate the error to the calling function.
    err?;
    // if no error occurred then we return the file contents.
    Ok(file_contents)
}

fn main() {
    let contents = read_file_propagate_error("resources/test.txt").unwrap();
    print!("{}", contents);
}
