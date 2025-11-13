pub fn boolean_explicit_test() {
    let is_programming_fun: bool = true;
    let is_fish_tasty: bool = false;
    let is_rust_fun: bool = true;

    println!("Is Programming Fun? {}", is_programming_fun);
    println!("Is Fish Tasty? {}", is_fish_tasty);
    println!("Is Rust Language fun? {}\n\n", is_rust_fun)
}

pub fn boolean_implicit_test() {
    let is_programming_fun = true;
    let is_fish_tasty = false;
    let is_rust_fun = true;

    println!("Is Programming Fun? {}", is_programming_fun);
    println!("Is Fish Tasty? {}", is_fish_tasty);
    println!("Is Rust Language fun? {}\n\n", is_rust_fun)
}

pub fn boolean_from_comparison() {
    let age = 25;
    let can_vote = age >= 18;

    println!("Can vote? {}", can_vote);
}
