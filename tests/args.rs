use prek_update_hook::*;

mod common;
use common::args_from_str;

#[test]
fn empty_args() {
    let args = parse_args(args_from_str("")).unwrap();
    assert!(!args.fix);
    assert!(!args.help);
    assert!(!args.version);
    assert!(args.exclude_repos.is_empty());
    assert!(args.repo_tags.is_empty());
    assert!(args.exclude_repo_tags.is_empty());
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
fn exclude_repo_with_equals_empty_value() {
    let err = parse_args(args_from_str("--exclude-repo=")).unwrap_err();
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
fn repo_tag_equals_valid_format() {
    let args = parse_args(args_from_str("--repo-tag=myrepo=v1.*")).unwrap();
    assert_eq!(args.repo_tags, vec![("myrepo".into(), "v1.*".into())]);
}

#[test]
fn repo_tag_with_equals_empty_pattern() {
    let args = parse_args(args_from_str("--repo-tag=myrepo=")).unwrap();
    assert_eq!(args.repo_tags, vec![("myrepo".into(), "".into())]);
}

#[test]
fn repo_tag_missing_value() {
    let err = parse_args(args_from_str("--repo-tag")).unwrap_err();
    assert_eq!(err, ParseError::MissingValue("--repo-tag".into()));
}

#[test]
fn repo_tag_invalid_format() {
    let err = parse_args(args_from_str("--repo-tag badvalue")).unwrap_err();
    assert_eq!(
        err,
        ParseError::InvalidRepoFilter {
            flag: "--repo-tag",
            value: "badvalue".into(),
        }
    );
}

#[test]
fn repo_tag_equals_invalid_format() {
    let err = parse_args(args_from_str("--repo-tag=novalue")).unwrap_err();
    assert_eq!(
        err,
        ParseError::InvalidRepoFilter {
            flag: "--repo-tag",
            value: "novalue".into(),
        }
    );
}

#[test]
fn exclude_repo_tag_long() {
    let args = parse_args(args_from_str("--exclude-repo-tag myrepo=v1.*")).unwrap();
    assert_eq!(
        args.exclude_repo_tags,
        vec![("myrepo".into(), "v1.*".into())]
    );
}

#[test]
fn exclude_repo_tag_equals() {
    let args = parse_args(args_from_str("--exclude-repo-tag=myrepo=v1.*")).unwrap();
    assert_eq!(
        args.exclude_repo_tags,
        vec![("myrepo".into(), "v1.*".into())]
    );
}

#[test]
fn exclude_repo_tag_missing_value() {
    let err = parse_args(args_from_str("--exclude-repo-tag")).unwrap_err();
    assert_eq!(err, ParseError::MissingValue("--exclude-repo-tag".into()));
}

#[test]
fn exclude_repo_tag_invalid_format() {
    let err = parse_args(args_from_str("--exclude-repo-tag badvalue")).unwrap_err();
    assert_eq!(
        err,
        ParseError::InvalidRepoFilter {
            flag: "--exclude-repo-tag",
            value: "badvalue".into(),
        }
    );
}

#[test]
fn exclude_repo_tag_equals_invalid_format() {
    let err = parse_args(args_from_str("--exclude-repo-tag=novalue")).unwrap_err();
    assert_eq!(
        err,
        ParseError::InvalidRepoFilter {
            flag: "--exclude-repo-tag",
            value: "novalue".into(),
        }
    );
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
fn multiple_exclude_repo_tags() {
    let args = parse_args(args_from_str(
        "--exclude-repo-tag a=1.* --exclude-repo-tag=b=2.*",
    ))
    .unwrap();
    assert_eq!(
        args.exclude_repo_tags,
        vec![("a".into(), "1.*".into()), ("b".into(), "2.*".into())]
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
fn passthrough_args() {
    let args = parse_args(args_from_str("--verbose --unrelated")).unwrap();
    assert_eq!(args.passthrough, vec!["--verbose", "--unrelated"]);
}

#[test]
fn combined_args() {
    let args = parse_args(args_from_str(
        "-f --exclude-repo=r1 --repo-tag r2=v2.* --exclude-repo-tag r3=v3.* --verbose",
    ))
    .unwrap();
    assert!(args.fix);
    assert_eq!(args.exclude_repos, vec!["r1"]);
    assert_eq!(args.repo_tags, vec![("r2".into(), "v2.*".into())]);
    assert_eq!(args.exclude_repo_tags, vec![("r3".into(), "v3.*".into())]);
    assert_eq!(args.passthrough, vec!["--verbose"]);
}

#[test]
fn args_struct_fields() {
    let args = parse_args(args_from_str(
        "-f --exclude-repo=a --repo-tag b=c --exclude-repo-tag d=e -- -x",
    ))
    .unwrap();
    assert!(args.fix);
    assert_eq!(args.exclude_repos, vec!["a"]);
    assert_eq!(args.repo_tags, vec![("b".into(), "c".into())]);
    assert_eq!(args.exclude_repo_tags, vec![("d".into(), "e".into())]);
    assert_eq!(args.passthrough, vec!["--", "-x"]);
}

#[test]
fn parse_error_display_missing() {
    let err = ParseError::MissingValue("--repo-tag".into());
    assert_eq!(format!("{err:?}"), r#"MissingValue("--repo-tag")"#);
}

#[test]
fn parse_error_display_invalid() {
    let err = ParseError::InvalidRepoFilter {
        flag: "--repo-tag",
        value: "bad".into(),
    };
    assert_eq!(
        format!("{err:?}"),
        r#"InvalidRepoFilter { flag: "--repo-tag", value: "bad" }"#
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
    assert!(args.exclude_repo_tags.is_empty());
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
    assert!(args.exclude_repo_tags.is_empty());
    assert!(args.passthrough.is_empty());
}
