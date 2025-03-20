//! Entry point for the `rust_math_lib` library.

/// The `integrals` module provides functions for performing integral calculations.
pub mod integrals;

#[cfg(test)]
mod tests {
    use super::*;

    /// Test for the `temp_test_function` in the `integrals` module.
    #[test]
    fn temp_test_function() {
        let result = integrals::temp_test_function();
        assert_eq!(result, 3);
    }
}