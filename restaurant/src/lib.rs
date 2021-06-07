// tells Rust to load contents of the module from another file with the same name as the module
mod front_of_house;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    // default for enum variants to be public
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
}

// "use" with absolute path
// use crate::front_of_house::hosting;
// "use" with relative path
// use self::front_of_house::hosting;
// "pub use" allows external code to take advantage of the front_of_house module
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path, preferred way
    front_of_house::hosting::add_to_waitlist();

    // using "use" keyword
    hosting::add_to_waitlist();

    // ordering breakfast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // changing our mind about which bread we'd like
    meal.toast = String::from("Sourdough");
    println!("I'd like {} toast please", meal.toast);

    // ordering appetizers
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    println!("Order 1 and 2 are {:?} and {:?} respectively", order1, order2);
}

// NOTE: when specifying functions, it's more idiomatic to print the parent module as well
// DON'T DO THIS:
// use crate::front_of_house::hosting::add_to_waitlist;
// add_to_waitlist();
// DO THIS:
// use crate::front_of_house::hosting;
// hosting::add_to_waitlist();
// NOTE: do the inverse when bringing in structs, enums, and other items
// use std::collections::HashMap;
// let mut map = HashMap::new();
// NOTE: if there is a name conflict between two modules, then split them up. otherwise, Rust
// wouldn't know which "Result" to use
// use std::fmt;
// use std::io;
// fn fun1() -> fmt::Result {}
// fn fun2() -> io::Result {}
// NOTE: in the above "Result" example, we can alias using "as"
// use std::fmt::Result;
// use std::io::Result as IoResult;

// NOTE: if using multiple items defined in the same crate or module
// use std::cmp::Ordering;
// use std::io;
// vvvvvvvvvvvvvvvvvvvvvvvvvvvvv
// use std::{cmp::Ordering, io};
// NOTE: when using two items with the same subpath
// use std::io;
// use std::io::Write;
// vvvvvvvvvvvvvvvvvvvvvvvvvvv
// use std::io::{self, Write};
// NOTE: to bring all public items defined in a path, use the glob operator
// use std::collections::*;
