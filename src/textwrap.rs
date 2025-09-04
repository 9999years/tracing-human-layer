//! Extensions and utilities for the [`textwrap`] crate.

use std::borrow::Cow;

use textwrap::LineEnding;
use textwrap::Options;
use textwrap::WordSeparator;
use textwrap::WordSplitter;
use textwrap::WrapAlgorithm;

#[cfg(doc)]
use crate::HumanLayer;
#[cfg(doc)]
use crate::ProvideStyle;

/// The width to wrap text at.
#[derive(Debug, Clone, Copy)]
enum TextWrapWidth {
    /// Wrap text at the width of the terminal, or 80 columns by default.
    TerminalWidth,
    /// Wrap text at a given fixed width.
    Fixed(usize),
}

/// Options for wrapping and filling text. Like [`textwrap::Options`], but owned.
///
/// We want to vary the [`textwrap::Options::initial_indent`] and
/// [`textwrap::Options::subsequent_indent`] depending on the log level, so those fields are
/// set in a [`HumanLayer`]'s [`ProvideStyle`] implementation instead.
#[derive(Debug, Clone)]
pub struct TextWrapOptionsOwned {
    /// The width in columns at which the text will be wrapped.
    width: TextWrapWidth,
    /// Line ending used for breaking lines.
    line_ending: LineEnding,
    /// Allow long words to be broken if they cannot fit on a line.
    /// When set to `false`, some lines may be longer than
    /// `self.width`. See the [`Options::break_words`] method.
    break_words: bool,
    /// Wrapping algorithm to use.
    wrap_algorithm: WrapAlgorithm,
    /// The line breaking algorithm to use.
    word_separator: WordSeparator,
    /// The method for splitting words. This can be used to prohibit
    /// splitting words on hyphens, or it can be used to implement
    /// language-aware machine hyphenation.
    word_splitter: WordSplitter,
}

impl TextWrapOptionsOwned {
    /// Construct a new [`TextWrapOptionsOwned`]. This differs from [`textwrap::Options::new`]
    /// in the following ways:
    ///
    /// - The `width` defaults to the terminal's width (except in tests, where the width is
    ///   always 80 columns).
    /// - The `word_separator` is set to [`WordSeparator::AsciiSpace`].
    /// - The `word_splitter` is set to [`WordSplitter::NoHyphenation`].
    pub fn new() -> Self {
        Self {
            // In tests, the terminal is always 80 characters wide.
            width: if cfg!(test) {
                TextWrapWidth::Fixed(80)
            } else {
                TextWrapWidth::TerminalWidth
            },
            line_ending: LineEnding::LF,
            break_words: false,
            wrap_algorithm: WrapAlgorithm::new(),
            word_separator: WordSeparator::AsciiSpace,
            word_splitter: WordSplitter::NoHyphenation,
        }
    }

    /// Use a given fixed width. This corresponds to [`textwrap::Options::new`].
    pub fn with_width(self, width: usize) -> Self {
        Self {
            width: TextWrapWidth::Fixed(width),
            ..self
        }
    }

    /// Use the width of the terminal, or 80 columns by default. This corresponds to
    /// [`textwrap::Options::with_termwidth`]. Note that the terminal width is queried lazily,
    /// as `tracing` records are formatted.
    pub fn with_termwidth(self) -> Self {
        Self {
            width: TextWrapWidth::TerminalWidth,
            ..self
        }
    }

    /// Corresponds to [`textwrap::Options::line_ending`].
    pub fn with_line_ending(self, line_ending: LineEnding) -> Self {
        Self {
            line_ending,
            ..self
        }
    }

    /// Corresponds to [`textwrap::Options::break_words`].
    pub fn with_break_words(self, break_words: bool) -> Self {
        Self {
            break_words,
            ..self
        }
    }

    /// Corresponds to [`textwrap::Options::wrap_algorithm`].
    pub fn with_wrap_algorithm(self, wrap_algorithm: WrapAlgorithm) -> Self {
        Self {
            wrap_algorithm,
            ..self
        }
    }

    /// Corresponds to [`textwrap::Options::word_separator`].
    pub fn with_word_separator(self, word_separator: WordSeparator) -> Self {
        Self {
            word_separator,
            ..self
        }
    }

    /// Corresponds to [`textwrap::Options::word_splitter`].
    pub fn with_word_splitter(self, word_splitter: WordSplitter) -> Self {
        Self {
            word_splitter,
            ..self
        }
    }
}

impl Default for TextWrapOptionsOwned {
    fn default() -> Self {
        Self::new()
    }
}

/// Note that this leaves the [`textwrap::Options::initial_indent`] and
/// [`textwrap::Options::subsequent_indent`] fields empty.
impl<'a> From<&'_ TextWrapOptionsOwned> for Options<'a> {
    fn from(opts: &'_ TextWrapOptionsOwned) -> Self {
        match opts.width {
            TextWrapWidth::TerminalWidth => Options::with_termwidth(),
            TextWrapWidth::Fixed(width) => Options::new(width),
        }
        .line_ending(opts.line_ending)
        .break_words(opts.break_words)
        .wrap_algorithm(opts.wrap_algorithm)
        .word_separator(opts.word_separator)
        .word_splitter(opts.word_splitter.clone())
    }
}

/// Extension trait adding methods to [`textwrap::Options`]
pub(crate) trait TextWrapOptionsExt {
    /// Wrap the given text into lines.
    fn wrap<'s>(&self, text: &'s str) -> Vec<Cow<'s, str>>;
}

impl<'a> TextWrapOptionsExt for Options<'a> {
    fn wrap<'s>(&self, text: &'s str) -> Vec<Cow<'s, str>> {
        textwrap::wrap(text, self)
    }
}

/// A trivial implementation which does nothing when the [`Option`] is [`None`].
impl<'a> TextWrapOptionsExt for Option<Options<'a>> {
    fn wrap<'s>(&self, text: &'s str) -> Vec<Cow<'s, str>> {
        match self {
            Some(options) => textwrap::wrap(text, options),
            None => {
                vec![Cow::Borrowed(text)]
            }
        }
    }
}
