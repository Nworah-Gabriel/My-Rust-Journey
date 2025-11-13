mod print;
mod variables;
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
}
