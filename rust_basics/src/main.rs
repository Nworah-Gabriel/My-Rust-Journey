mod print;
mod variables;
mod operators;
mod boolean;

fn main() {
    // using my first public function to test `println!` macro
    print::use_println();
    // using my second public function to test `print!` macro
    print::use_print();
    // Testing variable storage through functions
    variables::user_detail();
    // printing values of different data types stored in implicitly declared local variables
    variables::implicitly_declared_variables();
    // printing values of different data types stored in explicitly declared local variables
    variables::explicitly_declared_variables();
    // testing operators in rust
    operators::assignment_operators();
    operators::arithmetic_operators();
    operators::comparison_operators();
    operators::logical_operators();
    operators::bitwise_operators();
    // testing boolean variable values
    boolean::boolean_implicit_test();
    boolean::boolean_explicit_test();
    boolean::boolean_from_comparison();
}
