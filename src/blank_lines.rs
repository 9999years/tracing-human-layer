#[cfg(doc)]
use crate::HumanLayer;

/// When to emit blank lines between log messages.
///
/// [`HumanLayer`] supports printing blank lines between log messages long enough to be
/// line-wrapped. This enum determines when blank lines are printed.
#[derive(Debug, Clone, Copy)]
pub enum BlankLines {
    /// Blank lines are never printed between log messages.
    Never,

    /// A blank line is printed before and after a "long" log message.
    ///
    /// If two "long" log messages are printed in a row, only one blank line will be printed
    /// between them.
    ///
    /// Currently, a log message is considered "long" if it causes line-wrapping. In the future,
    /// other characeristics may be taken into account (for example, if the message has multiple
    /// lines of fields and spans attached).
    AroundLong,

    /// A blank line is always printed between log messages.
    Always,
}
