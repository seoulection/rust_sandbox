// standard library to provide ability to accept user input and print the result as output
use std::io;
use std::cmp::Ordering;
// from the "rand" crate
use rand::Rng;

fn main() {
    println!("Guess the number");

    // immutable variable to hold a randomly generated value between 1-100
    // 1..101 is upper-bound exclusive. 1..=100 is upper-bound inclusive
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
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

        // "trim" eliminates any whitespace in the string
        // "parse" parses a string into some kind of number. Can parse a variety of number types,
        // therefore we need to tell Rust the exact number type
        // ex) let guess: u32 tells it to parse to an unsigned, 32-bit integer
        // switching from "expect" to "match" is how you generally move from crashing on an error
        // to handling an error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("The guess must be a number");
                continue;
            }
        };

        // similar to Python's formatted string literal "f"
        println!("You guessed: {}", guess);

        // compares the guess to secret number, then executes the 'arm' that is truthy
        // NOTE: the comparison between "guess" and "secret_number" means that Rust will infer that
        // "secret_number" is a u32, resulting in a comparison between two values of the same type
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
