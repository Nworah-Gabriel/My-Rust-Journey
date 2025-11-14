mod print;
mod variables;
mod operators;
mod boolean;
mod conditional_statements;
mod match_test;
mod loops;
mod while_loops;
mod for_loops;
mod functions;
mod scope;
mod strings;
mod ownership;
mod borrowing;

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

    // testing conditional statements
    conditional_statements::check_age();
    conditional_statements::check_if_else();
    conditional_statements::if_as_an_expression();

    // testing match
    match_test::learn_match();
    match_test::match_mail_provider();
    match_test::multiple_match();
    match_test::match_with_return_value();

    // testing loops
    loops::simple_loop();
    loops::loop_with_return_value();

    // testing while loops
    while_loops::simple_while_loop();
    while_loops::while_loops_with_break();
    while_loops::while_loops_with_continue();

    // testing for loops
    for_loops::simple_for_loops();
    for_loops::simple_for_loops_including_last_number();
    for_loops::for_loops_with_break_and_continue();

    // testing functions
    functions::greet("SAGGIO");
    functions::test_add();
    functions::test_add_without_return_keyword();

    // testing scope
    scope::variable_shadowing();
    scope::same_variable_different_scope();

    // testing strings (`&str`, `String`)
    strings::first_string_literal_test();
    strings::second_string_literal_test();
    strings::modifying_string_literal_with_push_str();
    strings::modifying_string_literal_with_push();
    strings::test_concatinate_string_literals();
    strings::test_concatinate_string_literals_with_plus_sign();
    strings::test_string_length();

    // testing ownership
    ownership::test_simple_ownership_with_string();
    ownership::test_simple_ownership_with_number();
    ownership::test_clone();

    // testing borrowing
    borrowing::test_simple_borrowing();
    borrowing::updating_a_reference();
}
