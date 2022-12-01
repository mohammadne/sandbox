pub fn run() {
    let mut count = 0;

    // infinite loop
    loop {
        count += 1;

        println!("count is {}", count);

        if count == 20 {
            break;
        }
    }

    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("fizz");
        }

        count + 11;
    }

    for x in 0..10 {
        println!("inside for loop")
    }
}
