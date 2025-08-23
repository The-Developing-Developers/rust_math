// This file demonstrates compile-time errors when trying to use non-numeric types
// Uncomment any of these lines to see the compile error

use rust_math_lib::linear_algebra::Vector;

#[allow(dead_code)]
fn demonstrate_compile_errors() {
    // These will cause compile-time errors:

    // let string_vector = Vector::new(vec!["hello".to_string(), "world".to_string()]);
    // // ^ ERROR: the trait bound `String: Number` is not satisfied

    // let bool_vector = Vector::new(vec![true, false, true]);
    // // ^ ERROR: the trait bound `bool: Number` is not satisfied

    // struct CustomType { value: i32 }
    // let custom_vector = Vector::new(vec![CustomType { value: 1 }]);
    // // ^ ERROR: the trait bound `CustomType: Number` is not satisfied

    // let option_vector = Vector::new(vec![Some(1), None, Some(3)]);
    // // ^ ERROR: the trait bound `Option<i32>: Number` is not satisfied
}

fn main() {
    println!("This file compiles successfully because the error-causing lines are commented out.");
    println!("Uncomment any of the lines in demonstrate_compile_errors() to see the compile-time protection in action.");

    // This works fine - using a supported numeric type
    let valid_vector = Vector::new(vec![1.0, 2.0, 3.0]);
    println!("Valid vector: {}", valid_vector);
}
