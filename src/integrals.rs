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

    #[test]
    fn test_integrate_square() {
        let lower_limit = 0.0;
        let upper_limit = 3.0;
        let num_intervals = 1e8 as u32;
        let function = |x: f64| x * x;
        let result = Integrator::integrate(function, lower_limit, upper_limit, num_intervals);
        let expected = 9.0;
        let tolerance = 1e-6;
        let delta = (result - expected).abs();
        println!("Result:    {}\nExpected:  {}\nTolerance: {}\nDelta:     {}", result, expected, tolerance, delta);
        assert!(delta < tolerance);
    }

    #[test]
    fn test_integrate_sine() {
        let lower_limit = 0.0;
        let upper_limit = std::f64::consts::PI;
        let num_intervals = 1e8 as u32;
        let function = |x: f64| (x).sin();
        let result = Integrator::integrate(function, lower_limit, upper_limit, num_intervals);
        let expected = 2.0;
        let tolerance = 1e-6;
        let delta = (result - expected).abs();
        println!("Result:    {}\nExpected:  {}\nTolerance: {}\nDelta:     {}", result, expected, tolerance, delta);
        assert!(delta < tolerance);
    }

    #[test]
    fn test_integrate_sine_reverse() {
        let lower_limit = std::f64::consts::PI;
        let upper_limit = 0.0;
        let num_intervals = 1e8 as u32;
        let function = |x: f64| (x).sin();
        let result = Integrator::integrate(function, lower_limit, upper_limit, num_intervals);
        let expected = -2.0;
        let tolerance = 1e-6;
        let delta = (result - expected).abs();
        println!("Result:    {}\nExpected:  {}\nTolerance: {}\nDelta:     {}", result, expected, tolerance, delta);
        assert!(delta < tolerance);
    }
}