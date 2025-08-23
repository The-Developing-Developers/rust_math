//! Linear algebra module providing vector and matrix operations.
//!
//! This module provides a generic `Vector<T>` type that supports:
//!
//! - **Numeric types only**: Only types implementing the `Number` trait can be used
//!   (f32, f64, i8, i16, i32, i64, isize). This prevents creation of non-mathematical vectors.
//! - **Arbitrary dimensions**: Supports 2D, 3D, 4D, and higher dimensional vectors
//! - **Non-zero dimensions**: Prevents creation of zero-dimensional vectors
//! - **Comprehensive operations**: Addition, subtraction, scalar multiplication/division,
//!   dot product, cross product (3D), normalization, and magnitude calculation
//! - **Memory efficient**: Uses `Vec<T>` internally for dynamic sizing
//! - **Safe indexing**: Implements `Index` and `IndexMut` for element access
//! - **Compile-time safety**: Non-numeric types are rejected at compile time
//!
//! # Design Principles
//!
//! 1. **Numeric types only**: The `Number` trait ensures that only mathematical types
//!    can be used to create vectors, preventing nonsensical operations like adding strings.
//!
//! 2. **Flexible dimensionality**: Unlike fixed-size vector types, this implementation
//!    allows vectors of any dimension to be created at runtime.
//!
//! 3. **Zero-copy operations**: Many operations are implemented for references (`&Vector<T>`)
//!    to avoid unnecessary allocations.
//!
//! 4. **Panic on invalid operations**: The API panics on mathematically invalid operations
//!    (e.g., dot product of different-dimensional vectors) to catch errors early.
//!
//! # Supported Types
//!
//! The following numeric types implement the `Number` trait and can be used with `Vector<T>`:
//!
//! - **Floating point**: `f32`, `f64`
//! - **Signed integers**: `i8`, `i16`, `i32`, `i64`, `isize`
//!
//! Note: Unsigned integers are not supported because they don't support negation,
//! which is required for complete vector operations.
//!
//! # Examples
//!
//! ```rust
//! use rust_math_lib::linear_algebra::Vector;
//!
//! // Create vectors (only numeric types allowed)
//! let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
//! let v2 = Vector::vec3(4.0, 5.0, 6.0);
//!
//! // Basic operations
//! let sum = &v1 + &v2;              // Vector addition
//! let scaled = &v1 * 2.0;           // Scalar multiplication
//! let dot = v1.dot(&v2);            // Dot product
//! let magnitude = v1.magnitude();    // Euclidean length
//!
//! // 3D specific operations
//! let cross = v1.cross(&v2);        // Cross product (3D only)
//! let normalized = v1.normalize();   // Unit vector
//!
//! // Integer vectors also work
//! let int_vector = Vector::new(vec![1i32, 2i32, 3i32]);
//!
//! // This would cause a compile error:
//! // let string_vector = Vector::new(vec!["hello".to_string()]); // ❌ Error!
//! ```
//!
//! // Different numeric types
//! let int_vector = Vector::new(vec![1, 2, 3, 4]);
//! let float_vector = Vector::new(vec![1.5, 2.7, 3.14]);
//! ```

use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub, Mul, Div, Index, IndexMut, Neg};

/// A trait that defines the requirements for numeric types that can be used in vectors.
/// This ensures that only numeric types can be used to create vectors.
pub trait Number:
    Clone + Copy + Default + PartialEq +
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>
{
    /// Convert this number to f64 for magnitude calculations.
    /// This may be lossy for large integers.
    fn to_f64(self) -> f64;

    /// Create a number from f64, used for normalization.
    /// This may panic or be lossy for integer types.
    fn from_f64(val: f64) -> Self;
}

// Implement Number for floating point types
impl Number for f32 {
    fn to_f64(self) -> f64 { self as f64 }
    fn from_f64(val: f64) -> Self { val as f32 }
}

impl Number for f64 {
    fn to_f64(self) -> f64 { self }
    fn from_f64(val: f64) -> Self { val }
}

// Implement Number for signed integer types
impl Number for i8 {
    fn to_f64(self) -> f64 { self as f64 }
    fn from_f64(val: f64) -> Self { val as i8 }
}

impl Number for i16 {
    fn to_f64(self) -> f64 { self as f64 }
    fn from_f64(val: f64) -> Self { val as i16 }
}

impl Number for i32 {
    fn to_f64(self) -> f64 { self as f64 }
    fn from_f64(val: f64) -> Self { val as i32 }
}

impl Number for i64 {
    fn to_f64(self) -> f64 { self as f64 }
    fn from_f64(val: f64) -> Self { val as i64 }
}

impl Number for isize {
    fn to_f64(self) -> f64 { self as f64 }
    fn from_f64(val: f64) -> Self { val as isize }
}

// Note: We don't implement Number for unsigned types or i128/u128
// because they don't support negation or may have conversion issues

/// A generic vector type that supports any numeric type and arbitrary dimensions.
///
/// # Examples
///
/// ```
/// use rust_math_lib::linear_algebra::Vector;
///
/// // Create a 3D vector of f64
/// let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
/// let v2 = Vector::new(vec![4.0, 5.0, 6.0]);
///
/// // Vector operations
/// let sum = &v1 + &v2;
/// let dot_product = v1.dot(&v2);
/// let magnitude = v1.magnitude();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Vector<T> {
    components: Vec<T>,
}

impl<T> Vector<T>
where
    T: Number,
{
    /// Creates a new vector from a Vec of components.
    ///
    /// # Panics
    ///
    /// Panics if the input vector is empty (zero dimensions).
    pub fn new(components: Vec<T>) -> Self {
        if components.is_empty() {
            panic!("Vector cannot have zero dimensions");
        }
        Self { components }
    }

    /// Returns the number of dimensions (components) in the vector.
    pub fn dimension(&self) -> usize {
        self.components.len()
    }

    /// Returns a slice of the vector's components.
    pub fn components(&self) -> &[T] {
        &self.components
    }

    /// Returns a mutable slice of the vector's components.
    pub fn components_mut(&mut self) -> &mut [T] {
        &mut self.components
    }

    /// Converts the vector into its underlying Vec of components.
    pub fn into_components(self) -> Vec<T> {
        self.components
    }

    /// Returns an iterator over the vector's components.
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.components.iter()
    }

    /// Returns a mutable iterator over the vector's components.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.components.iter_mut()
    }
}

impl<T> Vector<T>
where
    T: Number,
{
    /// Creates a new vector with `n` dimensions, all initialized to the default value.
    pub fn zeros(n: usize) -> Self {
        if n == 0 {
            panic!("Vector cannot have zero dimensions");
        }
        Self {
            components: vec![T::default(); n],
        }
    }
}

impl<T> Vector<T>
where
    T: Number,
{
    /// Creates a new vector with `n` dimensions, all initialized to `value`.
    pub fn filled(n: usize, value: T) -> Self {
        if n == 0 {
            panic!("Vector cannot have zero dimensions");
        }
        Self {
            components: vec![value; n],
        }
    }

    /// Computes the dot product with another vector.
    ///
    /// # Panics
    ///
    /// Panics if the vectors have different dimensions.
    pub fn dot(&self, other: &Vector<T>) -> T {
        if self.dimension() != other.dimension() {
            panic!("Vectors must have the same dimension for dot product");
        }

        self.components
            .iter()
            .zip(other.components.iter())
            .map(|(a, b)| *a * *b)
            .fold(T::default(), |acc, x| acc + x)
    }
}

impl<T> Vector<T>
where
    T: Number,
{
    /// Computes the magnitude (length) of the vector.
    pub fn magnitude(&self) -> f64 {
        let sum_of_squares = self.components
            .iter()
            .map(|&x| {
                let val = x.to_f64();
                val * val
            })
            .sum::<f64>();
        sum_of_squares.sqrt()
    }

    /// Computes the squared magnitude of the vector (avoids square root computation).
    pub fn magnitude_squared(&self) -> f64 {
        self.components
            .iter()
            .map(|&x| {
                let val = x.to_f64();
                val * val
            })
            .sum()
    }
}

impl<T> Vector<T>
where
    T: Number,
{
    /// Returns a unit vector in the same direction.
    ///
    /// # Panics
    ///
    /// Panics if the vector has zero magnitude.
    pub fn normalize(&self) -> Vector<T> {
        let mag = self.magnitude();
        if mag == 0.0 {
            panic!("Cannot normalize zero vector");
        }

        let components = self.components
            .iter()
            .map(|&x| {
                let val = x.to_f64();
                T::from_f64(val / mag)
            })
            .collect();

        Vector::new(components)
    }

    /// Normalizes the vector in place.
    ///
    /// # Panics
    ///
    /// Panics if the vector has zero magnitude.
    pub fn normalize_mut(&mut self) {
        let mag = self.magnitude();
        if mag == 0.0 {
            panic!("Cannot normalize zero vector");
        }

        for component in &mut self.components {
            let val = component.to_f64();
            *component = T::from_f64(val / mag);
        }
    }
}impl<T> Vector<T>
where
    T: Number,
{
    /// Computes the cross product with another 3D vector.
    ///
    /// # Panics
    ///
    /// Panics if either vector is not 3-dimensional.
    pub fn cross(&self, other: &Vector<T>) -> Vector<T> {
        if self.dimension() != 3 || other.dimension() != 3 {
            panic!("Cross product is only defined for 3D vectors");
        }

        let a = &self.components;
        let b = &other.components;

        Vector::new(vec![
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ])
    }
}

// Index trait implementation for accessing components
impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.components[index]
    }
}

impl<T> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.components[index]
    }
}

// Arithmetic operations
impl<T> Add for &Vector<T>
where
    T: Number,
{
    type Output = Vector<T>;

    fn add(self, other: &Vector<T>) -> Vector<T> {
        if self.dimension() != other.dimension() {
            panic!("Vectors must have the same dimension for addition");
        }

        let components = self.components
            .iter()
            .zip(other.components.iter())
            .map(|(a, b)| *a + *b)
            .collect();

        Vector::new(components)
    }
}

impl<T> Sub for &Vector<T>
where
    T: Number,
{
    type Output = Vector<T>;

    fn sub(self, other: &Vector<T>) -> Vector<T> {
        if self.dimension() != other.dimension() {
            panic!("Vectors must have the same dimension for subtraction");
        }

        let components = self.components
            .iter()
            .zip(other.components.iter())
            .map(|(a, b)| *a - *b)
            .collect();

        Vector::new(components)
    }
}

// Scalar multiplication
impl<T> Mul<T> for &Vector<T>
where
    T: Number,
{
    type Output = Vector<T>;

    fn mul(self, scalar: T) -> Vector<T> {
        let components = self.components
            .iter()
            .map(|&x| x * scalar)
            .collect();

        Vector::new(components)
    }
}

// Scalar division
impl<T> Div<T> for &Vector<T>
where
    T: Number,
{
    type Output = Vector<T>;

    fn div(self, scalar: T) -> Vector<T> {
        let components = self.components
            .iter()
            .map(|&x| x / scalar)
            .collect();

        Vector::new(components)
    }
}

// Negation
impl<T> Neg for &Vector<T>
where
    T: Number + Neg<Output = T>,
{
    type Output = Vector<T>;

    fn neg(self) -> Vector<T> {
        let components = self.components
            .iter()
            .map(|&x| -x)
            .collect();

        Vector::new(components)
    }
}

// Display implementation
impl<T> Display for Vector<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, component) in self.components.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", component)?;
        }
        write!(f, "]")
    }
}

// Convenience constructors for common vector sizes
impl<T> Vector<T>
where
    T: Number,
{
    /// Creates a 2D vector.
    pub fn vec2(x: T, y: T) -> Self {
        Self::new(vec![x, y])
    }

    /// Creates a 3D vector.
    pub fn vec3(x: T, y: T, z: T) -> Self {
        Self::new(vec![x, y, z])
    }

    /// Creates a 4D vector.
    pub fn vec4(x: T, y: T, z: T, w: T) -> Self {
        Self::new(vec![x, y, z, w])
    }
}

// Common vector constants for f64
impl Vector<f64> {
    /// Unit vector in the X direction (1, 0, 0).
    pub fn unit_x() -> Self {
        Self::vec3(1.0, 0.0, 0.0)
    }

    /// Unit vector in the Y direction (0, 1, 0).
    pub fn unit_y() -> Self {
        Self::vec3(0.0, 1.0, 0.0)
    }

    /// Unit vector in the Z direction (0, 0, 1).
    pub fn unit_z() -> Self {
        Self::vec3(0.0, 0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let v = Vector::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(v.dimension(), 3);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }

    #[test]
    fn test_vector_addition() {
        let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
        let v2 = Vector::new(vec![4.0, 5.0, 6.0]);
        let result = &v1 + &v2;
        assert_eq!(result.components(), &[5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
        let v2 = Vector::new(vec![4.0, 5.0, 6.0]);
        let dot = v1.dot(&v2);
        assert_eq!(dot, 32.0); // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
    }

    #[test]
    fn test_magnitude() {
        let v = Vector::new(vec![3.0, 4.0]);
        assert_eq!(v.magnitude(), 5.0); // sqrt(9 + 16) = sqrt(25) = 5
    }

    #[test]
    fn test_cross_product() {
        let v1 = Vector::new(vec![1.0, 0.0, 0.0]);
        let v2 = Vector::new(vec![0.0, 1.0, 0.0]);
        let cross = v1.cross(&v2);
        assert_eq!(cross.components(), &[0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_normalization() {
        let v = Vector::new(vec![3.0, 4.0]);
        let normalized = v.normalize();
        assert!((normalized.magnitude() - 1.0).abs() < 1e-10);
        assert_eq!(normalized.components(), &[0.6, 0.8]);
    }

    #[test]
    #[should_panic(expected = "Vector cannot have zero dimensions")]
    fn test_empty_vector_panics() {
        Vector::<f64>::new(vec![]);
    }

    #[test]
    fn test_integer_vectors() {
        let v1 = Vector::new(vec![1, 2, 3]);
        let v2 = Vector::new(vec![4, 5, 6]);
        let sum = &v1 + &v2;
        assert_eq!(sum.components(), &[5, 7, 9]);

        let dot = v1.dot(&v2);
        assert_eq!(dot, 32);
    }

    #[test]
    fn test_numeric_types_only() {
        // Test that various numeric types work
        let _f32_vector = Vector::new(vec![1.0f32, 2.0f32]);
        let _f64_vector = Vector::new(vec![1.0f64, 2.0f64]);
        let _i8_vector = Vector::new(vec![1i8, 2i8]);
        let _i16_vector = Vector::new(vec![1i16, 2i16]);
        let _i32_vector = Vector::new(vec![1i32, 2i32]);
        let _i64_vector = Vector::new(vec![1i64, 2i64]);
        let _isize_vector = Vector::new(vec![1isize, 2isize]);

        // All these should compile without issues
        // Non-numeric types like String, bool, Vec<T>, etc. would cause compile errors
    }

    #[test]
    fn test_negation_with_signed_types() {
        let v = Vector::new(vec![1, -2, 3]);
        let negated = -&v;
        assert_eq!(negated.components(), &[-1, 2, -3]);

        let float_v = Vector::new(vec![1.5, -2.5, 3.5]);
        let negated_float = -&float_v;
        assert_eq!(negated_float.components(), &[-1.5, 2.5, -3.5]);
    }
}
