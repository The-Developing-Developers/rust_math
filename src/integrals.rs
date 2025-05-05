//! This module implements numerical integration algorithms.
//!
//! It contains the `Integrator` struct, with two methods which perform numerical integration:
//! - `riemann_integration`: Uses the Riemann sum method to approximate the integral of a function over a specified interval.
//! - `simpson_integration_one_third`: Uses Simpson's 1/3 rule to approximate the integral of a function over a specified interval.

type Function = Box<dyn Fn(f64) -> f64>;

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
    /// let result = Integral::new(Box::new(|x| x * x), 0.0, 3.0, 1e6 as u64).riemann_integration();
    /// println!("The integral is approximately: {}", result);
    /// ```
    pub fn riemann_integration(&mut self) -> f64 {
        let width = (self.upper_bound - self.lower_bound) / self.num_intervals as f64; // Width of each slice of the interval

        for i in 0..self.num_intervals {
            let x_coordinate = self.lower_bound + i as f64 * width;
            self.result += (self.function)(x_coordinate) * width; // Infinitesimal area to be accumulated
        }

        self.result
    }

    /// Performs numerical integration using Simpson's 1/3 rule.
    /// Simpson's 1/3 rule approximates the integrand function with the a quadratic interpolant.
    pub fn simpson_integration_one_third(&mut self) -> f64 {
        let width = (self.upper_bound - self.lower_bound) / self.num_intervals as f64; // Width of each slice of the interval

        for i in 0..self.num_intervals {
            let x_coordinate = self.lower_bound + i as f64 * width;
            let x_next = x_coordinate + width;
            let x_mid = (x_coordinate + x_next) / 2.0;

            // Simpson's rule: f(a) + 4f(m) + f(b)
            self.result += (self.function)(x_coordinate)
                + 4.0 * (self.function)(x_mid)
                + (self.function)(x_next);
        }

        self.result *= width / 6.0; // Last step can be factored out of the integral, because it is constant
        self.result
    }
}

// ---- Tests ---- //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::colours::*;

    /// Helper function to test integration with common logic.
    fn test_integration(
        function: fn(f64) -> f64,
        lower_bound: f64,
        upper_bound: f64,
        num_intervals: u32,
        expected: f64,
        error_tolerance: f64,
    ) {
        let mut integral = Integral::new(
            Box::new(function),
            lower_bound,
            upper_bound,
            num_intervals as u64,
        );
        let results_vec = vec![
            ("Riemann", integral.riemann_integration()),
            ("Simpson's 1/3", integral.simpson_integration_one_third()),
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
