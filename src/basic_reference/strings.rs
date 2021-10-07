// Primitive str = Immutable fixed length string somewhere in memory
// String = Growable, heap-allocated data structure - use when you need to
// modify or own string data

pub fn run(){

    // primitive type str
    let hello_primitive = "hello";

    // String 
    let mut hello = String::from("Hello");

    // push char
    hello.push('!');

    // push str
    hello.push_str(" Whats   up?");

    //capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // is empty
    println!("Is Empty: {}", hello.is_empty());

    // contains
    println!("Contains 'World' : {}", hello.contains("World"));

    // replace - not in place
    println!("Replace Hello : {}", hello.replace("Hello", ""));

    // split by white space
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);

    s.push('A');
    s.push('B');

    println!("s: {}", s);

    // assertion testing

    assert_eq!("ABC", s);

    // get length
    println!("hello_primitive: {} -- {}: {}", hello_primitive.len(), hello, hello.len());


    
}