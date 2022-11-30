pub fn run() {
    greeting("mohammad", "nasr");
    
    println!("sum: {}", sum(5, 5));

    let closure_sum = |num1: i32, num2: i32| 10 + num1 + num2;
    println!("closure sum {}", closure_sum(5, 5));
}

fn greeting(name: &str, family: &str) {
    println!("hello {} {}", name, family);
}

fn sum(num1: i32, num2: i32) -> i32 {
    num1 + num2 // return without semicolons
}
