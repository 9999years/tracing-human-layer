//! [tracing_subscriber::Layer]: tracing_subscriber::Layer
//! [tracing]: tracing
//! [textwrap-options]: HumanLayer::with_textwrap_options
//! [styles]: HumanLayer::with_style_provider
//! [disable-color]: HumanLayer::with_color_output
#![doc = include_str!("../README.md")]
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
