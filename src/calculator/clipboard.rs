//! Clipboard functionality for copying calculator results.

use arboard::Clipboard;

/// Copy text to the system clipboard.
///
/// Returns `Ok(())` on success, or an error message on failure.
pub fn copy_to_clipboard(text: &str) -> Result<(), String> {
    let mut clipboard =
        Clipboard::new().map_err(|e| format!("Failed to access clipboard: {}", e))?;

    clipboard
        .set_text(text.to_string())
        .map_err(|e| format!("Failed to copy to clipboard: {}", e))
}
