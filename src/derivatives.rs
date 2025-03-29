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

    #[test]
    fn test_differentiate_square() {
        let point = 2.0;
        let increment = 1e-6;
        let function = |x: f64| x * x;
        let result = Derivative::differentiate(function, point, increment);
        let expected = 4.0; // The derivative of x^2 at x = 2 is 2*x = 4
        let tolerance = 2e-6;
        let delta = (result - expected).abs();
        println!("Result:    {}\nExpected:  {}\nTolerance: {}\nDelta:     {}", result, expected, tolerance, delta);
        assert!(delta < tolerance, "Expected: {}, but got: {}", expected, result);
    }

    #[test]
    fn test_differentiate_sine() {
        let point = std::f64::consts::PI / 2.0;
        let increment = 1e-6;
        let function = |x: f64| (x).sin();
        let result = Derivative::differentiate(function, point, increment);
        let expected = 0.0; // The derivative of sin(x) at x = pi/2 is cos(pi/2) = 0
        let tolerance = 1e-6;
        let delta = (result - expected).abs();
        println!("Result:    {}\nExpected:  {}\nTolerance: {}\nDelta:     {}", result, expected, tolerance, delta);
        assert!(delta < tolerance, "Expected: {}, but got: {}", expected, result);
    }

    #[test]
    fn test_differentiate_exponential() {
        let point = 1.0;
        let increment = 1e-8;
        let function = |x: f64| (x).exp();
        let result = Derivative::differentiate(function, point, increment);
        let expected = std::f64::consts::E; // The derivative of e^x at x = 1 is e^1 = e
        let tolerance = 1e-6;
        let delta = (result - expected).abs();
        println!("Result:    {}\nExpected:  {}\nTolerance: {}\nDelta:     {}", result, expected, tolerance, delta);
        assert!(delta < tolerance, "Expected: {}, but got: {}", expected, result);
    }

    #[test]
    fn test_differentiate_exponential_neg() {
        let point = -1.0;
        let increment = 1e-8;
        let function = |x: f64| (-x).exp(); // e^(-x) = 1/e^x
        let result = Derivative::differentiate(function, point, increment);
        let expected = -std::f64::consts::E; // The derivative of e^(-x) at x = -1 is -e^(-(-1)) = -e
        let tolerance = 1e-6;
        let delta = (result - expected).abs();
        println!("Result:    {}\nExpected:  {}\nTolerance: {}\nDelta:     {}", result, expected, tolerance, delta);
        assert!(delta < tolerance, "Expected: {}, but got: {}", expected, result);
    }

    #[test]
    fn test_differentiate_custom_function_1() {
        let point = 1.0;
        let increment = 1e-8;
        let function = |x: f64| 5.0 * x.powi(3) + ((2.0 * x + 1.0).ln() / (3.0 * x - 2.0).sin());
        let result = Derivative::differentiate(function, point, increment);
        let expected = 13.2773430404; // Thanks to Desmos
        let tolerance = 1e-6;
        let delta = (result - expected).abs();
        println!("Result:    {}\nExpected:  {}\nTolerance: {}\nDelta:     {}", result, expected, tolerance, delta);
        assert!(delta < tolerance, "Expected: {}, but got: {}", expected, result);
    }
}