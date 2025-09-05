use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use tracing::span;
use tracing::Level;
use tracing_human_layer::HumanLayer;
use tracing_subscriber::layer::SubscriberExt;

fn emit_span_and_event() {
    let span = span!(
        Level::INFO,
        "puppy doggy",
        span_field = "span_field_value",
        id = 12843205,
        cute = "yes",
    );
    let _guard = span.enter();
    tracing::info!(
        event_field = "event_field_value",
        kind = "silly",
        puppy = "doggy",
        "inspecting pawbs and other stuff and other stuff and other stuff and this and that and the other thing"
    );
}

fn layer() -> HumanLayer<Vec<u8>> {
    HumanLayer::new()
        // 1MB probably enough?
        .with_output_writer(Vec::<u8>::with_capacity(1_000_000))
}

fn layer_stderr() -> HumanLayer {
    HumanLayer::new()
}

pub fn criterion_benchmark(criterion: &mut Criterion) {
    owo_colors::set_override(true);
    let mut group = criterion.benchmark_group("format event");

    tracing::subscriber::with_default(tracing_subscriber::registry().with(layer_stderr()), || {
        group.bench_function("colors, wrapping, stderr", |bencher| {
            bencher.iter(emit_span_and_event)
        });
    });

    tracing::subscriber::with_default(tracing_subscriber::registry().with(layer()), || {
        group.bench_function("colors, wrapping", |bencher| {
            bencher.iter(emit_span_and_event)
        });
    });

    tracing::subscriber::with_default(
        tracing_subscriber::registry().with(layer().with_color_output(false)),
        || {
            group.bench_function("no colors, wrapping", |bencher| {
                bencher.iter(emit_span_and_event)
            });
        },
    );

    tracing::subscriber::with_default(
        tracing_subscriber::registry().with(layer().with_textwrap_options(None)),
        || {
            group.bench_function("colors, no wrapping", |bencher| {
                bencher.iter(emit_span_and_event)
            });
        },
    );

    tracing::subscriber::with_default(
        tracing_subscriber::registry()
            .with(layer().with_color_output(false).with_textwrap_options(None)),
        || {
            group.bench_function("no colors, no wrapping", |bencher| {
                bencher.iter(emit_span_and_event)
            });
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
