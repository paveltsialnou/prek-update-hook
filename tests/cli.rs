mod common;
use common::bin;

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
fn binary_invalid_exclude_repo_tag_exits_two() {
    let output = bin()
        .arg("--exclude-repo-tag")
        .arg("badvalue")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("--exclude-repo-tag"));
}

#[test]
fn binary_missing_exclude_repo_exits_two() {
    let output = bin().arg("--exclude-repo").output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("--exclude-repo"));
}

#[test]
fn binary_missing_prek_exits_two() {
    // Point PATH at an empty directory so `prek` cannot be found on any host.
    let empty = std::env::temp_dir().join("prek-update-hook-empty-path");
    let _ = std::fs::remove_dir_all(&empty);
    std::fs::create_dir_all(&empty).unwrap();

    let output = bin().env("PATH", &empty).output().unwrap();

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("`prek` not found on PATH"));

    let _ = std::fs::remove_dir_all(&empty);
}
