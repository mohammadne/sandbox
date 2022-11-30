pub fn run() {
    let age = 22;
    let check_id: bool = false;

    if age >= 22 && check_id{
        println!("do you like to drink?");
    } else if age < 22 && check_id {
        println!("you have to leave!");
    } else {
        println!("show your ID");
    }
    
    let short_hand_if = if age >= 22 {true} else {false};
    println!("granted: {}", short_hand_if);
}
