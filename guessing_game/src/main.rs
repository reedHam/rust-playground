// Bring the standard io module into scope
use std::io;
use rand::Rng;

fn main() {
    println!("Guess your number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

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
