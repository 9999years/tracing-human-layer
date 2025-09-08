use std::str::FromStr;

use pico_args::Arguments;
use supports_color::Stream;
use tracing_human_layer::BlankLines;
use tracing_human_layer::TextWrapOptionsOwned;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use tracing_human_layer::HumanLayer;

const HELP: &str = "\
tracing-human-layer demo

See: https://github.com/9999years/tracing-human-layer

Options:
    --color    Force colored output
    --stdout   Write output to stdout rather than stderr
    --no-wrap  Don't wrap text
    --width N  Wrap to N columns
    --blank-lines CHOICE  How to wrap blank lines.
                          One of `never`, `around-long`, or `always`.
";

struct Args {
    color: bool,
    stdout: bool,
    wrap: bool,
    width: Option<usize>,
    blank_lines: Option<BlankLines>,
}

fn pico_error(err: String) -> pico_args::Error {
    pico_args::Error::ArgumentParsingFailed { cause: err }
}

struct BlankLinesParsable(BlankLines);

impl From<BlankLinesParsable> for BlankLines {
    fn from(value: BlankLinesParsable) -> Self {
        value.0
    }
}

impl FromStr for BlankLinesParsable {
    type Err = pico_args::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(match s {
            "never" => BlankLines::Never,
            "around-long" => BlankLines::AroundLong,
            "always" => BlankLines::Always,
            _ => {
                return Err(pico_error(format!(
                    "Expected one of `never`, `around-long`, `always`. Found: {s:?}"
                )));
            }
        }))
    }
}

impl Args {
    fn from_env() -> Result<Self, pico_args::Error> {
        let mut args = Arguments::from_env();

        if args.contains(["-h", "--help"]) {
            println!("{}", HELP);
            std::process::exit(0);
        }

        let blank_lines: Option<BlankLinesParsable> = args.opt_value_from_str("--blank-lines")?;

        let ret = Self {
            color: args.contains("--color"),
            stdout: args.contains("--stdout"),
            wrap: !args.contains("--no-wrap"),
            width: args.opt_value_from_str("--width")?,
            blank_lines: blank_lines.map(|blank_lines| blank_lines.into()),
        };

        let extra = args.finish();
        if !extra.is_empty() {
            return Err(pico_error(format!("Found unexpected args: {extra:?}")));
        }

        Ok(ret)
    }
}

fn main() {
    let args = Args::from_env().unwrap();

    let registry = tracing_subscriber::registry();

    let mut layer = HumanLayer::default()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_color_output(
            args.color
                || supports_color::on(if args.stdout {
                    Stream::Stdout
                } else {
                    Stream::Stderr
                })
                .map(|level| level.has_basic)
                .unwrap_or_default(),
        )
        .with_textwrap_options(if args.wrap {
            let mut opts = TextWrapOptionsOwned::new();
            if let Some(width) = args.width {
                opts = opts.with_width(width);
            }
            Some(opts)
        } else {
            None
        });

    if let Some(blank_lines) = args.blank_lines {
        layer = layer.with_blank_lines(blank_lines);
    }

    if args.stdout {
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
