use clap::Parser;
use supports_color::Stream;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use tracing_human_layer::HumanLayer;

#[derive(Parser)]
struct Opts {
    /// Use colored output.
    #[arg(long)]
    color: bool,
    /// Write output to stdout.
    #[arg(long)]
    stdout: bool,
}

fn main() {
    let opts = Opts::parse();

    let registry = tracing_subscriber::registry();

    let layer = HumanLayer::default()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_color_output(
            opts.color
                || supports_color::on(if opts.stdout {
                    Stream::Stdout
                } else {
                    Stream::Stderr
                })
                .map(|level| level.has_basic)
                .unwrap_or_default(),
        );

    if opts.stdout {
        registry
            .with(layer.with_output_writer(std::io::stdout()))
            .init();
    } else {
        registry.with(layer).init();
    }

    emit_events();
}

fn emit_events() {
    tracing::trace!("Trace event.");
    tracing::debug!("Debug event.");
    tracing::info!(field = "field-value", "Info event.");
    tracing::warn!("Warn event.");
    tracing::error!("Error event.");

    tracing::info!("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.");
    tracing::info!("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.");

    tracing::info!(
        field = "field-value",
        other_field = "my-other-field-value",
        "Info event."
    );

    let span = tracing::info_span!("my-span", path = "my/cool/path.txt");
    let _guard = span.enter();

    let span = tracing::info_span!(
        "copy",
        path = "my/cool/path.txt",
        other_path = "my/second/path.txt"
    );
    let _guard = span.enter();

    let span = tracing::info_span!("my-inner-span");
    let _guard = span.enter();

    tracing::trace!("Trace event.");
    tracing::debug!("Debug event.");
    tracing::info!(field = "field-value", "Info event.");
    tracing::warn!("Warn event.");
    tracing::error!("Error event.");

    tracing::info!("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.");

    tracing::info!(
        field = "field-value",
        other_field = "my-other-field-value",
        "Info event."
    );
}
