pub fn test_simple_borrowing() {
    let a = String::from("Hello");
    let b = &a;

    println!("\n\n Testing simple borrowing");
    println!("a = {}", a);
    println!("b = {}", b);
}

pub fn updating_a_reference() {
    let mut name = String::from("John");
    let name_ref = &mut name;
    name_ref.push_str(" Doe");

    println!("\n\n Testing simple reference update");
    println!("{}", name_ref);
    println!("{}", name);
}
