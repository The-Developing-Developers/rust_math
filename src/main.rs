//! Entry point for a CLI application that uses the `rust_math_lib` library.

use rust_math_lib::integrals;

fn main() {
    let result = integrals::temp_test_function();
    println!("The result is: {}", result);
}
