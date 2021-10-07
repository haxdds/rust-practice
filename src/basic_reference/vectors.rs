// Vectors are resizeable arrays

use std::mem;

pub fn run(){

    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // reassign value
    numbers[2] = 27;

    // add onto vector
    numbers.push(22);
    numbers.push(7);


    numbers.pop();

    println!("{:?}", numbers);

    // get single value
    println!("single value: {}", numbers[0]);

    // get vector length
    println!("vector length: {}", numbers.len());
    
    // vectors are stack-allocated
    println!("vector occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..2];

    println!("slice: {:?}", slice);

    // loop through vector values
    for x in numbers.iter() {
        println!("number: x:{} - *x:{} - &x:{}", x, *x, &x);
    }

    // loop and mutate values
    for x in numbers.iter_mut() {
        *x = *x * 2;
    }

    println!("multipled: {:?}", numbers);
}