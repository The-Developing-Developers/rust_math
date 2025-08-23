use rust_math_lib::linear_algebra::Vector;

fn main() {
    println!("=== Vector Examples ===\n");

    // Create vectors of different numeric types
    println!("1. Creating Vectors:");
    let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
    let v2 = Vector::vec3(4.0, 5.0, 6.0);
    let v_int = Vector::new(vec![1, 2, 3, 4]);

    println!("v1: {}", v1);
    println!("v2: {}", v2);
    println!("v_int: {}", v_int);
    println!();

    // Vector operations
    println!("2. Vector Operations:");
    let sum = &v1 + &v2;
    let diff = &v2 - &v1;
    println!("v1 + v2 = {}", sum);
    println!("v2 - v1 = {}", diff);
    println!();

    // Scalar operations
    println!("3. Scalar Operations:");
    let scaled = &v1 * 2.0;
    let divided = &v2 / 2.0;
    println!("v1 * 2 = {}", scaled);
    println!("v2 / 2 = {}", divided);
    println!();

    // Dot product and magnitude
    println!("4. Dot Product and Magnitude:");
    let dot = v1.dot(&v2);
    let mag1 = v1.magnitude();
    let mag2 = v2.magnitude();
    println!("v1 · v2 = {}", dot);
    println!("|v1| = {:.3}", mag1);
    println!("|v2| = {:.3}", mag2);
    println!();

    // Cross product (3D only)
    println!("5. Cross Product:");
    let unit_x = Vector::unit_x();
    let unit_y = Vector::unit_y();
    let cross = unit_x.cross(&unit_y);
    println!("unit_x × unit_y = {}", cross);
    println!();

    // Normalization
    println!("6. Normalization:");
    let v3 = Vector::vec2(3.0, 4.0);
    println!("v3 = {} (magnitude: {:.3})", v3, v3.magnitude());
    let normalized = v3.normalize();
    println!("normalized v3 = {} (magnitude: {:.3})", normalized, normalized.magnitude());
    println!();

    // Working with different dimensions
    println!("7. Different Dimensions:");
    let v2d = Vector::vec2(1.0, 2.0);
    let v4d = Vector::vec4(1.0, 2.0, 3.0, 4.0);
    let v5d = Vector::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);

    println!("2D vector: {} (dimension: {})", v2d, v2d.dimension());
    println!("4D vector: {} (dimension: {})", v4d, v4d.dimension());
    println!("5D vector: {} (dimension: {})", v5d, v5d.dimension());
    println!();

    // Zero and filled vectors
    println!("8. Special Constructors:");
    let zeros = Vector::<f64>::zeros(3);
    let filled = Vector::filled(4, 7.5);
    println!("zeros(3): {}", zeros);
    println!("filled(4, 7.5): {}", filled);
    println!();

    // Indexing
    println!("9. Indexing:");
    let mut v_mut = Vector::vec3(1.0, 2.0, 3.0);
    println!("Before: v_mut[1] = {}", v_mut[1]);
    v_mut[1] = 10.0;
    println!("After: v_mut[1] = {}, vector = {}", v_mut[1], v_mut);
}
