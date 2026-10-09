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
