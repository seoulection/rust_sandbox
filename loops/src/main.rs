fn main() {
    loop_example();
    while_example();
    for_example();
    for_range_reversed_example();
}

fn loop_example() {
    let mut counter = 0;

    // loops runs infinitely unless interrupted or a "break" is encountered
    let result = loop {
        counter += 1;

        if counter == 10 {
            // this breaks out of the loop and returns the value (counter * 2)
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn while_example() {
    let mut number = 3;

    // while loops run until the condition is false
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("TO THE MOON 🚀");
}

fn for_example() {
    let a = [1, 2, 3, 4, 5];

    // turns the array "a" into an iterable
    for element in a.iter() {
        println!("The value is {}", element);
    }
}

fn for_range_reversed_example() {
    // (1..4) specifies an upper-bound exclusive range
    // "rev()" reverses that range -> 3-1
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("TO THE MOON 🚀");
}
