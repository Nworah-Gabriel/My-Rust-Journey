pub fn check_age() {
    const AGE: i32 = 25;

    if AGE >= 18 {
        println!("You're qualified to vote");
    }
}

pub fn check_if_else() {
    let score = 93;

    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else if score >= 70 {
        println!("Grade: C");
    } else {
        println!("Grade: F");
    }
}

pub fn if_as_an_expression() {
    let time = 20;
    let greeting = if time < 18 { "Good day." } else { "Good evening." };
    println!("{}", greeting);
}
