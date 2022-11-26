pub fn run() {
    // default is "i32"
    let x = 1;

    // default is "f64"
    let y = 2.5;

    // explicit type
    let z: i64 = 342532;

    // finf max size
    println!("Max i32 is {}", std::i32::MAX);

    // boolean
    let is_active = 10 > 5;
    
    // character
    let c1 = 'a'

    println!("{:?}", (x, y, z, is_active, c1));
}