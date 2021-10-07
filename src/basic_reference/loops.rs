// Loops = used to iterate until a condition is met

pub fn run() {
    let mut count = 0;

    // infintie loop
    loop {
        count += 1;
        println!("number: {}", count);

        // need break condition
        if count == 20 {
            break;
        }
    }
    
    count = 0;
    // while loop
    while count <= 8 {
        if count % 2 == 0 {
            println!("Wizz");
        }else {
            println!("Cuzz");
        }
        count += 1;
    }

    for x in 0..16 {
        if x % 7 == 0 {
            println!("FizzBuzz");
        }else if x % 5 == 0 {
            println!("Fizz");
        }else {
            println!("Buzz");
        }
    }
}