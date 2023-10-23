use std::fmt::Debug;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use tracing::span;
use tracing::Event;
use tracing::Id;
use tracing::Level;
use tracing::Subscriber;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::FormattedFields;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::registry::Scope;
use tracing_subscriber::Layer;

use crate::HumanEvent;
use crate::HumanFields;
use crate::Style;
use crate::StyledSpanFields;

#[derive(Debug)]
pub struct HumanLayer {
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
}

impl Default for HumanLayer {
    fn default() -> Self {
        Self {
            last_event_was_long: Default::default(),
            span_events: FmtSpan::NONE,
        }
    }
}

impl HumanLayer {
    pub fn with_span_events(mut self, span_events: FmtSpan) -> Self {
        self.span_events = span_events;
        self
    }

    fn update_long(&self, last_event_was_long: AtomicBool) {
        self.last_event_was_long
            .store(last_event_was_long.load(Ordering::SeqCst), Ordering::SeqCst);
    }

    fn event<S>(&self, level: Level, scope: Option<Scope<'_, S>>) -> HumanEvent
    where
        S: tracing::Subscriber,
        S: for<'lookup> LookupSpan<'lookup>,
    {
        HumanEvent::new(
            level,
            self.last_event_was_long.load(Ordering::SeqCst).into(),
            scope,
        )
    }

    fn event_for_id<S>(&self, id: &Id, ctx: Context<'_, S>) -> HumanEvent
    where
        S: tracing::Subscriber,
        S: for<'lookup> LookupSpan<'lookup>,
    {
        self.event(
            *ctx.metadata(id)
                .expect("Metadata should exist for the span ID")
                .level(),
            ctx.span_scope(id),
        )
    }
}

impl<S> Layer<S> for HumanLayer
where
    S: Subscriber,
    S: for<'lookup> LookupSpan<'lookup>,
    Self: 'static,
{
    fn on_new_span(&self, attrs: &span::Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let mut fields = HumanFields::new_span();
        attrs.record(&mut fields);
        if let Some(span_ref) = ctx.span(id) {
            span_ref
                .extensions_mut()
                .insert(FormattedFields::<HumanLayer>::new(
                    StyledSpanFields {
                        style: Style::new(*attrs.metadata().level()),
                        fields,
                    }
                    .to_string(),
                ));

            if self.span_events.clone() & FmtSpan::NEW != FmtSpan::NONE {
                let mut human_event = self.event(*span_ref.metadata().level(), ctx.span_scope(id));
                human_event.fields.message = Some("new".into());
                print!("{human_event}");
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
                        style: Style::new(*span_ref.metadata().level()),
                        fields,
                    }
                    .to_string(),
                ));
        }
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let mut human_event = self.event(*event.metadata().level(), ctx.event_scope(event));
        event.record(&mut human_event);
        print!("{human_event}");
        self.update_long(human_event.last_event_was_long);
    }

    fn on_enter(&self, id: &Id, ctx: Context<'_, S>) {
        if self.span_events.clone() & FmtSpan::ENTER != FmtSpan::NONE {
            let mut human_event = self.event_for_id(id, ctx);
            human_event.fields.message = Some("enter".into());
            print!("{human_event}");
            self.update_long(human_event.last_event_was_long);
        }
    }

    fn on_exit(&self, id: &Id, ctx: Context<'_, S>) {
        if self.span_events.clone() & FmtSpan::EXIT != FmtSpan::NONE {
            let mut human_event = self.event_for_id(id, ctx);
            human_event.fields.message = Some("exit".into());
            print!("{human_event}");
            self.update_long(human_event.last_event_was_long);
        }
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        if self.span_events.clone() & FmtSpan::CLOSE != FmtSpan::NONE {
            let mut human_event = self.event_for_id(&id, ctx);
            human_event.fields.message = Some("close".into());
            print!("{human_event}");
            self.update_long(human_event.last_event_was_long);
        }
    }
}
