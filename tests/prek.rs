//! End-to-end tests that require a real `prek` binary on PATH.
//!
//! These run in CI only after `prek` is installed (see the `integration-tests`
//! job). They are kept separate from the rest of the suite so that everything
//! else can run in parallel without `prek`.

mod common;
use common::bin;
use std::path::Path;

fn init_repo(dir: &Path) {
    std::process::Command::new("git")
        .args(["init", "-q"])
        .current_dir(dir)
        .output()
        .unwrap();
}

#[test]
fn binary_prek_error_exits_two() {
    let dir = std::env::temp_dir().join("prek-update-hook-test-error");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    init_repo(&dir);

    // No prek config, so `prek update --check` fails.
    let output = bin().current_dir(&dir).output().unwrap();

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("prek exited with code"));

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn binary_prek_up_to_date_exits_zero() {
    let dir = std::env::temp_dir().join("prek-update-hook-test-up-to-date");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    init_repo(&dir);
    std::fs::write(
        dir.join("prek.toml"),
        "[[repos]]\n\
         hooks = [{ id = \"check-toml\", name = \"builtin (check-toml)\" }]\n\
         repo = \"builtin\"\n",
    )
    .unwrap();

    // Only builtin repos: there is nothing to update, so the check succeeds.
    let output = bin().current_dir(&dir).output().unwrap();

    assert_eq!(output.status.code(), Some(0));

    let _ = std::fs::remove_dir_all(&dir);
}
