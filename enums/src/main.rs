enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    let light_one = TrafficLight::Red;
    match light_one {
        TrafficLight::Red => println!("RED"),
        TrafficLight::Yellow => println!("ZZZ"),
        TrafficLight::Green => println!("YYY"),
    }
}
