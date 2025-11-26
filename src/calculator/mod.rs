//! Calculator module for evaluating mathematical expressions.
//!
//! This module provides functionality to:
//! - Detect if user input looks like a calculator expression
//! - Evaluate expressions using fasteval
//! - Copy results to the clipboard

mod clipboard;
mod detection;
mod evaluation;

pub use clipboard::copy_to_clipboard;
pub use detection::looks_like_expression;
pub use evaluation::{CalcResult, evaluate_expression};
