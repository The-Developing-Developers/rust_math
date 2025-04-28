//! This module implements numerical integration algorithms.
//!
//! For now, it only contains the `Integrator` struct with a single method `integrate`, which performs numerical
//! integration using the Riemann sum method.

type Function = Box<dyn Fn(f64) -> f64>; // TODO: GS consider using a trait object instead of a function pointer, or commonise the type definiton since it is used in both `integrals` and `derivatives` modules

/// A struct that provides numerical integration methods.
pub struct Integral {
    pub function: Function, // Function to integrate
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub num_intervals: u64,
    result: f64,
}

impl Integral {
    pub fn new(function: Function, lower_bound: f64, upper_bound: f64, num_intervals: u64) -> Self {
        // TODO: GS consider returning a Result instead of a struct
        let num_intervals = if num_intervals > 0 {
            num_intervals
        } else {
            1e6 as u64 // Default value for number of intervals
        };

        Integral {
            function,
            lower_bound,
            upper_bound,
            num_intervals,
            result: 0.0,
        }
    }

    /// Performs numerical integration using the Riemann sum method.
    ///
    /// # Arguments
    ///
    /// * `function` - A closure that represents the function to integrate.
    /// * `lower_limit` - The lower limit of the integration.
    /// * `upper_limit` - The upper limit of the integration.
    /// * `num_intervals` - The number of intervals to divide the integration range into. The higher the number, the more accurate the result, but the slower the computation. Recommended value: `1e6` or higher.
    ///
    /// # Returns
    ///
    /// The approximate value of the integral.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_math_lib::integrals::Integral;
    ///
    /// let result = Integral::new(Box::new(|x| x * x), 0.0, 3.0, 1e6 as u64).integrate();
    /// println!("The integral is approximately: {}", result);
    /// ```
    pub fn integrate(&mut self) -> f64 {
        let width = (self.upper_bound - self.lower_bound) / self.num_intervals as f64; // Width of each slice of the interval

        for i in 0..self.num_intervals {
            let x_coordinate = self.lower_bound + i as f64 * width;
            self.result += (self.function)(x_coordinate) * width; // Infinitesimal area to be accumulated
        }

        self.result
    }
}

// ---- Tests ---- //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::colours::{CYAN, GREEN, MAGENTA, RESET, YELLOW};

    /// Helper function to test integration with common logic.
    fn test_integration(
        function: fn(f64) -> f64,
        lower_bound: f64,
        upper_bound: f64,
        num_intervals: u32,
        expected: f64,
        tolerance: f64,
    ) {
        let mut integral = Integral::new(
            Box::new(function),
            lower_bound,
            upper_bound,
            num_intervals as u64,
        );
        let result = integral.integrate();
        let delta = (result - expected).abs();
        println!(
            "{}Result{}:    {}\n{}Expected{}:  {}\n{}Tolerance{}: {}\n{}Delta{}:     {}",
            MAGENTA,
            RESET,
            result,
            CYAN,
            RESET,
            expected,
            YELLOW,
            RESET,
            tolerance,
            GREEN,
            RESET,
            delta
        );
        assert!(delta < tolerance);
    }

    #[test]
    fn test_integrate_square() {
        test_integration(|x| x * x, 0.0, 3.0, 1e7 as u32, 9.0, 1e-5);
    }

    #[test]
    fn test_integrate_sine() {
        test_integration(
            |x| x.sin(),
            0.0,
            std::f64::consts::PI,
            1e7 as u32,
            2.0,
            1e-5,
        );
    }

    #[test]
    fn test_integrate_sine_reverse() {
        test_integration(
            |x| x.sin(),
            std::f64::consts::PI,
            0.0,
            1e7 as u32,
            -2.0,
            1e-5,
        );
    }

    #[test]
    fn test_integrate_custom_function_1() {
        test_integration(
            |x| ((x + 2.0).powi(2) + 3.0 * (4.0 * x).sin()) / (x + 4.0),
            -1.0,
            3.0,
            1e7 as u32,
            7.15492507558,
            1e-5,
        );
    }

    #[test]
    fn test_integrate_custom_function_2() {
        test_integration(
            |x| ((0.5 * x + 3.0).ln() / (-x).cos()).sqrt(),
            -1.5,
            1.4,
            1e7 as u32,
            4.00152756063,
            1e-5,
        );
    }
}
