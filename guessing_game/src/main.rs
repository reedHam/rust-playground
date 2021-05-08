// Bring the standard io library into scope
use std::io;

fn main() {
    println!("Guess your number!");
    
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed: {}", guess);
}
