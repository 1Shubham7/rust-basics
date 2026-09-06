struct Person {
    name: String,
    age: i32,
    height: u32,
}

fn main() {
    let shubham = Person {
        name: "shubham".to_string(),
        age: 22,
        height: 180,
    };

    println!("{} has age of {} and height of {}", shubham.name, shubham.age, shubham.height );
}