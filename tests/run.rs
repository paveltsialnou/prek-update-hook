use prek_update_hook::*;
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, ExitStatus};

mod common;
use common::args_from_str;

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
fn run_invalid_repo_tag_equals_format() {
    let result = run(args_from_str("--repo-tag=novalue"));
    match result {
        ExitCode::Error(msg) => assert!(msg.contains("--repo-tag")),
        _ => panic!("expected error"),
    }
}

#[test]
fn run_missing_exclude_repo_tag_value() {
    let result = run(args_from_str("--exclude-repo-tag"));
    match result {
        ExitCode::Error(msg) => assert!(msg.contains("--exclude-repo-tag requires a value")),
        _ => panic!("expected error"),
    }
}

#[test]
fn run_invalid_exclude_repo_tag_format() {
    let result = run(args_from_str("--exclude-repo-tag badvalue"));
    match result {
        ExitCode::Error(msg) => {
            assert!(msg.contains("--exclude-repo-tag requires <REPO>=<PATTERN>"))
        }
        _ => panic!("expected error"),
    }
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
fn exit_code_debug() {
    assert_eq!(format!("{:?}", ExitCode::Success), "Success");
    assert_eq!(format!("{:?}", ExitCode::UpdateNeeded), "UpdateNeeded");
    assert_eq!(
        format!("{:?}", ExitCode::Error("e".into())),
        r#"Error("e")"#
    );
}

#[test]
fn run_with_command_not_found() {
    let result = run_with(args_from_str("--verbose"), |_| {
        Command::new("/nonexistent").status()
    });
    match result {
        ExitCode::Error(msg) => assert!(msg.contains("`prek` not found on PATH")),
        _ => panic!("expected error"),
    }
}

#[test]
fn run_with_other_io_error() {
    let result = run_with(args_from_str("--verbose"), |_| {
        Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "nope",
        ))
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
