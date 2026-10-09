use crate::args::{parse_args, Args, ParseError};
use crate::command::{build_command, print_help};
use std::process::ExitStatus;

#[derive(Debug)]
pub enum ExitCode {
    Success,
    UpdateNeeded,
    Error(String),
}

pub fn run_with<I, F>(args: I, mut exec: F) -> ExitCode
where
    I: IntoIterator<Item = String>,
    F: FnMut(&Args) -> Result<ExitStatus, std::io::Error>,
{
    match parse_args(args) {
        Ok(parsed) => match (parsed.help, parsed.version) {
            (true, _) => {
                print_help();
                ExitCode::Success
            }
            (_, true) => {
                println!("{}", env!("CARGO_PKG_VERSION"));
                ExitCode::Success
            }
            _ => match exec(&parsed) {
                Ok(status) => match status.code() {
                    Some(0) => ExitCode::Success,
                    Some(1) => ExitCode::UpdateNeeded,
                    Some(code) => ExitCode::Error(format!("error: prek exited with code {code}")),
                    None => ExitCode::Error("error: prek was killed by a signal".into()),
                },
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => ExitCode::Error(
                    "error: `prek` not found on PATH (install from https://prek.j178.dev/)".into(),
                ),
                Err(e) => ExitCode::Error(format!("error: failed to run prek: {e}")),
            },
        },
        Err(ParseError::MissingValue(flag)) => {
            ExitCode::Error(format!("error: {flag} requires a value"))
        }
        Err(ParseError::InvalidRepoFilter { flag, value }) => ExitCode::Error(format!(
            "error: {flag} requires <REPO>=<PATTERN>, got: {value}"
        )),
    }
}

#[must_use]
pub fn run<I: IntoIterator<Item = String>>(args: I) -> ExitCode {
    run_with(args, |a| {
        let mut cmd = build_command(a);
        cmd.status()
    })
}
