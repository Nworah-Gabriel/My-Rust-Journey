pub fn test_simple_ownership_with_string() {
    let a = String::from("Hello");
    let b = a;

    println!("\n\nTesting ownership with string literals\n");
    println!("{}", b);
}

pub fn test_simple_ownership_with_number() {
    println!("\n\nTesting ownership with numbers\n");

    let a = 5;
    let b = a;
    println!("a = {}", a);
    println!("b = {}", b);
}

pub fn test_clone() {
    println!("\n\nTesting string clone from owner\n");

    let a = String::from("Hello");
    let b = a.clone();

    println!("a = {}", a);
    println!("b = {}", b);
}
