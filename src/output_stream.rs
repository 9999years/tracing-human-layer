use std::io::Write;

use owo_colors::Stream as OwoStream;

/// A stream to write logs to.
///
/// It would be nice to support arbitrary [`Write`] instances, but for now we have to support
/// [`owo_colors::Stream`].
#[derive(Debug, Clone, Copy)]
pub enum OutputStream {
    /// The standard output stream.
    Stdout,
    /// The standard error stream.
    Stderr,
}

impl OutputStream {
    pub(crate) fn writer(&self) -> OutputWriter {
        match self {
            OutputStream::Stdout => OutputWriter::Stdout(std::io::stdout()),
            OutputStream::Stderr => OutputWriter::Stderr(std::io::stderr()),
        }
    }
}

impl From<OutputStream> for OwoStream {
    fn from(value: OutputStream) -> Self {
        match value {
            OutputStream::Stdout => OwoStream::Stdout,
            OutputStream::Stderr => OwoStream::Stderr,
        }
    }
}

impl TryFrom<OwoStream> for OutputStream {
    type Error = OwoStream;

    fn try_from(value: OwoStream) -> Result<Self, Self::Error> {
        match value {
            OwoStream::Stdout => Ok(Self::Stdout),
            OwoStream::Stderr => Ok(Self::Stderr),
            OwoStream::Stdin => Err(value),
        }
    }
}

pub(crate) enum OutputWriter {
    Stdout(std::io::Stdout),
    Stderr(std::io::Stderr),
}

impl Write for OutputWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            OutputWriter::Stdout(w) => w.write(buf),
            OutputWriter::Stderr(w) => w.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            OutputWriter::Stdout(w) => w.flush(),
            OutputWriter::Stderr(w) => w.flush(),
        }
    }
}
