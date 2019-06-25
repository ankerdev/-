pub use super::functions::*;

#[allow(dead_code)]
const MY_LUCKY_NUMBER: i32 = 22;

// The `cfg(test)` attribute signifies that the module will only
// compile and run when `cargo test` is ran.
#[cfg(test)]
mod tests {
    // Import constants and variables from parent scope
    // use super::super::functions::add;
    use super::*;
    // If we don't want to import everything we can selectively import variables/functions like so:
    // use super::add;
    // use super::MY_LUCKY_NUMBER;

    // Every test should start with the `test` attribute
    #[test]
    fn test_add() {
        assert_eq!(10, add(5, 5));
        assert_ne!(10, add(5, 4));
    }

    #[test]
    fn test_my_favorite_number() {
        assert_eq!(22, MY_LUCKY_NUMBER);
    }
}
