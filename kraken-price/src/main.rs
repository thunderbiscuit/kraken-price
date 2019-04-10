use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let coin = &args[1];

    println!("the coin requested is {}", coin);
}
