enum Movement {Up, Down, Right, Left}

fn move_avatar(m: Movement) {
    match m {
        Movement::Up => println!("moving up"),
        Movement::Down => println!("moving down"),
        Movement::Right => println!("moving right"),
        Movement::Left => println!("moving left")
    }
}

pub fn run() {
    let avatar1 = Movement::Left;
    move_avatar(avatar1);
}