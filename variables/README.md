# Cargo Variables
[Source](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)  

Some information about rust variables.

## Attained Knowledge
* RUST variables are immutable by default mutability is enabled with the `mut` reserve word while initializing. 
```
let foo = 5; // immutable
let mut bar = 5; // mutable
```
* In RUST variables ar named using snake case `my_variable`.
* Immutable variables can be "Redefined" or shadowed as rust calls it by using `let` again `let foo = 1` would shadow foo to 1.
* RUST also has constant variables that can be defined using `const` this means that the variables cannot be shadowed.
* For collections rust has three basic types Tuples, Arrays, and Vectors.
* Tuples cannot change and size and can contain any type. `let tup = ("test", 3, 2.1)`
* Arrays cannot change in size and must contain only one type. `let array = [1, 2, 3, 4, 5]`
    Arrays are bounds protected and accessing outside of the bounds will result in a panic error.
* Vectors NOT COVERED