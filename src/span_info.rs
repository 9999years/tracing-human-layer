use tracing_subscriber::fmt::FormattedFields;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::registry::Scope;

use crate::HumanLayer;

#[derive(Debug)]
pub struct SpanInfo {
    /// The span's name.
    pub name: &'static str,
    /// The span's target (typically the module name).
    #[allow(dead_code)]
    target: String,
    /// The span's fields, formatted.
    pub fields: String,
}

impl SpanInfo {
    /// Get a list of `SpanInfo`s from a [`Scope`] by traversing its spans from root to leaf
    /// (outside-in).
    ///
    /// This relies on the [`super::HumanLayer`] to insert formatted fields in the span's
    /// extensions.
    pub fn from_scope<S>(scope: Scope<'_, S>) -> Vec<Self>
    where
        S: tracing::Subscriber,
        S: for<'lookup> LookupSpan<'lookup>,
    {
        let mut spans = Vec::new();
        for span in scope.from_root() {
            let extensions = span.extensions();
            let fields = &extensions
                .get::<FormattedFields<HumanLayer>>()
                .expect("A span should always have formatted fields")
                .fields;
            spans.push(SpanInfo {
                name: span.name(),
                target: span.metadata().target().into(),
                fields: fields.to_owned(),
            });
        }
        spans
    }
}
