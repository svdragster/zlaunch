//! Expression detection for the calculator feature.
//!
//! Determines whether user input looks like a mathematical expression
//! that should be evaluated by the calculator.

use lazy_static::lazy_static;
use regex::Regex;

/// Known mathematical function names supported by fasteval.
const MATH_FUNCTIONS: &[&str] = &[
    "sin", "cos", "tan", "asin", "acos", "atan", "sinh", "cosh", "tanh", "asinh", "acosh", "atanh",
    "sqrt", "abs", "ceil", "floor", "round", "log", "ln", "exp", "min", "max", "pi", "e",
];

lazy_static! {
    /// Matches strings containing only math-safe characters.
    /// Allows: digits, whitespace, operators, parentheses, letters (for functions), dots, commas.
    static ref MATH_SAFE_CHARS: Regex = Regex::new(
        r"^[\d\s\.\,\+\-\*/%\^()a-zA-Z_]+$"
    ).unwrap();

    /// Matches parentheses that contain something (not empty).
    static ref HAS_PARENS: Regex = Regex::new(
        r"\([^)]+\)"
    ).unwrap();
}

/// Check if input looks like a potential calculator expression.
///
/// Returns `true` if the input:
/// 1. Contains only math-safe characters
/// 2. Has at least one operator, function call, or non-trivial parentheses
/// 3. Is not just a plain number
///
/// This is a fast pre-check before attempting actual evaluation.
pub fn looks_like_expression(input: &str) -> bool {
    let trimmed = input.trim();

    // Too short or empty
    if trimmed.len() < 2 {
        return false;
    }

    // Must contain only math-safe characters
    if !MATH_SAFE_CHARS.is_match(trimmed) {
        return false;
    }

    // Check if it's just a plain number (with optional decimals/commas)
    if is_plain_number(trimmed) {
        return false;
    }

    // Must have at least one of: binary operator, function, or parentheses
    has_operator(trimmed) || has_function(trimmed) || HAS_PARENS.is_match(trimmed)
}

/// Check if the input is just a plain number (no operations).
fn is_plain_number(input: &str) -> bool {
    let cleaned: String = input.chars().filter(|c| !c.is_whitespace()).collect();

    // Allow optional leading minus for negative numbers
    let to_check = cleaned.strip_prefix('-').unwrap_or(&cleaned);

    // A plain number contains only digits, dots, and commas
    !to_check.is_empty()
        && to_check
            .chars()
            .all(|c| c.is_ascii_digit() || c == '.' || c == ',')
}

/// Check if input contains a binary operator.
fn has_operator(input: &str) -> bool {
    // Check for +, *, /, ^, %
    if input.contains('+')
        || input.contains('*')
        || input.contains('/')
        || input.contains('^')
        || input.contains('%')
    {
        return true;
    }

    // Check for minus that's not just a negative sign at the start
    let chars: Vec<char> = input.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        if c == '-' && i > 0 {
            // Look back past any whitespace to find the previous non-space character
            let prev_non_space = chars[..i]
                .iter()
                .rev()
                .find(|&&ch| !ch.is_whitespace())
                .copied();

            // It's a binary minus if preceded by a digit, closing paren, or letter
            if let Some(prev) = prev_non_space
                && (prev.is_ascii_digit() || prev == ')' || prev.is_ascii_alphabetic())
            {
                return true;
            }
        }
    }

    false
}

/// Check if input contains a known math function.
fn has_function(input: &str) -> bool {
    let lower = input.to_lowercase();
    MATH_FUNCTIONS.iter().any(|&func| {
        lower.contains(&format!("{}(", func)) || lower.contains(&format!("{} (", func))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_numbers_rejected() {
        assert!(!looks_like_expression("123"));
        assert!(!looks_like_expression("42.5"));
        assert!(!looks_like_expression("-123"));
        assert!(!looks_like_expression("1,234,567"));
        assert!(!looks_like_expression("  42  "));
    }

    #[test]
    fn test_expressions_accepted() {
        assert!(looks_like_expression("2+2"));
        assert!(looks_like_expression("2 + 2"));
        assert!(looks_like_expression("10 * 5"));
        assert!(looks_like_expression("100 / 4"));
        assert!(looks_like_expression("2^8"));
        assert!(looks_like_expression("10 % 3"));
        assert!(looks_like_expression("(2 + 3) * 4"));
        assert!(looks_like_expression("10 - 5"));
    }

    #[test]
    fn test_functions_accepted() {
        assert!(looks_like_expression("sin(0)"));
        assert!(looks_like_expression("cos(pi())"));
        assert!(looks_like_expression("16^0.5")); // sqrt via exponentiation
        assert!(looks_like_expression("abs(-5)"));
        assert!(looks_like_expression("log(10, 100)"));
    }

    #[test]
    fn test_invalid_input_rejected() {
        assert!(!looks_like_expression(""));
        assert!(!looks_like_expression("a"));
        assert!(!looks_like_expression("hello world"));
        assert!(!looks_like_expression("firefox"));
    }
}
