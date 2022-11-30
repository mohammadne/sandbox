pub fn run () {
    let args: Vec<String> = std::args().collect();
    let command = args[0].clone();

    println!("Args: {:?}, command: {}", args, command);
}