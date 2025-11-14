pub fn simple_for_loops() {
    print!("\n\nSimple for Loops\n\n");

    for i in 1..10 {
        println!("Value of i at current loop: {}", i);
    }
}

pub fn simple_for_loops_including_last_number() {
    print!("\n\nSimple for Loops including the last loop count\n\n");

    for i in 1..=10 {
        println!("Value of i at current loop: {}", i);
    }
}

pub fn for_loops_with_break_and_continue() {

    print!("\n\nSimple for Loops with break and continue\n\n");

    for i in 1..=10 {
        if i == 4 {
            continue;
        }
        if i == 10 {
            break;
        }
        println!("i is: {}", i);
    }
}
