// src/clipboard.rs

// We use the "arboard" crate to interact with the system clipboard.
// This crate provides a simple and cross-platform API with better Wayland support.
// Make sure to add `arboard = "3.6.1"` to your [dependencies] in Cargo.toml.
use arboard::Clipboard;
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
    
    // Create a new clipboard instance. This is the entry point to using the clipboard.
    let mut clipboard = Clipboard::new()
        .map_err(|e| {
            warn!(error = ?e, "Failed to create clipboard instance");
            e
        })?;

    // Set the contents of the clipboard to the provided text.
    clipboard.set_text(text)
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
