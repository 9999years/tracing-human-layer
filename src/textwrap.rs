//! Extensions and utilities for the [`textwrap`] crate.

use std::borrow::Cow;

use textwrap::Options;
use textwrap::WordSeparator;
use textwrap::WordSplitter;

/// Get [`textwrap`] options with our settings.
pub fn options<'a>() -> Options<'a> {
    let opts = Options::with_termwidth()
        .break_words(false)
        .word_separator(WordSeparator::AsciiSpace)
        .word_splitter(WordSplitter::NoHyphenation);

    // In tests, the terminal is always 80 characters wide.
    if cfg!(test) {
        opts.with_width(80)
    } else {
        opts
    }
}

/// Extension trait adding methods to [`textwrap::Options`]
pub trait TextWrapOptionsExt {
    /// Set the `width` to wrap the text to.
    fn with_width(self, width: usize) -> Self;

    /// Wrap the given text into lines.
    fn wrap<'s>(&self, text: &'s str) -> Vec<Cow<'s, str>>;
}

impl<'a> TextWrapOptionsExt for Options<'a> {
    fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    fn wrap<'s>(&self, text: &'s str) -> Vec<Cow<'s, str>> {
        textwrap::wrap(text, self)
    }
}
