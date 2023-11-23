use std::borrow::Cow;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitStatus;
use std::process::Output;
use std::process::Stdio;

use escargot::format::Message;
use miette::miette;
use miette::Context;
use miette::IntoDiagnostic;

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
    fn executable(&self) -> miette::Result<PathBuf> {
        let messages = escargot::CargoBuild::new()
            .example(&self.name)
            .exec()
            .into_diagnostic()
            .wrap_err_with(|| format!("Failed to build example binary `{}`", self.name))?;
        for message in messages {
            if let Message::CompilerArtifact(artifact) =
                message.into_diagnostic()?.decode().into_diagnostic()?
            {
                if artifact.target.name != self.name
                    || !artifact.target.kind.contains(&Cow::Borrowed("example"))
                {
                    continue;
                }
                return Ok(artifact
                    .executable
                    .ok_or_else(|| miette!("Example `{}` has no binary", self.name))?
                    .into_owned());
            }
        }
        Err(miette!("No example output binary found"))
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

    pub fn output(&self) -> miette::Result<Utf8Output> {
        let executable = self.executable().wrap_err_with(|| {
            format!("Failed to get executable path for example `{}`", self.name)
        })?;

        let output = Command::new(executable)
            .args(&self.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .into_diagnostic()
            .wrap_err_with(|| format!("Failed to execute example `{}`", self.name))?;

        if output.status.success() {
            output.try_into()
        } else {
            Err(miette!("Example `{}` failed: {}", self.name, output.status))
        }
    }
}

/// Like [`std::process::Output`] but UTF-8 decoded.
pub struct Utf8Output {
    pub status: ExitStatus,
    pub stdout: String,
    pub stderr: String,
}

impl TryFrom<Output> for Utf8Output {
    type Error = miette::Report;

    fn try_from(output: Output) -> Result<Self, Self::Error> {
        let stdout = String::from_utf8(output.stdout).map_err(|err| {
            miette!(
                "Command wrote invalid stdout: {err}: {}",
                String::from_utf8_lossy(err.as_bytes())
            )
        })?;
        let stderr = String::from_utf8(output.stderr).map_err(|err| {
            miette!(
                "Command wrote invalid stderr: {err}: {}",
                String::from_utf8_lossy(err.as_bytes())
            )
        })?;

        Ok(Self {
            status: output.status,
            stdout,
            stderr,
        })
    }
}
