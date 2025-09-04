use std::borrow::Cow;
use std::fmt::Display;
use std::path::PathBuf;
use std::process::Command;

use command_error::CommandExt;
use escargot::error::CargoError;
use escargot::format::Message;
use utf8_command::Utf8Output;

pub type Result<T> = std::result::Result<T, TestError>;

#[derive(Debug)]
pub enum TestError {
    Cargo { example: String, inner: CargoError },
    ExecutableNotFound { example: String },
    ExecutableNotBuilt { example: String },
    Command(command_error::Error),
}

impl From<command_error::Error> for TestError {
    fn from(value: command_error::Error) -> Self {
        Self::Command(value)
    }
}

impl Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestError::Cargo { example, inner } => {
                write!(f, "Failed to build example {example:?}: {inner}")
            }
            TestError::ExecutableNotFound { example } => {
                write!(f, "Example {example:?} has no binary")
            }
            TestError::ExecutableNotBuilt { example } => write!(f, "Example {example:?} not built"),
            TestError::Command(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for TestError {}

pub struct Example {
    name: String,
    args: Vec<String>,
}

impl Example {
    pub fn name(name: impl AsRef<str>) -> Self {
        Example {
            name: name.as_ref().to_owned(),
            args: Default::default(),
        }
    }

    /// Get the path of the example binary.
    fn executable(&self) -> Result<PathBuf> {
        let messages = escargot::CargoBuild::new()
            .example(&self.name)
            .exec()
            .map_err(|inner| TestError::Cargo {
                example: self.name.clone(),
                inner,
            })?;

        for message in messages {
            if let Message::CompilerArtifact(artifact) = message
                .map_err(|inner| TestError::Cargo {
                    example: self.name.clone(),
                    inner,
                })?
                .decode()
                .map_err(|inner| TestError::Cargo {
                    example: self.name.clone(),
                    inner,
                })?
            {
                if artifact.target.name != self.name
                    || !artifact.target.kind.contains(&Cow::Borrowed("example"))
                {
                    continue;
                }
                return Ok(artifact
                    .executable
                    .ok_or_else(|| TestError::ExecutableNotFound {
                        example: self.name.clone(),
                    })?
                    .into_owned());
            }
        }

        Err(TestError::ExecutableNotBuilt {
            example: self.name.clone(),
        })
    }

    pub fn arg(&mut self, arg: impl AsRef<str>) -> &mut Self {
        self.args.push(arg.as_ref().to_owned());
        self
    }

    pub fn args(&mut self, args: impl IntoIterator<Item = impl AsRef<str>>) -> &mut Self {
        self.args
            .extend(args.into_iter().map(|arg| arg.as_ref().to_owned()));
        self
    }

    pub fn output(&self) -> Result<Utf8Output> {
        let executable = self.executable()?;

        Ok(Command::new(executable)
            .args(&self.args)
            .output_checked_utf8()?)
    }
}
