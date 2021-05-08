## First cargo project
[Source](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)  

Cargo is a dependancy and build tool for rust.

# Cargo project

* Use the command `cargo new <project-name>` to create a new directory initialized by cargo.
* Then build using the `cargo build` command in the directory created with the command above.
    When using cargo build a `cargo.lock` file is created keeping track of the exact dependency's required to build.
* `cargo build` will create a debug executable with the projects name in the `target/debug/` directory.
* `cargo run` will build and run the debug exe in one command.
* `cargo check` will statically analyse your code to check for compilation errors.