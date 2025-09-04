//! A human-friendly and colorful terminal output [`tracing_subscriber::Layer`] for [`tracing`].

#![deny(missing_docs)]

pub use layer::HumanLayer;
pub use style::LayerStyles;
pub use style::ProvideStyle;
pub use style::Style;
pub use textwrap::options as textwrap_options;

pub(crate) use color::ShouldColor;
pub(crate) use event::HumanEvent;
pub(crate) use fields::HumanFields;
pub(crate) use span_fields::StyledSpanFields;
pub(crate) use span_info::SpanInfo;

mod color;
mod event;
mod fields;
mod layer;
mod span_fields;
mod span_info;
mod style;
mod textwrap;
