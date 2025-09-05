//! A human-friendly and colorful terminal output [`tracing_subscriber::Layer`] for [`tracing`].
//!
//! ## Performance
//!
//! TL;DR: Logging performance is dominated by the cost of writing to stderr.
//!
//! I haven't done too much performance work on `tracing-human-layer`, but I do have a couple
//! benchmarks. It seems to take 2-6µs to format an event (including emitting a span and event),
//! with the exact cost depending on whether or not color output
//! ([`HumanLayer::with_color_output`]) or text wrapping ([`HumanLayer::with_textwrap_options`])
//! is enabled.
//!
//! Formatting an event _and writing it to stderr_ takes 20µs, so actually showing the logs to the
//! user is about 3.5× slower than just formatting them.

#![deny(missing_docs)]

pub use layer::HumanLayer;
pub use style::LayerStyles;
pub use style::ProvideStyle;
pub use style::Style;
pub use textwrap::TextWrapOptionsOwned;

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
