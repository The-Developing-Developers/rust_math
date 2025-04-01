//! This module implements numerical integration algorithms.
//!
//! For now, it only contains the `Integrator` struct with a single method `integrate`, which performs numerical
//! integration using the Riemann sum method.

/// A struct that provides numerical integration methods.
pub struct Integrator;

impl Integrator {
    /// Performs numerical integration using the Riemann sum method.
    ///
    /// # Arguments
    ///
    /// * `integrand_fn` - A closure that represents the function to integrate.
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
    /// use rust_math_lib::integrals::Integrator;
    ///
    /// let result = Integrator::integrate(|x| x * x, 0.0, 3.0, 1e6 as u32);
    /// println!("The integral is approximately: {}", result);
    /// ```
    pub fn integrate<F>(integrand_fn: F, lower_limit: f64, upper_limit: f64, num_intervals: u32) -> f64
    where
        F: Fn(f64) -> f64, // `F` is a closure that takes an `f64` argument and returns a `f64`
    {
        let mut sum = 0.0;
        let dx = (upper_limit - lower_limit) / num_intervals as f64; // Width of each slice of the interval

        for i in 0..num_intervals {
            let x = lower_limit + i as f64 * dx;
            sum += integrand_fn(x) * dx; // Infinitesimal area to be accumulated
        }

        sum
    }
}

// ---- Tests ---- //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::colours::{MAGENTA, RESET, CYAN, YELLOW, GREEN};

    /// Helper function to test integration with common logic.
    fn test_integration<F>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        num_intervals: u32,
        expected: f64,
        tolerance: f64,
    ) where
        F: Fn(f64) -> f64,
    {
        let result = Integrator::integrate(function, lower_limit, upper_limit, num_intervals);
        let delta = (result - expected).abs();
        println!(
            "{}Result{}:    {}\n{}Expected{}:  {}\n{}Tolerance{}: {}\n{}Delta{}:     {}",
            MAGENTA, RESET, result, CYAN, RESET, expected, YELLOW, RESET, tolerance, GREEN, RESET, delta
        );
        assert!(delta < tolerance);
    }

    #[test]
    fn test_integrate_square() {
        test_integration(
            |x| x * x,
            0.0,
            3.0,
            1e7 as u32,
            9.0,
            1e-5,
        );
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
