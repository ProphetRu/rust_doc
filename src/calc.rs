//! # Calc
//!
//! This is a simple tutorial on how to use documentation in Rust.

/// The function adds two numbers and returns the sum.
///
/// # Examples
///
/// ```
/// let answer = calc::plus(2, 3);
/// assert_eq!(answer, 5);
/// ```
pub fn plus(x: i32, y: i32) -> i32 {
    x + y
}

/// The function subtracts two numbers and returns the difference.
///
/// # Examples
///
/// ```
/// let answer = calc::minus(6, 3);
/// assert_eq!(answer, 2);
/// ```
pub fn minus(x: i32, y: i32) -> i32 {
    x - y
}

/// The function multiplies two numbers and returns the product.
///
/// # Examples
///
/// ```
/// let answer = calc::mul(2, 4);
/// assert_eq!(answer, 8);
/// ```
pub fn mul(x: i32, y: i32) -> i32 {
    x * y
}

/// The function divides two numbers and returns the quotient or an error.
///
/// # Examples
///
/// ### Example Ok
/// ```
/// let answer = calc::div(8, 2);
/// assert!(result.is_ok());
/// assert_eq!(answer.unwrap(), 4);
/// ```
///
/// ### Example Err
/// ```
/// let answer = calc::div(8, 0);
/// assert!(answer.is_err());
/// ```
pub fn div(x: i32, y: i32) -> Result<i32, ()> {
    if y == 0 {
        return Err(());
    }

    Ok(x / y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus() {
        assert_eq!(plus(1, 2), 3);
    }

    #[test]
    fn test_minus() {
        assert_eq!(minus(5, 2), 3);
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul(5, 2), 10);
    }

    #[test]
    fn test_div_ok() {
        let result = div(10, 2);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5);
    }

    #[test]
    fn test_div_fail() {
        let result = div(10, 0);
        assert!(result.is_err());
    }
}