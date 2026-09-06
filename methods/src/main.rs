struct Person {
    name: String,
    _age: u32,
    _height: u32,
}

impl Person {
    fn get_name(&self) {
        println!("the name of the person is {}", self.name);
    }
}

fn main() { 
    let shubham = Person {
        name: "shubham".to_string(),
        _age: 22,
        _height: 180,
    };

    shubham.get_name();
}