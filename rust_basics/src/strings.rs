pub fn first_string_literal_test() {
    println!("\n\nFirst string literal test `.to_string`\n");

    let name = "SAGGIO".to_string();

    println!("I am {}!", name);
}

pub fn second_string_literal_test() {
    println!("\n\nSecond string literal test `String::from(\"\")` \n");

    let name = String::from("SAGGIO");

    println!("I am {}!", name);
}

pub fn modifying_string_literal_with_push_str() {
    println!(
        "\n\nSecond string literal test `String::from(\"\")` with modification `push_str()`\n"
    );
    let mut greeting = String::from("Hello");
    greeting.push_str(" World");
    println!("{}", greeting);
}

pub fn modifying_string_literal_with_push() {
    println!("\n\nSecond string literal test `String::from(\"\")` with modification `push()`\n");
    let mut word = String::from("Hi");
    word.push('!');
    println!("{}", word);
}

pub fn test_concatinate_string_literals() {
    println!("\n\nTest string concatination `format!` macro\n");
    let s1 = String::from("Hello");
    let s2 = String::from("World!");
    let s3 = String::from("What a beautiful day!");
    let result = format!("{} {}, {}", s1, s2, s3);
    println!("{}", result);
}

pub fn test_concatinate_string_literals_with_plus_sign() {
    println!("\n\nTest string concatination with + \n");
    let s1 = String::from("Hello");
    let s2 = String::from("World!");
    let s3 = String::from("What a beautiful day!");
    let result = s1 + " " + &s2 + " " + &s3;
    println!("{}", result);
}

pub fn test_string_length() {
    println!("\n\nTesting string length\n");
    let name = String::from("SAGGIO");
    println!("Length: {}", name.len());
}
