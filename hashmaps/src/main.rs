use std::collections::HashMap;

fn main() {
    let mut ages = HashMap::new();

    ages.insert("Shubham", 25);
    ages.insert("John", 30);

    println!("{}", ages["Shubham"]);
}