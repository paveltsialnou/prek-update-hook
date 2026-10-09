use prek_update_hook::*;

mod common;
use common::args_from_str;

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
fn build_command_with_exclude_repo_tag() {
    let args = parse_args(args_from_str("--exclude-repo-tag r1=v1.*")).unwrap();
    let cmd = build_command(&args);
    let collected: Vec<_> = cmd.get_args().collect();
    assert_eq!(
        collected,
        vec!["update", "--check", "--repo-exclude-tag", "r1=v1.*"]
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
fn print_help_does_not_panic() {
    print_help();
}
