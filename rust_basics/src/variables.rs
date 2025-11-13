pub fn user_detail(){
    let user = "Anonymous";
    let age = 22;
    println!("\nName: {} Age: {}", user, age);
}


pub fn implicitly_declared_variables(){
    let my_num = 5;      
    let my_double = 5.99;
    let my_letter = 'D'; 
    let mut my_bool = true;  
    let my_text = "Hello";
    // printing the previous value
    println!("my_bool {}", my_bool);
    // re-assigning a mutable variable
    my_bool = false;
    println!("\n\nImplicitly declared variables\n");
    println!("my_num: {}\nmy_double: {}\nmy_letter: {}\nmy_bool: {}\nmy_text: {}\n\n",my_num, my_double, my_letter, my_bool, my_text);
}

pub fn explicitly_declared_variables(){
    let my_num: i32 = 5;      
    let my_double: f64 = 5.99;
    let my_letter: char = 'D';
    let my_bool: bool = true; 
    let my_text: &str = "Hello";
    println!("Explicitly declared variables\n");
    println!("my_num: {}\nmy_double: {}\nmy_letter: {}\nmy_bool: {}\nmy_text: {}\n\n",my_num, my_double, my_letter, my_bool, my_text); 
}