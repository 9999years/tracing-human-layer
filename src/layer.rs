use std::fmt::Debug;
use std::io::Stderr;
use std::io::Write;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use parking_lot::Mutex;
use tracing::span;
use tracing::Event;
use tracing::Id;
use tracing::Metadata;
use tracing::Subscriber;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::FormattedFields;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::registry::Scope;
use tracing_subscriber::Layer;

use crate::HumanEvent;
use crate::HumanFields;
use crate::LayerStyles;
use crate::ProvideStyle;
use crate::ShouldColor;
use crate::SpanInfo;
use crate::StyledSpanFields;
use crate::TextWrapOptionsOwned;

#[cfg(doc)]
use crate::Style;

/// A human-friendly [`tracing_subscriber::Layer`].
pub struct HumanLayer<W = Stderr, S = LayerStyles> {
    /// We print blank lines before and after long log messages to help visually separate them.
    ///
    /// This becomes an issue if two long log messages are printed one after another.
    ///
    /// If this variable is `true`, we skip the blank line before to prevent printing two blank
    /// lines in a row.
    ///
    /// This variable is mutated whenever a [`HumanEvent`] is displayed.
    last_event_was_long: AtomicBool,
    /// Which span events to emit.
    span_events: FmtSpan,
    /// Whether to color the output.
    color_output: ShouldColor,
    /// Options for wrapping text, if any.
    textwrap_options: Option<TextWrapOptionsOwned>,
    /// The writer where output is written.
    output_writer: Mutex<W>,
    /// Styles for writing events.
    styles: S,
}

impl<W, S> Debug for HumanLayer<W, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HumanLayer")
            .field("span_events", &self.span_events)
            .field("color_output", &self.color_output)
            .field("textwrap_options", &self.textwrap_options)
            // These strings get debug-formatted, which is a bit ugly, but it's fine.
            // See: https://github.com/rust-lang/rust/issues/117729
            .field("output_writer", &std::any::type_name::<W>())
            .field("styles", &std::any::type_name::<S>())
            .finish_non_exhaustive()
    }
}

impl Default for HumanLayer {
    fn default() -> Self {
        Self {
            last_event_was_long: Default::default(),
            span_events: FmtSpan::NONE,
            color_output: ShouldColor::Always,
            output_writer: Mutex::new(std::io::stderr()),
            styles: LayerStyles::new(),
            textwrap_options: Some(TextWrapOptionsOwned::new()),
        }
    }
}

impl HumanLayer {
    /// Construct a new [`HumanLayer`] that writes to [`Stderr`].
    pub fn new() -> Self {
        Self::default()
    }
}

impl<W, S> HumanLayer<W, S> {
    /// Set the writer that log messages are written to.
    ///
    /// This does not change colored output by default.
    ///
    /// The `output_writer` should implement [`std::io::Write`] for the [`HumanLayer`] to
    /// implement [`tracing_subscriber::Layer`].
    pub fn with_output_writer<W2>(self, output_writer: W2) -> HumanLayer<W2, S> {
        HumanLayer {
            last_event_was_long: self.last_event_was_long,
            span_events: self.span_events,
            color_output: self.color_output,
            output_writer: Mutex::new(output_writer),
            styles: self.styles,
            textwrap_options: self.textwrap_options,
        }
    }

    /// Set the [`textwrap::Options`].
    ///
    /// If `None`, no text wrapping is performed.
    pub fn with_textwrap_options(mut self, textwrap_options: Option<TextWrapOptionsOwned>) -> Self {
        self.textwrap_options = textwrap_options;
        self
    }

    /// Set the output coloring.
    pub fn with_color_output(mut self, color_output: bool) -> Self {
        // TODO: Should we expose `ShouldColor` and take it as a parameter here?
        self.color_output = if color_output {
            ShouldColor::Always
        } else {
            ShouldColor::Never
        };
        self
    }

    /// Set which span events are logged.
    pub fn with_span_events(mut self, span_events: FmtSpan) -> Self {
        self.span_events = span_events;
        self
    }

    /// Set the output style to the given [`ProvideStyle`] implementation, which supplies
    /// [`Style`]s.
    pub fn with_style_provider<S2>(self, styles: S2) -> HumanLayer<W, S2> {
        HumanLayer {
            last_event_was_long: self.last_event_was_long,
            span_events: self.span_events,
            color_output: self.color_output,
            output_writer: self.output_writer,
            styles,
            textwrap_options: self.textwrap_options,
        }
    }

    fn update_long(&self, last_event_was_long: AtomicBool) {
        self.last_event_was_long
            .store(last_event_was_long.load(Ordering::SeqCst), Ordering::SeqCst);
    }
}

impl<W, S> HumanLayer<W, S>
where
    S: ProvideStyle,
{
    fn event<R>(
        &self,
        metadata: &'static Metadata<'static>,
        scope: Option<Scope<'_, R>>,
    ) -> HumanEvent<'_>
    where
        R: tracing::Subscriber,
        R: for<'lookup> LookupSpan<'lookup>,
    {
        HumanEvent {
            // Note: We load the value out of our `AtomicBool` and then clone it to create a _new_
            // `AtomicBool`. After writing an event, we update _our_ `AtomicBool`.
            last_event_was_long: self.last_event_was_long.load(Ordering::SeqCst).into(),
            style: self.styles.for_metadata(metadata),
            color: self.color_output,
            spans: scope
                .map(|scope| SpanInfo::from_scope(scope))
                .unwrap_or_default(),
            fields: HumanFields::new_event(),
            textwrap_options: self.textwrap_options.as_ref().map(|options| options.into()),
        }
    }

    fn event_for_id<U>(&self, id: &Id, ctx: Context<'_, U>) -> HumanEvent<'_>
    where
        U: tracing::Subscriber,
        U: for<'lookup> LookupSpan<'lookup>,
    {
        self.event(
            ctx.metadata(id)
                .expect("Metadata should exist for the span ID"),
            ctx.span_scope(id),
        )
    }
}

impl<S, W> Layer<S> for HumanLayer<W>
where
    S: Subscriber,
    S: for<'lookup> LookupSpan<'lookup>,
    Self: 'static,
    W: Write,
{
    fn on_new_span(&self, attrs: &span::Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let mut fields = HumanFields::new_span();
        attrs.record(&mut fields);
        if let Some(span_ref) = ctx.span(id) {
            span_ref
                .extensions_mut()
                .insert(FormattedFields::<HumanLayer>::new(
                    StyledSpanFields {
                        style: self.styles.for_metadata(attrs.metadata()),
                        color: self.color_output,
                        fields,
                    }
                    .to_string(),
                ));

            if self.span_events.clone() & FmtSpan::NEW != FmtSpan::NONE {
                let mut human_event = self.event(span_ref.metadata(), ctx.span_scope(id));
                human_event.fields.message = Some("new".into());
                let _ = write!(self.output_writer.lock(), "{human_event}");
                self.update_long(human_event.last_event_was_long);
            }
        }
    }

    fn on_record(&self, id: &Id, values: &span::Record<'_>, ctx: Context<'_, S>) {
        let mut fields = HumanFields::new_span();
        values.record(&mut fields);
        if let Some(span_ref) = ctx.span(id) {
            span_ref
                .extensions_mut()
                .insert(FormattedFields::<HumanLayer>::new(
                    StyledSpanFields {
                        style: self.styles.for_metadata(span_ref.metadata()),
                        color: self.color_output,
                        fields,
                    }
                    .to_string(),
                ));
        }
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let mut human_event = self.event(event.metadata(), ctx.event_scope(event));
        event.record(&mut human_event);
        let _ = write!(self.output_writer.lock(), "{human_event}");
        self.update_long(human_event.last_event_was_long);
    }

    fn on_enter(&self, id: &Id, ctx: Context<'_, S>) {
        if self.span_events.clone() & FmtSpan::ENTER != FmtSpan::NONE {
            let mut human_event = self.event_for_id(id, ctx);
            human_event.fields.message = Some("enter".into());
            let _ = write!(self.output_writer.lock(), "{human_event}");
            self.update_long(human_event.last_event_was_long);
        }
    }

    fn on_exit(&self, id: &Id, ctx: Context<'_, S>) {
        if self.span_events.clone() & FmtSpan::EXIT != FmtSpan::NONE {
            let mut human_event = self.event_for_id(id, ctx);
            human_event.fields.message = Some("exit".into());
            let _ = write!(self.output_writer.lock(), "{human_event}");
            self.update_long(human_event.last_event_was_long);
        }
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        if self.span_events.clone() & FmtSpan::CLOSE != FmtSpan::NONE {
            let mut human_event = self.event_for_id(&id, ctx);
            human_event.fields.message = Some("close".into());
            let _ = write!(self.output_writer.lock(), "{human_event}");
            self.update_long(human_event.last_event_was_long);
        }
    }
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use super::*;

    #[test]
    fn test_debug() {
        expect![[r#"
            HumanLayer {
                span_events: FmtSpan::NONE,
                color_output: Always,
                textwrap_options: Some(
                    TextWrapOptionsOwned {
                        width: Fixed(
                            80,
                        ),
                        line_ending: LF,
                        break_words: false,
                        wrap_algorithm: OptimalFit(
                            Penalties {
                                nline_penalty: 1000,
                                overflow_penalty: 2500,
                                short_last_line_fraction: 4,
                                short_last_line_penalty: 25,
                                hyphen_penalty: 25,
                            },
                        ),
                        word_separator: AsciiSpace,
                        word_splitter: NoHyphenation,
                    },
                ),
                output_writer: "std::io::stdio::Stderr",
                styles: "tracing_human_layer::style::LayerStyles",
                ..
            }"#]]
        .assert_eq(&format!("{:#?}", HumanLayer::new()));
    }
}
