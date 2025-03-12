///
/// TODO: this is the entry point for a CLI application that uses the `rust_math` library
///
use rust_math::integrals;

fn main() {
    let result = integrals::temp_test_function();
    println!("The result is: {}", result);
}
