use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    println!("{:?}", numbers);
    println!("single: {}", numbers[0]);
    println!("vector occupies {} bytes", mem::size_of_val(&numbers));
    
    numbers.push(5);
    println!("{:?}", numbers);
    numbers.pop();
    
    for x in numbers.iter() {
        println!("number: {}", x);
    }
    
    // loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("{:?}", numbers);

}