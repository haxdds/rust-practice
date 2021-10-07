// Arrays - fixed length list where elements are same data types

use std::mem;

pub fn run(){

    let mut numbers: [i32;5] = [1, 2, 3, 4, 5];

    // reassign value
    numbers[2] = 27;

    println!("{:?}", numbers);

    // get single value
    println!("single value: {}", numbers[0]);

    // get array length
    println!("array length: {}", numbers.len());
    
    // arrays are stack-allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..2];

    println!("slice: {:?}", slice);
}