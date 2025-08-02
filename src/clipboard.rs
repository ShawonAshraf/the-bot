// src/clipboard.rs

// We use the "clipboard" crate to interact with the system clipboard.
// This crate provides a simple and cross-platform API.
// Make sure to add `clipboard = "0.5.0"` to your [dependencies] in Cargo.toml.
extern crate clipboard;

use clipboard::{ClipboardContext, ClipboardProvider};
use std::error::Error;
use tracing::{info, warn, debug};

/// Copies the given text to the system clipboard.
///
/// This function initializes a clipboard context and then uses it to set
/// the clipboard's contents to the provided string slice.
///
/// # Arguments
///
/// * `text` - A string slice that will be copied to the clipboard.
///
/// # Returns
///
/// * `Ok(())` if the text was copied successfully.
/// * `Err(Box<dyn Error>)` if there was an error interacting with the clipboard.
///
/// # Example
///
/// ```
/// use crate::clipboard::copy_to_clipboard;
/// use tracing::{info, error};
///
/// match copy_to_clipboard("hello world") {
///     Ok(_) => info!("Copied to clipboard!"),
///     Err(e) => error!(error = %e, "Failed to copy to clipboard"),
/// }
/// ```
pub fn copy_to_clipboard(text: &str) -> Result<(), Box<dyn Error>> {
    debug!(text_length = text.len(), "Attempting to copy text to clipboard");
    
    // Create a new clipboard context. This is the entry point to using the clipboard.
    let mut ctx: ClipboardContext = ClipboardProvider::new()
        .map_err(|e| {
            warn!(error = ?e, "Failed to create clipboard context");
            e
        })?;

    // Set the contents of the clipboard to the provided text.
    // The `set_contents` method takes ownership of a String, so we convert `text`.
    ctx.set_contents(text.to_owned())
        .map_err(|e| {
            warn!(
                error = ?e,
                text_length = text.len(),
                "Failed to set clipboard contents"
            );
            e
        })?;

    // If both operations succeed, return Ok.
    info!(text_length = text.len(), "Successfully copied text to clipboard");
    Ok(())
}

// Unit tests for the clipboard module.
#[cfg(test)]
mod tests {
    use super::*;
    use clipboard::{ClipboardContext, ClipboardProvider};

    /// Tests the core functionality of copying to and reading from the clipboard.
    ///
    /// This test first copies a predefined string to the clipboard using the
    /// `copy_to_clipboard` function. It then creates its own clipboard context
    /// to read the content back and asserts that the pasted content matches
    /// the original string.
    #[test]
    fn test_copy_and_paste() {
        // skip in github action runners
        if std::env::var("CI").is_ok() {
            // Skip clipboard test in CI environments
            warn!("Skipping clipboard test in CI environment");
            return;
        }
        // 1. Define the test string we want to copy.
        let test_string = "Hello, Rust Clipboard!";

        // 2. Call our function to copy the string to the clipboard.
        //    We use `unwrap()` here because in a test environment, we expect this
        //    to succeed. If it fails, the test should panic.
        assert!(copy_to_clipboard(test_string).is_ok());

        // 3. Create a new clipboard context to verify the content.
        let mut ctx: ClipboardContext =
            ClipboardProvider::new().expect("Failed to create clipboard context for verification.");

        // 4. Read the content back from the clipboard.
        let pasted_content = ctx
            .get_contents()
            .expect("Failed to get contents from clipboard.");

        // 5. Assert that the content we read back is the same as what we copied.
        assert_eq!(test_string, pasted_content);
    }
}
