///
/// TODO: write a doc comment to describe the crate
///
pub mod integrals;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temp_test_function() {
        let result = integrals::temp_test_function();
        assert_eq!(result, 3);
    }
}
