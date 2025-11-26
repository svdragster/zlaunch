//! Expression evaluation using fasteval.
//!
//! Wraps fasteval to provide a simple interface for evaluating
//! mathematical expressions and formatting results.

use std::collections::BTreeMap;

/// Result of evaluating a calculator expression.
#[derive(Clone, Debug)]
pub enum CalcResult {
    /// Successful calculation with a valid numeric result.
    Success {
        /// The original expression.
        expression: String,
        /// The numeric value.
        value: f64,
        /// Formatted for display (with thousand separators).
        display_result: String,
        /// Formatted for clipboard (raw number).
        clipboard_result: String,
    },
    /// Expression evaluated but result is not a valid number.
    Error {
        /// The original expression.
        expression: String,
        /// Error message to display.
        message: String,
    },
}

impl CalcResult {
    /// Get the expression that was evaluated.
    pub fn expression(&self) -> &str {
        match self {
            Self::Success { expression, .. } => expression,
            Self::Error { expression, .. } => expression,
        }
    }

    /// Check if this is a successful result.
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success { .. })
    }

    /// Get the display string (result or error message).
    pub fn display(&self) -> &str {
        match self {
            Self::Success { display_result, .. } => display_result,
            Self::Error { message, .. } => message,
        }
    }

    /// Get the clipboard string (only for successful results).
    pub fn clipboard(&self) -> Option<&str> {
        match self {
            Self::Success {
                clipboard_result, ..
            } => Some(clipboard_result),
            Self::Error { .. } => None,
        }
    }
}

/// Evaluate a mathematical expression.
///
/// Returns `Some(CalcResult)` if the expression can be parsed,
/// or `None` if parsing fails entirely.
pub fn evaluate_expression(input: &str) -> Option<CalcResult> {
    let expression = input.trim().to_string();

    // Use an empty namespace (no custom variables)
    let mut namespace = BTreeMap::<String, f64>::new();

    match fasteval::ez_eval(&expression, &mut namespace) {
        Ok(value) => {
            if value.is_nan() {
                Some(CalcResult::Error {
                    expression,
                    message: "Not a Number".to_string(),
                })
            } else if value.is_infinite() {
                let msg = if value.is_sign_positive() {
                    "Infinity"
                } else {
                    "-Infinity"
                };
                Some(CalcResult::Error {
                    expression,
                    message: msg.to_string(),
                })
            } else {
                Some(CalcResult::Success {
                    expression,
                    display_result: format_display(value),
                    clipboard_result: format_clipboard(value),
                    value,
                })
            }
        }
        Err(_) => None, // Parse error - silently fail
    }
}

/// Format a number for display with thousand separators.
fn format_display(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e15 {
        // Integer display with thousand separators
        format_with_separators(value as i64)
    } else {
        // Decimal display
        let formatted = format!("{:.10}", value);
        let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');

        // Add thousand separators to the integer part
        if let Some(dot_pos) = trimmed.find('.') {
            let (int_part, dec_part) = trimmed.split_at(dot_pos);
            let int_val: i64 = int_part.parse().unwrap_or(0);
            format!("{}{}", format_with_separators(int_val), dec_part)
        } else {
            let int_val: i64 = trimmed.parse().unwrap_or(0);
            format_with_separators(int_val)
        }
    }
}

/// Format an integer with thousand separators.
fn format_with_separators(value: i64) -> String {
    let is_negative = value < 0;
    let abs_value = value.abs();
    let s = abs_value.to_string();

    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }

    let formatted: String = result.chars().rev().collect();
    if is_negative {
        format!("-{}", formatted)
    } else {
        formatted
    }
}

/// Format a number for clipboard (raw number, no separators).
fn format_clipboard(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e15 {
        format!("{}", value as i64)
    } else {
        let formatted = format!("{:.10}", value);
        formatted
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_evaluation() {
        let result = evaluate_expression("2 + 2").unwrap();
        assert!(result.is_success());
        assert_eq!(result.display(), "4");
        assert_eq!(result.clipboard(), Some("4"));
    }

    #[test]
    fn test_thousand_separators() {
        let result = evaluate_expression("1000 * 1000").unwrap();
        assert!(result.is_success());
        assert_eq!(result.display(), "1,000,000");
        assert_eq!(result.clipboard(), Some("1000000"));
    }

    #[test]
    fn test_decimal_result() {
        let result = evaluate_expression("1 / 3").unwrap();
        assert!(result.is_success());
        // Should have decimal places, no trailing zeros
        assert!(result.display().starts_with("0.333"));
    }

    #[test]
    fn test_division_by_zero() {
        let result = evaluate_expression("1 / 0").unwrap();
        assert!(!result.is_success());
        assert_eq!(result.display(), "Infinity");
    }

    #[test]
    fn test_invalid_expression() {
        // Truly invalid expressions that fasteval cannot parse
        let result = evaluate_expression("2 +* 2");
        assert!(result.is_none());
    }

    #[test]
    fn test_functions() {
        // Use exponentiation for square root since sqrt is not built-in
        let result = evaluate_expression("16^0.5").unwrap();
        assert!(result.is_success());
        assert_eq!(result.display(), "4");
    }

    #[test]
    fn test_trig_functions() {
        let result = evaluate_expression("sin(0)").unwrap();
        assert!(result.is_success());
        assert_eq!(result.display(), "0");
    }
}
