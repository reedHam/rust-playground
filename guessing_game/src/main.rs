// Bring the standard io module into scope
use std::io;

fn main() {
    println!("Guess your number!");
    
    println!("Please input your guess.");

    // Create a new instance of the String type
    let mut guess = String::new();

    // Call stdin from the io module
    io::stdin()
        // &mut specifies that guess is a mutable reference 
        .read_line(&mut guess)
        // Since read_line returns a Result with an Err variant it must be handled with the expect
        .expect("Failed to read line.");

    // String templates are possible with println 
    // Placeholders are replaced in left to right order with more than one variable
    println!("You guessed: {}", guess);
}
