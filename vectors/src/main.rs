fn main() {
    let mut num = vec![10,20,30];
    println!("Hello, world!");
    println!("first element: {}, second element: {}", num[0], num[1]);

    num.push(40);
    for i in num {
        println!("{}", i);
    }
}
