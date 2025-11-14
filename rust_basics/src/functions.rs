pub fn greet(name: &str) {
    println!("\n\nFunction with Parameter");
    println!("\nGood Day, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

pub fn test_add() {
    let sum = add(3, 4);
    println!("\n\nTesting Add\n");
    println!("Sum is: {}", sum);
}

fn add_without_return_keyword(a: i32, b: i32) -> i32 {
    a + b
}

pub fn test_add_without_return_keyword() {
     println!("\n\nFunction without `return` keyword");
    let sum = add_without_return_keyword(3, 4);
    println!("Sum is: {}", sum);
}