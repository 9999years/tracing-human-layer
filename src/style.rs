use std::fmt::Display;

use owo_colors::Style as OwoStyle;
use tracing::Level;

use super::SpanInfo;

#[derive(Debug)]
pub struct Style {
    /// First-line indent text.
    indent_text: &'static str,

    /// Subsequent indent text.
    pub subsequent_indent: &'static str,

    /// Should output be colored?
    color: bool,

    /// Style for first-line indent text.
    indent: OwoStyle,

    /// Style for message text.
    text: OwoStyle,

    /// Style for field names.
    field_name: OwoStyle,

    /// Style for field values.
    field_value: OwoStyle,

    /// Style for span names.
    span_name: OwoStyle,

    /// Style for `in` in `in span ...`
    span_in: OwoStyle,
}

impl Style {
    pub fn new(level: Level, color: bool) -> Self {
        let indent_text;
        let span_in = OwoStyle::new().dimmed();
        let mut indent = OwoStyle::new();
        let mut text = OwoStyle::new();
        let mut field_name = OwoStyle::new().bold();
        let mut field_value = OwoStyle::new();
        let mut span_name = OwoStyle::new();

        match level {
            Level::TRACE => {
                indent_text = "TRACE ";
                indent = indent.purple();
                text = text.dimmed();
                field_name = field_name.dimmed();
                field_value = field_value.dimmed();
                span_name = span_name.dimmed();
            }
            Level::DEBUG => {
                indent_text = "DEBUG ";
                indent = indent.blue();
                text = text.dimmed();
                field_name = field_name.dimmed();
                field_value = field_value.dimmed();
                span_name = span_name.dimmed();
            }
            Level::INFO => {
                indent_text = "• ";
                indent = indent.green();
            }
            Level::WARN => {
                indent_text = "⚠ ";
                indent = indent.yellow();
                text = text.yellow();
            }
            Level::ERROR => {
                indent_text = "⚠ ";
                indent = indent.red();
                text = text.red();
            }
        }

        Self {
            indent_text,
            subsequent_indent: "  ",
            color,
            indent,
            text,
            field_name,
            field_value,
            span_name,
            span_in,
        }
    }

    pub fn style_field(&self, name: &str, value: &str) -> String {
        format!(
            "{name}{value}",
            name = name.colored(self.color, self.field_name),
            value = format!("={value}").colored(self.color, self.field_value),
        )
    }

    pub fn indent_colored(&self) -> String {
        self.indent_text
            .colored(self.color, self.indent)
            .to_string()
    }

    pub fn style_message(&self, message: &str) -> String {
        message.colored(self.color, self.text).to_string()
    }

    pub fn style_span_name(&self, name: &str) -> String {
        name.colored(self.color, self.span_name).to_string()
    }

    pub fn style_span(&self, span: &SpanInfo) -> String {
        format!(
            "{in_}{name}{fields}",
            in_ = "in ".colored(self.color, self.span_in),
            name = span.name.colored(self.color, self.span_name),
            fields = span.fields,
        )
    }
}

trait IntoConditionalColor: Display {
    fn colored(&self, color: bool, style: OwoStyle) -> ConditionalColor<&Self> {
        ConditionalColor::new(self).color(color).style(style)
    }
}

impl<T> IntoConditionalColor for T where T: Display {}

/// Like `if_supports_color`, but I control it :)
struct ConditionalColor<T> {
    color: bool,
    style: OwoStyle,
    inner: T,
}

impl<T> ConditionalColor<T> {
    pub fn new(inner: T) -> Self {
        Self {
            color: true,
            style: OwoStyle::new(),
            inner,
        }
    }

    pub fn color(mut self, color: bool) -> Self {
        self.color = color;
        self
    }

    pub fn style(mut self, style: OwoStyle) -> Self {
        self.style = style;
        self
    }
}

impl<T> Display for ConditionalColor<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.color {
            self.style.style(&self.inner).fmt(f)
        } else {
            self.inner.fmt(f)
        }
    }
}
