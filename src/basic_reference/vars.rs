 // variables hold primitive data or references to data
 // variables are immutable by default
 // Rust is a block-scoped language (variable set in function pertains to that scope)


pub fn run() {
    let name = "Brad";

    let mut age = 37;

    println!("My name is {} and I am {} years old", name, age);

    age = 38;

    println!("My name is {} and I am {} years old", name, age);

    const ID: i32 = 001;
    
    println!("ID: {}", ID);

    let ( my_name, my_age ) = ("Brad", 37);

    println!("{} is {}", my_name, my_age);
}
