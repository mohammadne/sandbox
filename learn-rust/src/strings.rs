pub fn run() {
    let hello = "Hello" // primitive fixed string
    let world = String::from("World"); // growable string
    world.push('!')
    world.push_str("!!")

    printls!("length; {}", hello.len());
    printls!("capacity; {}", world.capacity());
    printls!("empty?"و world.is_empty())
    printls!("contains rld?"و world.contains("rld"))
    printls!("replace"و world.replace("World", "There"))

    for word in String::from("sample sentence").split_whitespace() {
        println!("{}", word);
    }

    // create string eith capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    asset_eq!(2, s.len())
}