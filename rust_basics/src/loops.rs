pub fn simple_loop() {
    let mut count = 1;

    print!("\n\n");

    loop {
        println!("Hello World!");

        if count == 3 {
            break;
        }

        count += 1;
    }
}

pub fn loop_with_return_value() {
    let mut count = 1;

    print!("\n\n");

    let result = loop {
        println!("Hello!");

        if count == 10 {
            break count;
        }

        count += 1;
    };

    println!("The loop stopped at: {}", result);
}
