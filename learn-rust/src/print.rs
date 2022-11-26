pub fn run() {
    // print to console
    println!("Hello from print.rs file");

    // basic formatting
    println!("string interpolation {} from {}", 1, "mohammad");

    // positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "brad", "mass", "code");

    // named arguments
    println!("{name} likes to play {activity}", name = "mohammad", activity = "vollyball");

    // placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}
