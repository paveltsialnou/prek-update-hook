use prek_update_hook::*;
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, ExitStatus};

fn args_from_str(s: &str) -> Vec<String> {
    let mut v: Vec<String> = vec!["prek-update-hook".into()];
    v.extend(s.split_whitespace().map(String::from));
    v
}

#[test]
fn empty_args() {
    let args = parse_args(args_from_str("")).unwrap();
    assert!(!args.fix);
    assert!(!args.help);
    assert!(!args.version);
    assert!(args.exclude_repos.is_empty());
    assert!(args.repo_tags.is_empty());
    assert!(args.passthrough.is_empty());
}

#[test]
fn fix_short_flag() {
    let args = parse_args(args_from_str("-f")).unwrap();
    assert!(args.fix);
}

#[test]
fn fix_long_flag() {
    let args = parse_args(args_from_str("--fix")).unwrap();
    assert!(args.fix);
}

#[test]
fn exclude_repo_long() {
    let args = parse_args(args_from_str("--exclude-repo foo")).unwrap();
    assert_eq!(args.exclude_repos, vec!["foo"]);
}

#[test]
fn exclude_repo_equals() {
    let args = parse_args(args_from_str("--exclude-repo=foo")).unwrap();
    assert_eq!(args.exclude_repos, vec!["foo"]);
}

#[test]
fn exclude_repo_missing_value() {
    let err = parse_args(args_from_str("--exclude-repo")).unwrap_err();
    assert_eq!(err, ParseError::MissingValue("--exclude-repo".into()));
}

#[test]
fn repo_tag_long() {
    let args = parse_args(args_from_str("--repo-tag myrepo=v1.*")).unwrap();
    assert_eq!(args.repo_tags, vec![("myrepo".into(), "v1.*".into())]);
}

#[test]
fn repo_tag_equals() {
    let args = parse_args(args_from_str("--repo-tag=myrepo=v1.*")).unwrap();
    assert_eq!(args.repo_tags, vec![("myrepo".into(), "v1.*".into())]);
}

#[test]
fn repo_tag_missing_value() {
    let err = parse_args(args_from_str("--repo-tag")).unwrap_err();
    assert_eq!(err, ParseError::MissingValue("--repo-tag".into()));
}

#[test]
fn repo_tag_invalid_format() {
    let err = parse_args(args_from_str("--repo-tag badvalue")).unwrap_err();
    assert_eq!(err, ParseError::InvalidRepoTag("badvalue".into()));
}

#[test]
fn passthrough_args() {
    let args = parse_args(args_from_str("--verbose --unrelated")).unwrap();
    assert_eq!(args.passthrough, vec!["--verbose", "--unrelated"]);
}

#[test]
fn combined_args() {
    let args = parse_args(args_from_str(
        "-f --exclude-repo=r1 --repo-tag r2=v2.* --verbose",
    ))
    .unwrap();
    assert!(args.fix);
    assert_eq!(args.exclude_repos, vec!["r1"]);
    assert_eq!(args.repo_tags, vec![("r2".into(), "v2.*".into())]);
    assert_eq!(args.passthrough, vec!["--verbose"]);
}

#[test]
fn multiple_exclude_repos() {
    let args = parse_args(args_from_str("--exclude-repo a --exclude-repo=b")).unwrap();
    assert_eq!(args.exclude_repos, vec!["a", "b"]);
}

#[test]
fn multiple_repo_tags() {
    let args = parse_args(args_from_str("--repo-tag a=1.* --repo-tag=b=2.*")).unwrap();
    assert_eq!(
        args.repo_tags,
        vec![("a".into(), "1.*".into()), ("b".into(), "2.*".into())]
    );
}

#[test]
fn build_command_check_mode() {
    let args = parse_args(args_from_str("")).unwrap();
    let cmd = build_command(&args);
    assert_eq!(
        cmd.get_args().collect::<Vec<_>>(),
        vec!["update", "--check"]
    );
}

#[test]
fn build_command_fix_mode() {
    let args = parse_args(args_from_str("--fix")).unwrap();
    let cmd = build_command(&args);
    assert_eq!(cmd.get_args().collect::<Vec<_>>(), vec!["update"]);
}

#[test]
fn build_command_with_exclude_and_tag() {
    let args = parse_args(args_from_str(
        "--exclude-repo=r1 --repo-tag r2=v2.* --verbose",
    ))
    .unwrap();
    let cmd = build_command(&args);
    let collected: Vec<_> = cmd.get_args().collect();
    assert_eq!(
        collected,
        vec![
            "update",
            "--check",
            "--exclude-repo",
            "r1",
            "--repo-include-tag",
            "r2=v2.*",
            "--verbose",
        ]
    );
}

#[test]
fn help_long_flag_in_passthrough() {
    let args = parse_args(args_from_str("--help")).unwrap();
    assert!(args.help);
    assert!(args.passthrough.is_empty());
}

#[test]
fn help_short_flag_in_passthrough() {
    let args = parse_args(args_from_str("-h")).unwrap();
    assert!(args.help);
    assert!(args.passthrough.is_empty());
}

#[test]
fn version_short_flag() {
    let args = parse_args(args_from_str("-V")).unwrap();
    assert!(args.version);
}

#[test]
fn version_long_flag() {
    let args = parse_args(args_from_str("--version")).unwrap();
    assert!(args.version);
}

#[test]
fn print_help_does_not_panic() {
    print_help();
}

#[test]
fn parse_error_display_missing() {
    let err = ParseError::MissingValue("--repo-tag".into());
    assert_eq!(format!("{err:?}"), r#"MissingValue("--repo-tag")"#);
}

#[test]
fn parse_error_display_invalid() {
    let err = ParseError::InvalidRepoTag("bad".into());
    assert_eq!(format!("{err:?}"), r#"InvalidRepoTag("bad")"#);
}

#[test]
fn args_struct_fields() {
    let args = parse_args(args_from_str("-f --exclude-repo=a --repo-tag b=c -- -x")).unwrap();
    assert!(args.fix);
    assert_eq!(args.exclude_repos, vec!["a"]);
    assert_eq!(args.repo_tags, vec![("b".into(), "c".into())]);
    assert_eq!(args.passthrough, vec!["--", "-x"]);
}

#[test]
fn run_help_long() {
    let result = run(args_from_str("--help"));
    assert!(matches!(result, ExitCode::Success));
}

#[test]
fn run_help_short() {
    let result = run(args_from_str("-h"));
    assert!(matches!(result, ExitCode::Success));
}

#[test]
fn run_missing_exclude_repo_value() {
    let result = run(args_from_str("--exclude-repo"));
    match result {
        ExitCode::Error(msg) => assert!(msg.contains("--exclude-repo")),
        _ => panic!("expected error"),
    }
}

#[test]
fn run_missing_repo_tag_value() {
    let result = run(args_from_str("--repo-tag"));
    match result {
        ExitCode::Error(msg) => assert!(msg.contains("--repo-tag")),
        _ => panic!("expected error"),
    }
}

#[test]
fn run_invalid_repo_tag_format() {
    let result = run(args_from_str("--repo-tag badvalue"));
    match result {
        ExitCode::Error(msg) => assert!(msg.contains("--repo-tag")),
        _ => panic!("expected error"),
    }
}

#[test]
fn repo_tag_equals_invalid_format() {
    let err = parse_args(args_from_str("--repo-tag=novalue")).unwrap_err();
    assert_eq!(err, ParseError::InvalidRepoTag("novalue".into()));
}

#[test]
fn repo_tag_equals_valid_format() {
    let args = parse_args(args_from_str("--repo-tag=myrepo=v1.*")).unwrap();
    assert_eq!(args.repo_tags, vec![("myrepo".into(), "v1.*".into())]);
}

#[test]
fn run_invalid_repo_tag_equals_format() {
    let result = run(args_from_str("--repo-tag=novalue"));
    match result {
        ExitCode::Error(msg) => assert!(msg.contains("--repo-tag")),
        _ => panic!("expected error"),
    }
}

#[test]
fn exit_code_debug() {
    assert_eq!(format!("{:?}", ExitCode::Success), "Success");
    assert_eq!(format!("{:?}", ExitCode::UpdateNeeded), "UpdateNeeded");
    assert_eq!(
        format!("{:?}", ExitCode::Error("e".into())),
        r#"Error("e")"#
    );
}

#[test]
fn build_command_fix_with_exclude_and_tag() {
    let args = parse_args(args_from_str(
        "--fix --exclude-repo=a --repo-tag b=c --verbose",
    ))
    .unwrap();
    let cmd = build_command(&args);
    let collected: Vec<_> = cmd.get_args().collect();
    assert_eq!(
        collected,
        vec![
            "update",
            "--exclude-repo",
            "a",
            "--repo-include-tag",
            "b=c",
            "--verbose",
        ]
    );
}

#[test]
fn build_command_multiple_excludes() {
    let args = parse_args(args_from_str("--exclude-repo=x --exclude-repo=y")).unwrap();
    let cmd = build_command(&args);
    let collected: Vec<_> = cmd.get_args().collect();
    assert_eq!(
        collected,
        vec![
            "update",
            "--check",
            "--exclude-repo",
            "x",
            "--exclude-repo",
            "y"
        ]
    );
}

#[test]
fn build_command_multiple_tags() {
    let args = parse_args(args_from_str("--repo-tag a=1.* --repo-tag=b=2.*")).unwrap();
    let cmd = build_command(&args);
    let collected: Vec<_> = cmd.get_args().collect();
    assert_eq!(
        collected,
        vec![
            "update",
            "--check",
            "--repo-include-tag",
            "a=1.*",
            "--repo-include-tag",
            "b=2.*",
        ]
    );
}

#[test]
fn parse_args_empty_iter() {
    let args = parse_args(std::iter::empty::<String>()).unwrap();
    assert!(!args.fix);
    assert!(!args.help);
    assert!(!args.version);
    assert!(args.exclude_repos.is_empty());
    assert!(args.repo_tags.is_empty());
    assert!(args.passthrough.is_empty());
}

#[test]
fn parse_args_only_binary_name() {
    let args = parse_args(vec!["prek-update-hook".to_string()]).unwrap();
    assert!(!args.fix);
    assert!(!args.help);
    assert!(!args.version);
    assert!(args.exclude_repos.is_empty());
    assert!(args.repo_tags.is_empty());
    assert!(args.passthrough.is_empty());
}

#[test]
fn run_missing_exclude_repo_value_via_run() {
    let result = run(args_from_str("--exclude-repo"));
    match result {
        ExitCode::Error(msg) => assert!(msg.contains("--exclude-repo requires a value")),
        _ => panic!("expected error"),
    }
}

#[test]
fn run_missing_repo_tag_value_via_run() {
    let result = run(args_from_str("--repo-tag"));
    match result {
        ExitCode::Error(msg) => assert!(msg.contains("--repo-tag requires a value")),
        _ => panic!("expected error"),
    }
}

#[test]
fn run_invalid_repo_tag_format_via_run() {
    let result = run(args_from_str("--repo-tag badvalue"));
    match result {
        ExitCode::Error(msg) => assert!(msg.contains("--repo-tag requires <REPO>=<PATTERN>")),
        _ => panic!("expected error"),
    }
}

#[test]
fn run_invalid_repo_tag_equals_via_run() {
    let result = run(args_from_str("--repo-tag=novalue"));
    match result {
        ExitCode::Error(msg) => assert!(msg.contains("--repo-tag requires <REPO>=<PATTERN>")),
        _ => panic!("expected error"),
    }
}

#[test]
fn exclude_repo_with_equals_empty_value() {
    let err = parse_args(args_from_str("--exclude-repo=")).unwrap_err();
    assert_eq!(err, ParseError::MissingValue("--exclude-repo".into()));
}

#[test]
fn repo_tag_with_equals_empty_pattern() {
    let args = parse_args(args_from_str("--repo-tag=myrepo=")).unwrap();
    assert_eq!(args.repo_tags, vec![("myrepo".into(), "".into())]);
}

#[test]
fn run_with_command_not_found() {
    let result = run_with(args_from_str("--verbose"), |_| {
        Command::new("/nonexistent").status()
    });
    match result {
        ExitCode::Error(msg) => assert!(msg.contains("failed to run prek")),
        _ => panic!("expected error"),
    }
}

#[test]
fn run_with_command_success() {
    let result = run_with(args_from_str("--verbose"), |_| {
        Command::new("true").status()
    });
    assert!(matches!(result, ExitCode::Success));
}

#[test]
fn run_with_command_nonzero_exit() {
    let result = run_with(args_from_str("--verbose"), |_| {
        Command::new("false").status()
    });
    assert!(matches!(result, ExitCode::UpdateNeeded));
}

#[test]
fn run_with_command_error_exit() {
    let result = run_with(args_from_str("--verbose"), |_| {
        Ok(ExitStatus::from_raw(2 << 8)) // exit code 2
    });
    match result {
        ExitCode::Error(msg) => assert!(msg.contains("prek exited with code 2")),
        _ => panic!("expected error"),
    }
}

#[test]
fn run_with_signal_killed() {
    let result = run_with(args_from_str("--verbose"), |_| {
        Ok(ExitStatus::from_raw(9)) // SIGKILL
    });
    match result {
        ExitCode::Error(msg) => assert!(msg.contains("killed by a signal")),
        _ => panic!("expected error"),
    }
}

#[test]
fn run_with_help_flag() {
    let result = run_with(args_from_str("--help"), |_| unreachable!());
    assert!(matches!(result, ExitCode::Success));
}

#[test]
fn run_with_version_flag() {
    let result = run_with(args_from_str("--version"), |_| unreachable!());
    assert!(matches!(result, ExitCode::Success));
}

#[test]
fn run_with_error_path() {
    let result = run_with(args_from_str("--repo-tag bad"), |_| unreachable!());
    match result {
        ExitCode::Error(msg) => assert!(msg.contains("--repo-tag")),
        _ => panic!("expected error"),
    }
}

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_prek-update-hook"))
}

#[test]
fn binary_help_exits_zero() {
    let output = bin().arg("--help").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("prek-update-hook"));
}

#[test]
fn binary_version_exits_zero() {
    let output = bin().arg("--version").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn binary_invalid_repo_tag_exits_two() {
    let output = bin().arg("--repo-tag").arg("badvalue").output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("--repo-tag"));
}

#[test]
fn binary_missing_exclude_repo_exits_two() {
    let output = bin().arg("--exclude-repo").output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("--exclude-repo"));
}

#[test]
fn binary_prek_error_exits_two() {
    let dir = std::env::temp_dir().join("prek-update-hook-test");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::process::Command::new("git")
        .args(["init"])
        .current_dir(&dir)
        .output()
        .unwrap();
    let output = bin().current_dir(&dir).output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("prek exited with code"));
    let _ = std::fs::remove_dir_all(&dir);
}
