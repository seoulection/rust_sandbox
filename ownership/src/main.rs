fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s' value moves into the function
                        // s is no longer valid starting here

    let x = 5; // x comes into scope

    makes_copy(x); // x is of type i32, which is a Copy type, so x can be used afterwards
                   // for example, x can be used from here onward until it is out of scope
} // x goes out of scope, then s. Since the value of s was moved, nothing special happens

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("Some string: {}", some_string);
} // some_string goes out of scope and "drop" is called. The backing memory is freed

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("Some integer: {}", some_integer);
} // some_integer goes out of scope, nothing special happens
