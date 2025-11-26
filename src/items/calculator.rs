//! Calculator item representing a calculation result.

use crate::calculator::CalcResult;

/// A calculator item representing a calculation result.
#[derive(Clone, Debug)]
pub struct CalculatorItem {
    /// Unique identifier for this item.
    pub id: String,
    /// The original expression entered by the user.
    pub expression: String,
    /// The result formatted for display (with thousand separators).
    pub display_result: String,
    /// The result formatted for clipboard (raw number).
    /// None if the result is an error (NaN, Infinity).
    pub clipboard_result: Option<String>,
    /// Whether this is an error result.
    pub is_error: bool,
}

impl CalculatorItem {
    /// Create a new calculator item from a CalcResult.
    pub fn from_calc_result(result: CalcResult) -> Self {
        match result {
            CalcResult::Success {
                expression,
                display_result,
                clipboard_result,
                ..
            } => Self {
                id: "calculator-result".to_string(),
                expression,
                display_result,
                clipboard_result: Some(clipboard_result),
                is_error: false,
            },
            CalcResult::Error {
                expression,
                message,
            } => Self {
                id: "calculator-result".to_string(),
                expression,
                display_result: message,
                clipboard_result: None,
                is_error: true,
            },
        }
    }

    /// Get the text to copy to clipboard.
    /// Returns the clipboard result for successful calculations,
    /// or the display result for errors (so user can still copy the error message).
    pub fn text_for_clipboard(&self) -> &str {
        self.clipboard_result
            .as_deref()
            .unwrap_or(&self.display_result)
    }
}
