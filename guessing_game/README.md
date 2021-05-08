# Cargo CLI guessing game.
[Source](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)  

This program creates a prompt and takes an integer.  
Then compares the entered integer to a random integer.  
If the two number match you win.

## Attained Knowledge
* Rust variables are immutable by default mutability is enabled with the `mut` reserve word while initializing. 
```
let foo = 5; // immutable
let mut bar = 5; // mutable
```
* Type constructors can be called using the `::` scope operator like so `let mut word = String::new`  
* This works for module static methods and is showcased by `io:stdin()` where we are calling the stdin method thats part of the io module.
* References in rust are designated using the `&` similar to C++. `.read_line(&mut guess)` is passing a mutable reference to guess.
* Rust provides a number of `Result` types and some modules have their own types e.g. `io::Result`.
* `Result` types are enums containing Ok an Err variants that can be used for error handling.
* If method returns `Result` that can be an Err variant then rust will throw and error for not handling that potential error.
