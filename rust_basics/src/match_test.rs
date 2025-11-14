pub fn learn_match() {
    let day = 6;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day."),
    }
}

pub fn match_mail_provider() {
    let email: &str = "gabrielnworah6@gmail.com";
    let provider: &str = "gmail";

    match provider {
        "gmail" => println!("\n\nType: SMTP\nPort: 587\nEmail:{}\nSending Email...", email),
        "yahoo" => println!("\n\nType: IMAP\nPort: 600\nEmail:{}\nSending Email...", email),
        "privatemail" => println!("\n\nType: POP3\nPort: 263\nEmail:{}\nSending Email...", email),
        _ => println!("Invalid mail provider"),
    }
}

pub fn multiple_match() {
    let day = 4;

    match day {
        1 | 2 | 3 | 4 | 5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day"),
    }
}

pub fn match_with_return_value() {
    let day = 4;

    let result = match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day.",
    };

    println!("{}", result);
}
