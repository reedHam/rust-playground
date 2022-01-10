// Bring the standard io module into scope
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number from 1 to 10!");
    
    let secret_number = Rng::rand::thread_rng().gen_range(1..=100);

    // This flag will remove the line below when compiled for release mode 
    // this can probably be better accomplished with a logging crate
    #[cfg(debug_assertions)]
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // Create a new instance of the String type
        let mut guess = String::new();

        // Call stdin from the io module
        io::stdin()
            // &mut specifies that guess is a mutable reference 
            .read_line(&mut guess)
            // Since read_line returns a Result with an Err variant it must be handled with the expect
            .expect("Failed to read line.");

        // Due to the new line captured by read line guess is a string 
        // To convert guess into a number we trim and parse it while specifying the new data type u32
        // By using match we can add code branches for the returned Result type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Input.");
                continue
            },
        };

        // String templates are possible with println 
        // Placeholders are replaced in left to right order with more than one variable
        println!("You guessed: {}", guess);

        // Match compares a value to keys with code branches
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => { 
                println!("You got it!");
                break;
            },
            Ordering::Greater => println!("Too big!"),
        }
    }
}
