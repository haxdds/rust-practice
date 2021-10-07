// Functions - used to store blocks of code for reuse

pub fn run() {
    greeting("Hello", "My Dude");

    //bind function values to variables

    let sum: i32 = add(5, 5);

    println!("sum: {}", sum);

    // closure
    let n3: i32 = 10; 
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;

    println!("closure sum: {}", add_nums(3, 12));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    return n1 + n2;
}