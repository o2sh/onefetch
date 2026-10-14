//! The only code that writes escape sequences or talks to the tty.
//!
//! The rest of `render` produces styled text (`text::Line`), which
//! `ansi::paint` writes out. Every piece of text goes through `sanitize` on
//! its way, so that repository data can't inject escape sequences.

pub(crate) mod ansi;
pub mod image;
mod sanitize;

use std::env;

pub fn supports_true_color() -> bool {
    env::var("COLORTERM")
        .map(|colorterm| colorterm == "truecolor" || colorterm == "24bit")
        .unwrap_or(false)
}
