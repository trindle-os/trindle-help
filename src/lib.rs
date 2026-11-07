//! The part of Help that Trindle's own tests read: where the pages are, and which section a failure's words lead to.

#[path = "help/clean.rs"]
pub mod clean;
#[path = "help/errors.rs"]
pub mod errors;

/// The folder of the pages, one markdown file for each page id.
pub const CONTENT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/site/content");

/// What `trindle-help --classify` prints for `text`: the section its words lead to, or None.
pub fn classify(text: &str) -> Option<&'static str> {
    errors::classify(&clean::error(text))
}
