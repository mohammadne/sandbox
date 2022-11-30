// fixed list
// elements are the same data type

use std::mem;

pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);
    println!("single: {}", numbers[0]);
    println!("array occupies {} bytes", mem::size_of_val(&numbers));
 
    let mut re_assignables: [i32; 5] = [1, 2, 3, 4, 5];
    re_assignables[2] = 20;

    let slice: &[i32] = &numbers[0..3];
    println!("slice: {}", slice)
}