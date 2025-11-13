pub fn arithmetic_operators() {
    let add = 5 + 3;
    let sub = 10 - 4;
    let mul = 6 * 2;
    let div = 12 / 3;
    let rem = 10 % 3;

    println!("Arithmetic Operators");
    println!("Add: {}", add);
    println!("Sub: {}", sub);
    println!("Mul: {}", mul);
    println!("Div: {}", div);
    println!("Rem: {}\n\n\n", rem);
}

pub fn assignment_operators() {
    let mut x = 10;

    println!("Assignment Operators");
    println!("Start: {}", x);

    x += 5;
    println!("After += 2: {}", x);

    x -= 2;
    println!("After -= 1: {}", x);

    x *= 2;
    println!("After *= 12: {}", x);

    x /= 3;
    println!("After /= 4: {}", x);

    x %= 4;
    println!("After %= 3: {}\n", x);
}

pub fn comparison_operators() {
    let a = 5;
    let b = 10;

    println!("Comparism Operators");
    println!("5 == 10: {}", a == b);
    println!("5 != 10: {}", a != b);
    println!("5 < 10: {}", a < b);
    println!("5 >= 10: {}", a >= b);
}

pub fn logical_operators() {
    let logged_in = true;
    let is_admin = false;

    println!("Logical Operators");
    println!("Is regular user: {}", logged_in && !is_admin);
    println!("Has any access: {}", logged_in || is_admin);
    println!("Not logged in: {}\n", !logged_in);
}

pub fn bitwise_operators() {
    let a = 5;
    let b = 10;

    println!("Bitwise Operators");
    println!("5 & 10: {}", a & b);
    println!("5 >> 10: {}", a >> b);
    println!("5 << 10: {}", a << b);
    println!("5 | 10: {}\n", a | b);
}
