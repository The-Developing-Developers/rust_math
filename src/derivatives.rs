//! This module implements numerical derivatives algorithms.
//!
//! For now, it only contains the `Derivative` struct with a single method `differentiate`, which performs numerical
//! differentiation using finite differences.
//!
//! TODO: add central difference method.

/// A struct that provides numerical differentiation methods.
pub struct Derivative;

impl Derivative {
    /// Performs numerical differentiation using finite differences.
    ///
    /// # Arguments
    ///
    /// * `function` - A closure that represents the function to differentiate.
    /// * `point` - The point at which to evaluate the derivative.
    /// * `h` - The step size for the finite difference approximation. A smaller value will yield a more accurate result, but may lead to numerical instability. Recommended value: `1e-5` or smaller.
    ///
    /// # Returns
    ///
    /// The approximate value of the derivative at the given point.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_math_lib::derivatives::Derivative;
    ///
    /// let result = Derivative::differentiate(|x| x * x, 2.0, 1e-5);
    /// println!("The derivative is approximately: {}", result);
    /// ```
    pub fn differentiate<F>(function: F, point: f64, increment: f64) -> f64
    where
        F: Fn(f64) -> f64, // `F` is a closure that takes an `f64` argument and returns a `f64`
    {
        let derivative = (function(point + increment) - function(point)) / increment; // Central difference formula
        derivative
    }
}

// ---- Tests ---- //

#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils::colours::{*};

    /// Helper function to test differentiation with common logic.
    fn test_differentiation<F>(
        function: F,
        point: f64,
        increment: f64,
        expected: f64,
        tolerance: f64,
    ) where
        F: Fn(f64) -> f64,
    {
        let result = Derivative::differentiate(function, point, increment);
        let delta = (result - expected).abs();
        println!(
            "{}Result{}:    {}\n{}Expected{}:  {}\n{}Tolerance{}: {}\n{}Delta{}:     {}",
            MAGENTA, RESET, result, CYAN, RESET, expected, YELLOW, RESET, tolerance, GREEN, RESET, delta
        );
        assert!(delta < tolerance);
    }

    #[test]
    fn test_differentiate_square() {
        test_differentiation(
            |x| x * x,
            2.0,
            1e-6,
            4.0,
            2e-6,
        );
    }

    #[test]
    fn test_differentiate_sine() {
        test_differentiation(
            |x| x.sin(),
            std::f64::consts::PI / 2.0,
            1e-6,
            0.0,
            1e-6,
        );
    }

    #[test]
    fn test_differentiate_exponential() {
        test_differentiation(
            |x| x.exp(),
            1.0,
            1e-8,
            std::f64::consts::E,
            1e-6,
        );
    }

    #[test]
    fn test_differentiate_exponential_neg() {
        test_differentiation(
            |x| (-x).exp(),
            -1.0,
            1e-8,
            -std::f64::consts::E,
            1e-6,
        );
    }

    #[test]
    fn test_differentiate_custom_function_1() {
        test_differentiation(
            |x| 5.0 * x.powi(3) + ((2.0 * x + 1.0).ln() / (3.0 * x - 2.0).sin()),
            1.0,
            1e-8,
            13.2773430404,
            1e-6,
        );
    }
}