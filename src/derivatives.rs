//! This module implements numerical derivatives algorithms.
//!
//! For now, it only contains the `Derivative` struct with a single method `forward_difference`, which performs numerical
//! differentiation using finite differences.

/// A struct that provides numerical differentiation methods.
pub struct Derivative {
    pub function: fn(f64) -> f64,
    pub x_coordinate: f64,
    pub increment: f64,
    result: f64,
}

impl Derivative {
    pub fn new(function: fn(f64) -> f64, x_coordinate: f64, increment: f64) -> Self {
        assert!(
            // TODO: GS consider removing the assert!() and use a Result type instead.
            increment > 0.0,
            "Infinitesimal increment must be greater than zero."
        );
        Derivative {
            function,
            x_coordinate,
            increment,
            result: 0.0,
        }
    }

    pub fn get_result(&self) -> f64 {
        self.result
    }

    // TODO: GS add a print result method to print the result of the derivative

    // TODO: GS add central and backward difference methods

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
    /// let mut derivative = Derivative::new(|x| x * x, 2.0, 1e-6);
    /// let result = derivative.forward_difference();
    /// ```
    pub fn forward_difference(&mut self) -> f64 // TODO: GS is it possible to only modify the result field and not the whole struct?
    {
        self.result = ((self.function)(self.x_coordinate + self.increment)
            - (self.function)(self.x_coordinate))
            / self.increment; // Forward difference formula
        self.result
    }
}

// ---- Tests ---- //

#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils::colours::*;

    /// Helper function to test differentiation with common logic.
    fn test_differentiation(
        function: fn(f64) -> f64,
        x_coordinate: f64,
        increment: f64,
        expected: f64,
        tolerance: f64,
    ) {
        let mut derivative = Derivative::new(function, x_coordinate, increment); // TODO: GS is it possible to only modify the result field and not the whole struct?
        let result = derivative.forward_difference();
        let delta = (result - expected).abs();
        println!(
            "{}Result{}:    {}\n{}Expected{}:  {}\n{}Tolerance{}: {}\n{}Delta{}:     {}",
            MAGENTA, RESET, result,
            CYAN,    RESET, expected,
            YELLOW,  RESET, tolerance,
            GREEN,   RESET, delta
        );
        assert!(delta < tolerance);
    }

    #[test]
    fn test_differentiate_square() {
        test_differentiation(|x| x * x, 2.0, 1e-6, 4.0, 2e-6);
    }

    #[test]
    fn test_differentiate_sine() {
        test_differentiation(|x| x.sin(), std::f64::consts::PI / 2.0, 1e-6, 0.0, 1e-6);
    }

    #[test]
    fn test_differentiate_exponential() {
        test_differentiation(|x| x.exp(), 1.0, 1e-8, std::f64::consts::E, 1e-6);
    }

    #[test]
    fn test_differentiate_exponential_neg() {
        test_differentiation(|x| (-x).exp(), -1.0, 1e-8, -std::f64::consts::E, 1e-6);
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
