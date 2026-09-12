//! # Edsarah Software and Solution — Scientific Calculator (core library)
//!
//! Pure calculation logic behind the `edsarah-calculator` command line program.
//! Recursion, error handling and overflow checks all live in this file.

use std::fmt;

/// The largest input accepted by [`factorial`].
pub const MAX_FACTORIAL_INPUT: u64 = 20;

/// Help text shared by the CLI and the interactive prompt.
pub const HELP_TEXT: &str = "\
Edsarah Software and Solution - Scientific Calculator

Usage:
  edsarah-calculator <operation> [operand ...]   run a single calculation
  edsarah-calculator                             start the interactive prompt

Operations:
  add | +          <a> <b>   a + b
  sub | -          <a> <b>   a - b
  mul | *          <a> <b>   a * b
  div | /          <a> <b>   a / b
  pow | ^          <a> <b>   a raised to the power b
  factorial | !    <n>       n!  (recursive, 0 <= n <= 20)
  sqrt             <a>       square root of a
  exp              <a>       e raised to the power a
  ln               <a>       natural logarithm of a
  log10            <a>       base-10 logarithm of a
  sin | cos | tan  <a>       trigonometric function, angle in radians
  abs              <a>       absolute value of a

Interactive commands:
  help, ?          show this text
  quit, exit       leave the program

Examples:
  edsarah-calculator factorial 5
  edsarah-calculator add 2 3
  edsarah-calculator pow 2 10
";

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Every way in which a calculation can fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalcError {
    NegativeInput { value: String },
    NotANumber { value: String },
    Overflow { context: String },
    DivisionByZero,
    UnknownOperation { name: String },
    InvalidArgumentCount {
        operation: String,
        expected: usize,
        got: usize,
    },
    DomainError { function: String, value: String },
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::NegativeInput { value } => write!(
                f,
                "'{value}' is negative, but this operation only accepts non-negative values"
            ),
            CalcError::NotANumber { value } => {
                write!(f, "'{value}' is not a valid number")
            }
            CalcError::Overflow { context } => {
                write!(f, "the result is too large to represent ({context})")
            }
            CalcError::DivisionByZero => write!(f, "division by zero is undefined"),
            CalcError::UnknownOperation { name } => write!(
                f,
                "'{name}' is not a known operation (type 'help' for a list)"
            ),
            CalcError::InvalidArgumentCount {
                operation,
                expected,
                got,
            } => write!(
                f,
                "'{operation}' expects {expected} operand(s) but received {got}"
            ),
            CalcError::DomainError { function, value } => {
                write!(f, "'{value}' is outside the domain of {function}")
            }
        }
    }
}

impl std::error::Error for CalcError {}

// ---------------------------------------------------------------------------
// Parsing
// ---------------------------------------------------------------------------

/// Parses a non-negative integer such as the argument of `factorial`.
pub fn parse_u64(raw: &str) -> Result<u64, CalcError> {
    let trimmed = raw.trim();

    if trimmed.is_empty() {
        return Err(CalcError::NotANumber {
            value: raw.to_string(),
        });
    }

    if trimmed.starts_with('-') {
        return Err(CalcError::NegativeInput {
            value: trimmed.to_string(),
        });
    }

    trimmed.parse::<u64>().map_err(|_| CalcError::NotANumber {
        value: trimmed.to_string(),
    })
}

/// Parses a floating point number, rejecting `inf` and `NaN`.
pub fn parse_f64(raw: &str) -> Result<f64, CalcError> {
    let trimmed = raw.trim();

    if trimmed.is_empty() {
        return Err(CalcError::NotANumber {
            value: raw.to_string(),
        });
    }

    let value = trimmed.parse::<f64>().map_err(|_| CalcError::NotANumber {
        value: trimmed.to_string(),
    })?;

    if value.is_finite() {
        Ok(value)
    } else {
        Err(CalcError::NotANumber {
            value: trimmed.to_string(),
        })
    }
}

/// Converts NaN into a domain error and infinity into an overflow error.
fn checked(value: f64, context: &str) -> Result<f64, CalcError> {
    if value.is_nan() {
        Err(CalcError::DomainError {
            function: context.to_string(),
            value: "the result is undefined".to_string(),
        })
    } else if value.is_infinite() {
        Err(CalcError::Overflow {
            context: context.to_string(),
        })
    } else {
        Ok(value)
    }
}

// ---------------------------------------------------------------------------
// Recursive routines
// ---------------------------------------------------------------------------

/// Computes `n!` recursively, with overflow protection.
pub fn factorial(n: u64) -> Result<u64, CalcError> {
    if n > MAX_FACTORIAL_INPUT {
        return Err(CalcError::Overflow {
            context: format!(
                "{n}! is larger than {MAX_FACTORIAL_INPUT}! and does not fit in a 64-bit integer"
            ),
        });
    }

    factorial_recursive(n)
}

/// The private worker that performs the recursion.
fn factorial_recursive(n: u64) -> Result<u64, CalcError> {
    // Base case: 0! = 1.
    if n == 0 {
        return Ok(1);
    }

    // Recursive case: n! = n * (n - 1)!
    let previous = factorial_recursive(n - 1)?;

    previous.checked_mul(n).ok_or_else(|| CalcError::Overflow {
        context: format!("{n}! exceeds the range of a 64-bit unsigned integer"),
    })
}

/// Raises `base` to an integer `exponent` using recursion.
pub fn power(base: f64, exponent: i64) -> Result<f64, CalcError> {
    if exponent == 0 {
        return Ok(1.0);
    }

    if exponent < 0 {
        let positive = exponent.checked_neg().ok_or_else(|| CalcError::Overflow {
            context: "the exponent is too small to negate".to_string(),
        })?;

        let denominator = power_positive(base, positive as u64);
        if denominator == 0.0 {
            return Err(CalcError::DivisionByZero);
        }
        return checked(1.0 / denominator, "negative exponent");
    }

    checked(power_positive(base, exponent as u64), "power")
}

/// Fast exponentiation: `base^exponent` for `exponent >= 0`.
fn power_positive(base: f64, exponent: u64) -> f64 {
    if exponent == 0 {
        return 1.0;
    }

    let half = power_positive(base, exponent / 2);
    if exponent % 2 == 0 {
        half * half
    } else {
        half * half * base
    }
}

// ---------------------------------------------------------------------------
// Arithmetic and scientific functions
// ---------------------------------------------------------------------------

pub fn add(left: f64, right: f64) -> Result<f64, CalcError> {
    checked(left + right, "addition")
}

pub fn subtract(left: f64, right: f64) -> Result<f64, CalcError> {
    checked(left - right, "subtraction")
}

pub fn multiply(left: f64, right: f64) -> Result<f64, CalcError> {
    checked(left * right, "multiplication")
}

pub fn divide(left: f64, right: f64) -> Result<f64, CalcError> {
    if right == 0.0 {
        return Err(CalcError::DivisionByZero);
    }
    checked(left / right, "division")
}

pub fn power_pair(base: f64, exponent: f64) -> Result<f64, CalcError> {
    let fits_in_i64 = exponent >= i64::MIN as f64 && exponent <= i64::MAX as f64;

    if exponent.fract() == 0.0 && fits_in_i64 {
        return power(base, exponent as i64);
    }

    checked(base.powf(exponent), "power")
}

pub fn sqrt(value: f64) -> Result<f64, CalcError> {
    if value < 0.0 {
        return Err(CalcError::DomainError {
            function: "the square root".to_string(),
            value: value.to_string(),
        });
    }
    checked(value.sqrt(), "square root")
}

pub fn exp(value: f64) -> Result<f64, CalcError> {
    checked(value.exp(), "exponential")
}

pub fn ln(value: f64) -> Result<f64, CalcError> {
    if value <= 0.0 {
        return Err(CalcError::DomainError {
            function: "the natural logarithm".to_string(),
            value: value.to_string(),
        });
    }
    checked(value.ln(), "natural logarithm")
}

pub fn log10(value: f64) -> Result<f64, CalcError> {
    if value <= 0.0 {
        return Err(CalcError::DomainError {
            function: "the base-10 logarithm".to_string(),
            value: value.to_string(),
        });
    }
    checked(value.log10(), "base-10 logarithm")
}

pub fn sin(value: f64) -> Result<f64, CalcError> {
    checked(value.sin(), "sine")
}

pub fn cos(value: f64) -> Result<f64, CalcError> {
    checked(value.cos(), "cosine")
}

pub fn tan(value: f64) -> Result<f64, CalcError> {
    checked(value.tan(), "tangent")
}

pub fn abs(value: f64) -> Result<f64, CalcError> {
    checked(value.abs(), "absolute value")
}

// ---------------------------------------------------------------------------
// Operation dispatch
// ---------------------------------------------------------------------------

fn expect_unary<'a>(operation: &str, operands: &'a [String]) -> Result<&'a str, CalcError> {
    if operands.len() != 1 {
        return Err(CalcError::InvalidArgumentCount {
            operation: operation.to_string(),
            expected: 1,
            got: operands.len(),
        });
    }
    Ok(operands[0].as_str())
}

fn expect_binary<'a>(
    operation: &str,
    operands: &'a [String],
) -> Result<(&'a str, &'a str), CalcError> {
    if operands.len() != 2 {
        return Err(CalcError::InvalidArgumentCount {
            operation: operation.to_string(),
            expected: 2,
            got: operands.len(),
        });
    }
    Ok((operands[0].as_str(), operands[1].as_str()))
}

fn binary_op<F>(operation: &str, operands: &[String], function: F) -> Result<f64, CalcError>
where
    F: Fn(f64, f64) -> Result<f64, CalcError>,
{
    let (left_raw, right_raw) = expect_binary(operation, operands)?;
    let left = parse_f64(left_raw)?;
    let right = parse_f64(right_raw)?;
    function(left, right)
}

fn unary_op<F>(operation: &str, operands: &[String], function: F) -> Result<f64, CalcError>
where
    F: Fn(f64) -> Result<f64, CalcError>,
{
    let raw = expect_unary(operation, operands)?;
    let value = parse_f64(raw)?;
    function(value)
}

/// Turns a textual command such as `"factorial 5"` into a numeric answer.
pub fn evaluate(operation: &str, operands: &[String]) -> Result<f64, CalcError> {
    let op = operation.trim().to_ascii_lowercase();

    match op.as_str() {
        "add" | "+" | "plus" => binary_op(&op, operands, add),
        "sub" | "-" | "subtract" | "minus" => binary_op(&op, operands, subtract),
        "mul" | "*" | "x" | "multiply" => binary_op(&op, operands, multiply),
        "div" | "/" | "divide" => binary_op(&op, operands, divide),
        "pow" | "^" | "power" => binary_op(&op, operands, power_pair),
        "factorial" | "fact" | "!" => {
            let raw = expect_unary(&op, operands)?;
            let n = parse_u64(raw)?;
            factorial(n).map(|value| value as f64)
        }
        "sqrt" | "root" => unary_op(&op, operands, sqrt),
        "exp" => unary_op(&op, operands, exp),
        "ln" => unary_op(&op, operands, ln),
        "log10" | "log" => unary_op(&op, operands, log10),
        "sin" => unary_op(&op, operands, sin),
        "cos" => unary_op(&op, operands, cos),
        "tan" => unary_op(&op, operands, tan),
        "abs" => unary_op(&op, operands, abs),
        _ => Err(CalcError::UnknownOperation {
            name: operation.trim().to_string(),
        }),
    }
}

// ---------------------------------------------------------------------------
// Output formatting
// ---------------------------------------------------------------------------

/// Formats a result for display.
pub fn format_result(value: f64) -> String {
    if value == 0.0 {
        return "0".to_string();
    }

    let magnitude = value.abs();

    if value.fract() == 0.0 && magnitude < 1.8e19 {
        format!("{value:.0}")
    } else if magnitude >= 1.0e15 || magnitude < 1.0e-4 {
        format!("{value:e}")
    } else {
        format!("{value}")
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn args(items: &[&str]) -> Vec<String> {
        items.iter().map(|item| item.to_string()).collect()
    }

    #[test]
    fn factorial_of_zero_is_one() {
        assert_eq!(factorial(0), Ok(1));
    }

    #[test]
    fn factorial_of_one_is_one() {
        assert_eq!(factorial(1), Ok(1));
    }

    #[test]
    fn factorial_of_five_is_one_hundred_twenty() {
        assert_eq!(factorial(5), Ok(120));
    }

    #[test]
    fn factorial_of_twenty_is_the_largest_that_fits_in_u64() {
        assert_eq!(factorial(20), Ok(2_432_902_008_176_640_000));
    }

    #[test]
    fn factorial_of_twenty_one_overflows() {
        assert!(matches!(factorial(21), Err(CalcError::Overflow { .. })));
    }

    #[test]
    fn parse_u64_accepts_plain_digits() {
        assert_eq!(parse_u64("12"), Ok(12));
    }

    #[test]
    fn parse_u64_ignores_surrounding_whitespace() {
        assert_eq!(parse_u64("  7  "), Ok(7));
    }

    #[test]
    fn parse_u64_rejects_negative_numbers() {
        assert!(matches!(
            parse_u64("-5"),
            Err(CalcError::NegativeInput { .. })
        ));
    }

    #[test]
    fn parse_u64_rejects_non_numeric_text() {
        assert!(matches!(
            parse_u64("banana"),
            Err(CalcError::NotANumber { .. })
        ));
    }

    #[test]
    fn parse_u64_rejects_empty_text() {
        assert!(matches!(parse_u64("   "), Err(CalcError::NotANumber { .. })));
    }

    #[test]
    fn parse_f64_accepts_decimals() {
        assert_eq!(parse_f64("3.5"), Ok(3.5));
    }

    #[test]
    fn parse_f64_rejects_infinity_and_nan() {
        assert!(parse_f64("inf").is_err());
        assert!(parse_f64("NaN").is_err());
    }

    #[test]
    fn addition_works() {
        assert_eq!(add(2.0, 3.0), Ok(5.0));
    }

    #[test]
    fn subtraction_works() {
        assert_eq!(subtract(10.0, 4.0), Ok(6.0));
    }

    #[test]
    fn multiplication_works() {
        assert_eq!(multiply(6.0, 7.0), Ok(42.0));
    }

    #[test]
    fn division_works() {
        assert_eq!(divide(9.0, 3.0), Ok(3.0));
    }

    #[test]
    fn division_by_zero_is_reported() {
        assert_eq!(divide(1.0, 0.0), Err(CalcError::DivisionByZero));
    }

    #[test]
    fn power_of_zero_exponent_is_one() {
        assert_eq!(power(5.0, 0), Ok(1.0));
    }

    #[test]
    fn power_uses_recursion_for_positive_exponents() {
        assert_eq!(power(2.0, 10), Ok(1024.0));
    }

    #[test]
    fn power_handles_negative_exponents() {
        assert_eq!(power(2.0, -1), Ok(0.5));
    }

    #[test]
    fn power_of_zero_with_negative_exponent_is_division_by_zero() {
        assert_eq!(power(0.0, -2), Err(CalcError::DivisionByZero));
    }

    #[test]
    fn power_overflow_is_detected() {
        assert!(matches!(
            power(10.0, 400),
            Err(CalcError::Overflow { .. })
        ));
    }

    #[test]
    fn square_root_works() {
        assert_eq!(sqrt(16.0), Ok(4.0));
    }

    #[test]
    fn square_root_of_negative_is_a_domain_error() {
        assert!(matches!(sqrt(-1.0), Err(CalcError::DomainError { .. })));
    }

    #[test]
    fn natural_log_of_zero_is_a_domain_error() {
        assert!(matches!(ln(0.0), Err(CalcError::DomainError { .. })));
    }

    #[test]
    fn log10_of_one_thousand_is_three() {
        let result = log10(1000.0).expect("1000 is in the domain of log10");
        assert!((result - 3.0).abs() < 1e-12, "expected 3.0, got {result}");
    }

    #[test]
    fn evaluate_runs_factorial() {
        assert_eq!(evaluate("factorial", &args(&["5"])), Ok(120.0));
    }

    #[test]
    fn evaluate_is_case_insensitive() {
        assert_eq!(evaluate("FACTORIAL", &args(&["5"])), Ok(120.0));
    }

    #[test]
    fn evaluate_runs_addition() {
        assert_eq!(evaluate("add", &args(&["2", "3"])), Ok(5.0));
    }

    #[test]
    fn evaluate_rejects_unknown_operation() {
        assert!(matches!(
            evaluate("frobnicate", &args(&["1"])),
            Err(CalcError::UnknownOperation { .. })
        ));
    }

    #[test]
    fn evaluate_rejects_wrong_operand_count() {
        assert!(matches!(
            evaluate("add", &args(&["1"])),
            Err(CalcError::InvalidArgumentCount { .. })
        ));
    }

    #[test]
    fn evaluate_reports_negative_factorial_input() {
        assert!(matches!(
            evaluate("factorial", &args(&["-3"])),
            Err(CalcError::NegativeInput { .. })
        ));
    }

    #[test]
    fn whole_numbers_are_printed_without_a_decimal_point() {
        assert_eq!(format_result(1024.0), "1024");
    }

    #[test]
    fn zero_is_printed_as_zero() {
        assert_eq!(format_result(0.0), "0");
        assert_eq!(format_result(-0.0), "0");
    }

    #[test]
    fn fractions_keep_their_decimal_part() {
        assert_eq!(format_result(0.5), "0.5");
    }
}