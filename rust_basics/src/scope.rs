pub fn variable_shadowing() {
    println!("\n\nScope Shadowing\n");
    // before shadowing
    let x: i32 = 10;
    println!("Before Shadowing: x = {}", x);
    // after shadowing
    let x: i32 = 20;
    println!("After Shadowing: x = {}", x);
}

pub fn same_variable_different_scope() {

    println!("\n\nSame Variable, Different scope\n");
    let x = 5;

    {
        let x = 10;
        println!("Inside block: {}", x);
    }

    println!("Outside block: {}", x);
}
