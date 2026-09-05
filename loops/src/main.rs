fn main() {
    for i in 0..=3 {
        println!("{i}");
    }

    for j in 10..20 {
        println!("{j}");
    }

    let mylist = ["shubham", "mahar"];
    for x in mylist {
        println!("{x}");
    }

    let mut num = 10;
    while num > 5 {
        println!("{num}");
        num = num - 1;
    }
}
