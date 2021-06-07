#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

fn main() {
    vectors_example();
    string_example();
    string_concatenation_example();
    hash_map_example();
    accessing_hash_map_example();
    updating_hash_map_example();
}

fn vectors_example() {
    let mut v1 = vec![1, 2, 3];
    v1.push(4);
    v1.push(5);
    v1.push(6);

    println!("The vector is {:?}", v1);

    let v2 = vec![1, 2, 3, 4, 5, 6];
    for i in &v2 {
        println!("{}", i);
    }
    let third: &i32 = &v2[2];
    println!("The third element is {}", third);

    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element")
    }

    let mut v3 = vec![1, 2, 3, 4];
    for i in &mut v3 {
        // using dereference operator "*" in order to get the value in i
        *i += 50;
    }
    println!("The new vector is {:?}", v3);

    // if you want a vector containing multiple types, create an enum first
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("hello")),
        SpreadsheetCell::Float(10.12)
    ];

    println!("The row is {:?}", row);
}

fn string_example() {
    let data = "Initial contents";
    // "to_string" turns a string literal into a String
    let _s = data.to_string();
    // equivalent to the above
    let _s = String::from("Initial contents");

    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);

    println!("push_str did not take ownership of s2: {}", s2);

    println!("The contents of the string: {}", s);

    let mut lo = String::from("lo");
    // push_str works with characters denoted by single quotes
    lo.push('l');

    println!("Are you laughing {}", lo);
}

fn string_concatenation_example() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // using "+" uses the standard "add" method, which looks like the following:
    // fn add(self, s: &str) -> String {}
    // Rust uses a deref coercion, which turns &String into &str
    let s3 = s1 + &s2; // NOTE: s1 has been moved here and can no longer be used

    println!("The string is {}", s3);

    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");

    // using "format!" doesn't take ownership from any of its parameters
    let s = format!("{}-{}-{}", s4, s5, s6);

    println!("s4 is {}", s4);
    println!("s5 is {}", s5);
    println!("s6 is {}", s6);
    println!("The formatted string is {}", s);
}

use std::collections::HashMap;

fn hash_map_example() {
    let mut scores = HashMap::new();

    // for hash maps, all keys must have the same time; all values must have the same type
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("The scores hash map: {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // "HashMap<_, _>" is needed because it's possible to "collect" into many different data
    // structures. using underscores for the key and value types allows Rust to infer the types
    // that the hash map contains based on the types of the data in the vectors
    let scores2: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("The scores2 hash map: {:?}", scores2);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Red");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // NOTE: owned values, ie. String, are moved into the hash map

    println!("The map is {:?}", map);
    // these won't work because hash map has ownership of "field_name" and "field_value"
    // println!("Doesn't work {} {}", field_name, field_value);
    // NOTE: if we instead passed references of "field_name" and "field_value", then the values
    // won't be moved into the hash map
}

fn accessing_hash_map_example() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("The score is {:?}", &score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn updating_hash_map_example() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 40); // this will replace the value 10

    println!("{:?}", &scores);

    // only inserting if the key has no value
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(100);

    // should print { "Blue": 40, "Yellow": 50 }
    println!("{:?}", &scores);

    // updating a value based on the old value
    // NOTE: go back to this to understand it
    let text = "hello world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

// given a list of integers, use a vector and return the mean, median, and mode
// convert strings to pig latin
// add employee names to a department in a company
