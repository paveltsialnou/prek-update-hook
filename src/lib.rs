use std::process::{Command, ExitStatus};

#[derive(Debug, PartialEq)]
pub struct Args {
    pub fix: bool,
    pub help: bool,
    pub version: bool,
    pub exclude_repos: Vec<String>,
    pub repo_tags: Vec<(String, String)>,
    pub exclude_repo_tags: Vec<(String, String)>,
    pub passthrough: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    MissingValue(String),
    InvalidRepoFilter { flag: &'static str, value: String },
}

pub fn parse_args<I: IntoIterator<Item = String>>(args: I) -> Result<Args, ParseError> {
    let args: Vec<String> = args.into_iter().skip(1).collect();

    let mut fix = false;
    let mut help = false;
    let mut version = false;
    let mut exclude_repos: Vec<String> = Vec::new();
    let mut repo_tags: Vec<(String, String)> = Vec::new();
    let mut exclude_repo_tags: Vec<(String, String)> = Vec::new();
    let mut passthrough: Vec<String> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-f" | "--fix" => fix = true,
            "-h" | "--help" => help = true,
            "-V" | "--version" => version = true,
            "--exclude-repo" => {
                i += 1;
                if i < args.len() {
                    exclude_repos.push(args[i].clone());
                } else {
                    return Err(ParseError::MissingValue("--exclude-repo".into()));
                }
            }
            arg if arg.starts_with("--exclude-repo=") => {
                let val = arg.strip_prefix("--exclude-repo=").unwrap();
                if val.is_empty() {
                    return Err(ParseError::MissingValue("--exclude-repo".into()));
                }
                exclude_repos.push(val.to_string());
            }
            "--repo-tag" => {
                i += 1;
                if i < args.len() {
                    if let Some((repo, pattern)) = args[i].split_once('=') {
                        repo_tags.push((repo.to_string(), pattern.to_string()));
                    } else {
                        return Err(ParseError::InvalidRepoFilter {
                            flag: "--repo-tag",
                            value: args[i].clone(),
                        });
                    }
                } else {
                    return Err(ParseError::MissingValue("--repo-tag".into()));
                }
            }
            arg if arg.starts_with("--repo-tag=") => {
                let val = arg.strip_prefix("--repo-tag=").unwrap();
                if let Some((repo, pattern)) = val.split_once('=') {
                    repo_tags.push((repo.to_string(), pattern.to_string()));
                } else {
                    return Err(ParseError::InvalidRepoFilter {
                        flag: "--repo-tag",
                        value: val.to_string(),
                    });
                }
            }
            "--exclude-repo-tag" => {
                i += 1;
                if i < args.len() {
                    if let Some((repo, pattern)) = args[i].split_once('=') {
                        exclude_repo_tags.push((repo.to_string(), pattern.to_string()));
                    } else {
                        return Err(ParseError::InvalidRepoFilter {
                            flag: "--exclude-repo-tag",
                            value: args[i].clone(),
                        });
                    }
                } else {
                    return Err(ParseError::MissingValue("--exclude-repo-tag".into()));
                }
            }
            arg if arg.starts_with("--exclude-repo-tag=") => {
                let val = arg.strip_prefix("--exclude-repo-tag=").unwrap();
                if let Some((repo, pattern)) = val.split_once('=') {
                    exclude_repo_tags.push((repo.to_string(), pattern.to_string()));
                } else {
                    return Err(ParseError::InvalidRepoFilter {
                        flag: "--exclude-repo-tag",
                        value: val.to_string(),
                    });
                }
            }
            _ => passthrough.push(args[i].clone()),
        }
        i += 1;
    }

    Ok(Args {
        fix,
        help,
        version,
        exclude_repos,
        repo_tags,
        exclude_repo_tags,
        passthrough,
    })
}

#[must_use]
pub fn build_command(args: &Args) -> Command {
    let mut cmd = Command::new("prek");
    cmd.arg("update");
    if !args.fix {
        cmd.arg("--check");
    }
    for repo in &args.exclude_repos {
        cmd.arg("--exclude-repo").arg(repo);
    }
    for (repo, pattern) in &args.repo_tags {
        cmd.arg("--repo-include-tag")
            .arg(format!("{repo}={pattern}"));
    }
    for (repo, pattern) in &args.exclude_repo_tags {
        cmd.arg("--repo-exclude-tag")
            .arg(format!("{repo}={pattern}"));
    }
    cmd.args(&args.passthrough);
    cmd
}

pub fn print_help() {
    println!(
        "prek-update-hook: check or update hook repositories pinned in the prek config.\n\
         \n\
         Usage: prek-update-hook [OPTIONS]\n\
         \n\
         Options:\n\
         \x20 -f, --fix              Apply available updates instead of only checking\n\
         \x20 -h, --help             Print help\n\
         \x20 -V, --version          Print version\n\
         \x20 --exclude-repo=<REPO>  Skip this repo during --fix (may be repeated)\n\
         \x20 --exclude-repo-tag <REPO>=<PATTERN>\n\
         \x20                       Ignore tags matching this glob for a repo\n\
         \x20                       (may be repeated)\n\
         \x20 --repo-tag <REPO>=<PATTERN>\n\
         \x20                       Only consider tags matching this glob for a repo\n\
         \x20                       (may be repeated)\n\
         \n\
         All other options are forwarded to `prek update`."
    );
}

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

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<Args, ParseError> {
        let mut argv = vec!["prek-update-hook".to_string()];
        argv.extend(args.iter().map(|s| s.to_string()));
        parse_args(argv)
    }

    #[test]
    fn parses_defaults() {
        let args = parse(&[]).unwrap();
        assert!(!args.fix);
        assert!(!args.help);
        assert!(!args.version);
        assert!(args.exclude_repos.is_empty());
        assert!(args.repo_tags.is_empty());
        assert!(args.exclude_repo_tags.is_empty());
        assert!(args.passthrough.is_empty());
    }

    #[test]
    fn parses_repo_filters() {
        let args = parse(&["--repo-tag", "a=1.*", "--exclude-repo-tag=b=2.*"]).unwrap();
        assert_eq!(args.repo_tags, vec![("a".into(), "1.*".into())]);
        assert_eq!(args.exclude_repo_tags, vec![("b".into(), "2.*".into())]);
    }

    #[test]
    fn invalid_repo_filter_reports_flag() {
        let err = parse(&["--exclude-repo-tag", "bad"]).unwrap_err();
        assert_eq!(
            err,
            ParseError::InvalidRepoFilter {
                flag: "--exclude-repo-tag",
                value: "bad".into(),
            }
        );
    }

    #[test]
    fn builds_command_with_filters() {
        let args = parse(&["--repo-tag", "a=1.*", "--exclude-repo-tag", "b=2.*"]).unwrap();
        let cmd = build_command(&args);
        let got: Vec<_> = cmd.get_args().collect();
        assert_eq!(
            got,
            vec![
                "update",
                "--check",
                "--repo-include-tag",
                "a=1.*",
                "--repo-exclude-tag",
                "b=2.*",
            ]
        );
    }
}
