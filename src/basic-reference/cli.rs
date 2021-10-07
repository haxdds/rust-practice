use std::env;

pub fn run() {

    let args: Vec<String> = env::args().collect();

    let path = args[0].clone();
    let command = args[1].clone();

    println!("{:?}", args);

}