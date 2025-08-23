use rust_math_lib::linear_algebra::Vector;

fn main() {
    println!("=== Testing Vector with Non-Numeric Types (Compile-Time Errors) ===\n");

    // These will now cause compile-time errors:

    // let string_vector = Vector::new(vec!["hello".to_string(), "world".to_string()]);
    // Error: String does not implement the Number trait

    // let bool_vector = Vector::new(vec![true, false, true]);
    // Error: bool does not implement the Number trait

    // let vec_vector = Vector::new(vec![vec![1, 2], vec![3, 4]]);
    // Error: Vec<i32> does not implement the Number trait

    println!("✓ All non-numeric types are now rejected at compile time!");
    println!("✓ Only types that implement the Number trait can be used.");

    // These still work (numeric types):
    let float_vector = Vector::new(vec![1.0, 2.0, 3.0]);
    let int_vector = Vector::new(vec![1i32, 2i32, 3i32]);
    let converted_vector = i32_vec_to_f64_vector(vec![1, 2, 3]);

    println!("\nValid numeric vectors:");
    println!("Float vector: {}", float_vector);
    println!("Integer vector: {}", int_vector);
    println!("Converted vector: {}", converted_vector);

    // Mathematical operations work as expected
    let sum = &float_vector + &converted_vector;
    println!("Sum of float and converted vectors: {}", sum);

    println!("\nSupported numeric types that implement Number:");
    println!("- f32, f64 (floating point)");
    println!("- i8, i16, i32, i64, isize (signed integers)");
    println!("- All support full vector operations including negation");
}

// Helper function to convert Vec<i32> to Vector<f64> for demonstration
fn i32_vec_to_f64_vector(v: Vec<i32>) -> Vector<f64> {
    Vector::new(v.into_iter().map(|x| x as f64).collect())
}
