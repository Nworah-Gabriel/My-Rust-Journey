pub fn simple_while_loop() {
    let mut count = 1;
    print!("\n\n");
    
    while count <= 5 {
        println!("Count: {}", count);
        count += 1;
    }
}

pub fn while_loops_with_break() {
    let mut num = 1;
    print!("\n\n");

    while num <= 10 {
        if num == 6 {
            break;
        }
        println!("Number: {}", num);
        num += 1;
    }
}

pub fn while_loops_with_continue() {
    let mut num = 1;
    print!("\n\n");

    while num <= 10 {
        if num == 6 {
            num += 1;
            continue;
        }

        println!("Number: {}", num);
        num += 1;
    }
}
