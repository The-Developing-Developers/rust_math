//! This module implements numerical derivatives algorithms.
//!
//! It contains the `Derivative` struct, with three methods which perform numerical differentiation:
//! - `forward_difference`: Uses the forward difference method to approximate the derivative of a function at a specified point.
//! - `backward_difference`: Uses the backward difference method to approximate the derivative of a function at a specified point.
//! - `central_difference`: Uses the central difference method to approximate the derivative of a function at a specified point.

type Function = Box<dyn Fn(f64) -> f64>; // TODO: GS consider using a trait object instead of a function pointer, or commonise the type definition since it is used in both `integrals` and `derivatives` modules

/// A struct that provides numerical differentiation methods.
pub struct Derivative {
    pub function: Function,
    pub x_coordinate: f64,
    pub increment: f64,
    result: f64,
}

impl Derivative {
    pub fn new(function: Function, x_coordinate: f64, increment: f64) -> Self {
        // TODO: GS consider returning a Result instead of a struct
        let increment = if increment > 0.0 {
            increment
        } else {
            1e-6 // Default value for increment
        };

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

    // TODO: GS add a copy constructor (`from`) as in `Integrator`

    /// Performs numerical differentiation using forward difference.
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
    /// let mut derivative = Derivative::new(Box::new(|x| x * x), 2.0, 1e-6);
    /// let result = derivative.forward_difference();
    /// ```
    pub fn forward_difference(&mut self) -> f64 {
        self.result = ((self.function)(self.x_coordinate + self.increment)
            - (self.function)(self.x_coordinate))
            / self.increment;
        self.result
    }

    /// Performs numerical differentiation using backward difference.
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
    /// /// # Example
    /// /// ```
    /// use rust_math_lib::derivatives::Derivative;
    /// let mut derivative = Derivative::new(Box::new(|x| x * x), 2.0, 1e-6);
    /// let result = derivative.backward_difference();
    /// /// ```
    pub fn backward_difference(&mut self) -> f64 {
        self.result = ((self.function)(self.x_coordinate)
            - (self.function)(self.x_coordinate - self.increment))
            / self.increment;
        self.result
    }

    /// Performs numerical differentiation using central difference.
    /// # Arguments
    /// * `function` - A closure that represents the function to differentiate.
    /// * `point` - The point at which to evaluate the derivative.
    /// * `h` - The step size for the finite difference approximation. A smaller value will yield a more accurate result, but may lead to numerical instability. Recommended value: `1e-5` or smaller.
    /// # Returns
    /// The approximate value of the derivative at the given point.
    /// # Example
    /// ```
    /// use rust_math_lib::derivatives::Derivative;
    /// let mut derivative = Derivative::new(Box::new(|x| x * x), 2.0, 1e-6);
    /// let result = derivative.central_difference();
    /// ```
    pub fn central_difference(&mut self) -> f64 {
        let half_increment = self.increment / 2.0;
        self.result = ((self.function)(self.x_coordinate + half_increment)
            - (self.function)(self.x_coordinate - half_increment))
            / self.increment;
        self.result
    }
}

// ---- Tests ---- //

#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils::colours::*;

    // TODO: GS use a parameterised test framework (e.g., `rstest` crate) to test multiple methods and configurations in a more structured way instead of repeating the same code for each test?

    /// Helper function to test differentiation using common logic.
    ///
    /// For each `function` passed as an argument, it will test the derivative using three methods: forward, backward, and central difference.
    ///
    /// /// # Arguments
    /// * `function` - The function to differentiate.
    /// * `x_coordinate` - The point at which to evaluate the derivative.
    /// * `increment` - The step size for the finite difference approximation.
    /// * `expected` - The expected value of the derivative.
    /// * `error_tolerance` - The acceptable error tolerance for the test.
    fn test_differentiation(
        function: fn(f64) -> f64,
        x_coordinate: f64,
        increment: f64,
        expected: f64,
        error_tolerance: f64,
    ) {
        let mut derivative = Derivative::new(Box::new(function), x_coordinate, increment);
        let results_vec = vec![
            ("Forward", derivative.forward_difference()),
            ("Backward", derivative.backward_difference()),
            ("Cental", derivative.central_difference()),
        ];

        for (method_name, result) in results_vec {
            let delta = (result - expected).abs();
            println!(
                "Method: {}\n  {}Result{}:    {}\n  {}Expected{}:  {}\n  {}Tolerance{}: {}\n  {}Delta{}:     {}",
                method_name,
                CYAN,
                RESET,
                result,
                YELLOW,
                RESET,
                expected,
                GREEN,
                RESET,
                error_tolerance,
                WHITE,
                RESET,
                delta
            );

            assert!(
                delta < error_tolerance,
                "Test failed for {} method",
                method_name
            );
        }
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
