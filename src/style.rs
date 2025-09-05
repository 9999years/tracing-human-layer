use std::borrow::Cow;
use std::fmt::Display;

use owo_colors::Style as OwoStyle;
use tracing::Level;
use tracing::Metadata;

use crate::ShouldColor;

#[cfg(doc)]
use crate::HumanLayer;

/// A value that can provide a [`Style`] for a given [`tracing`] event.
pub trait ProvideStyle {
    /// Provide a [`Style`] for a given set of [`tracing`] metadata.
    ///
    /// In the future we may want to give implementers access to more information, but all events
    /// contain metadata, so we can provide default implementations for future methods.
    fn for_metadata(&self, metadata: &'static Metadata<'static>) -> Cow<'_, Style>;
}

/// A simple [`ProvideStyle`] implementation which stores a style for each [`tracing::Level`].
#[derive(Debug)]
pub struct LayerStyles {
    /// Style for the [`Level::TRACE`] log level.
    pub trace: Style,
    /// Style for the [`Level::DEBUG`] log level.
    pub debug: Style,
    /// Style for the [`Level::INFO`] log level.
    pub info: Style,
    /// Style for the [`Level::WARN`] log level.
    pub warn: Style,
    /// Style for the [`Level::ERROR`] log level.
    pub error: Style,
}

impl LayerStyles {
    /// Create the default styles.
    pub fn new() -> Self {
        let base = Style {
            initial_indent_text: "".into(),
            subsequent_indent_text: "  ".into(),
            initial_indent: OwoStyle::new(),
            message: OwoStyle::new(),
            field_name: OwoStyle::new().bold(),
            field_value: OwoStyle::new(),
            span_name: OwoStyle::new(),
            span_in: OwoStyle::new().dimmed(),
        };

        Self {
            trace: Style {
                initial_indent_text: "TRACE ".into(),
                initial_indent: base.initial_indent.purple(),
                message: base.message.dimmed(),
                field_name: base.field_name.dimmed(),
                field_value: base.field_value.dimmed(),
                span_name: base.span_name.dimmed(),
                ..base.clone()
            },

            debug: Style {
                initial_indent_text: "DEBUG ".into(),
                initial_indent: base.initial_indent.blue(),
                message: base.message.dimmed(),
                field_name: base.field_name.dimmed(),
                field_value: base.field_value.dimmed(),
                span_name: base.span_name.dimmed(),
                ..base.clone()
            },

            info: Style {
                initial_indent_text: "• ".into(),
                initial_indent: base.initial_indent.green(),
                ..base.clone()
            },

            warn: Style {
                initial_indent_text: "⚠ ".into(),
                initial_indent: base.initial_indent.yellow(),
                message: base.message.yellow(),
                ..base.clone()
            },

            error: Style {
                initial_indent_text: "⚠ ".into(),
                initial_indent: base.initial_indent.red(),
                message: base.message.red(),
                ..base
            },
        }
    }

    /// Get the style for a given level.
    pub(crate) fn for_level(&self, level: Level) -> Cow<'_, Style> {
        Cow::Borrowed(match level {
            Level::TRACE => &self.trace,
            Level::DEBUG => &self.debug,
            Level::INFO => &self.info,
            Level::WARN => &self.warn,
            Level::ERROR => &self.error,
        })
    }
}

impl Default for LayerStyles {
    fn default() -> Self {
        Self::new()
    }
}

impl ProvideStyle for LayerStyles {
    fn for_metadata(&self, metadata: &'static Metadata<'static>) -> Cow<'_, Style> {
        self.for_level(*metadata.level())
    }
}

/// The style for formatting a [`tracing`] event.
///
/// A [`HumanLayer`] retrieves styles through a [`ProvideStyle`] implementation.
///
/// TODO: It should be possible to configure which spans and attributes are printed.
#[derive(Debug, Clone)]
pub struct Style {
    pub(crate) initial_indent_text: Cow<'static, str>,
    pub(crate) subsequent_indent_text: Cow<'static, str>,
    pub(crate) initial_indent: OwoStyle,
    pub(crate) message: OwoStyle,
    pub(crate) field_name: OwoStyle,
    pub(crate) field_value: OwoStyle,
    pub(crate) span_name: OwoStyle,
    pub(crate) span_in: OwoStyle,
}

impl Style {
    pub(crate) fn style_field<'a>(
        &'a self,
        color: ShouldColor,
        name: &'a str,
        value: &'a str,
    ) -> StyledField<'a> {
        StyledField {
            color,
            name,
            name_style: self.field_name,
            value,
            value_style: self.field_value,
        }
    }

    /// First-line indent text.
    pub fn with_initial_indent_text(mut self, initial_indent_text: Cow<'static, str>) -> Self {
        self.initial_indent_text = initial_indent_text;
        self
    }

    /// Subsequent indent text.
    pub fn with_subsequent_indent_text(
        mut self,
        subsequent_indent_text: Cow<'static, str>,
    ) -> Self {
        self.subsequent_indent_text = subsequent_indent_text;
        self
    }

    /// Style for first-line indent text.
    pub fn with_initial_indent(mut self, initial_indent: OwoStyle) -> Self {
        self.initial_indent = initial_indent;
        self
    }

    /// Style for message text.
    pub fn with_message(mut self, message: OwoStyle) -> Self {
        self.message = message;
        self
    }

    /// Style for field names.
    pub fn with_field_name(mut self, field_name: OwoStyle) -> Self {
        self.field_name = field_name;
        self
    }

    /// Style for field values.
    pub fn with_field_value(mut self, field_value: OwoStyle) -> Self {
        self.field_value = field_value;
        self
    }

    /// Style for span names.
    pub fn with_span_name(mut self, span_name: OwoStyle) -> Self {
        self.span_name = span_name;
        self
    }

    /// Style for the word `in` when writing that an event is `in span such_and_such`.
    ///
    /// The name is a bit clumsy, but it matches [`Style::with_span_name`]...
    pub fn with_span_in(mut self, span_in: OwoStyle) -> Self {
        self.span_in = span_in;
        self
    }
}

pub(crate) trait IntoConditionalColor: Display {
    fn colored(&self, color: ShouldColor, style: OwoStyle) -> ConditionalColor<&Self> {
        ConditionalColor {
            inner: self,
            style,
            color,
        }
    }
}

impl<T> IntoConditionalColor for T where T: Display {}

/// Like `if_supports_color`, but I control it :)
pub(crate) struct ConditionalColor<T> {
    color: ShouldColor,
    style: OwoStyle,
    inner: T,
}

impl<T> Display for ConditionalColor<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            ShouldColor::Always => self.style.style(&self.inner).fmt(f),
            ShouldColor::Never => self.inner.fmt(f),
        }
    }
}

pub(crate) struct StyledField<'a> {
    color: ShouldColor,
    name: &'a str,
    name_style: OwoStyle,
    value: &'a str,
    value_style: OwoStyle,
}

impl Display for StyledField<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.colored(self.color, self.name_style))?;
        write!(f, "{}", '='.colored(self.color, self.value_style))?;
        write!(f, "{}", self.value.colored(self.color, self.value_style))?;
        Ok(())
    }
}
