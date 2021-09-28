/// Formats an error to be displayed to a user. This will include a series of indented sources. If supplying a full source chain is
/// not desired, `err.to_string()` should be used instead (and you can use only the `def` eature of this crate if you're using that,
/// otherwise you probably don't need this crate at all).
pub fn fmt_err(err: &impl std::error::Error) -> String {
    // Used for functional recursion on the error source chain
    // For the given error, this will return its entire source chain
    fn format_err_internal(err: &impl std::error::Error) -> String {
        let mut msg = err.to_string();
        if let Some(source) = err.source() {
            let source_msg = format_err_internal(&source);
            // We want this to be indented
            let indented = source_msg.replace("\n", "\n\t");

            // This will end up like so:
            // error message
            // Caused by:
            //      source error message
            msg = format!("{}\nCaused by:\n\t{}", msg, indented);
        }

        msg
    }

    let err_msg = format_err_internal(err);
    format!(
        "Error: {}",
        // Add another newline between the top-level error and its source chain
        err_msg.replacen("\nCaused by:", "\n\nCaused by:", 1)
    )
}
