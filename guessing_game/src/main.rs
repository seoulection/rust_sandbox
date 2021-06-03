// standard library to provide ability to accept user input and print the result as output
use std::io;

fn main() {
    println!("Guess the number");
    println!("Please input your guess:");

    // "let" creates a variable. In Rust, variables are immutable by default
    // "mut" allows a variable to be mutable
    // String::new() is a function that returns a new instance of a String
    // altogether, the line below creates a mutable variable that is bound to a new, empty instance
    // of String
    let mut guess = String::new();

    // "stdin" returns an instance of Stdin, which is a type that represents a handle to the
    // standard input of your terminal
    // "read_line" accepts user input and appends the result into a string (without overwriting its
    // contents). It takes a string as an argument and returns an "io::Result"
    // & indicates the argument is a reference. Useful because you don't have to copy data into
    // memory multiple times
    // references are immutable by default, therefore "&mut" is required to make it mutable
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    // finally, "expect" is a method from a Result instance. Results are enums with Ok and Err
    // variants. Ok indicates the operation was successful and Err means the operation failed
    // If Err, "expect" will cause the program to crash and display the message passed in as the
    // argument. If Ok, "expect" will take the return value that Ok is holding and just return that
    // value

    // similar to Python's formatted string literal "f"
    println!("You guessed: {}", guess);
}
