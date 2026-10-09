use crate::args::Args;
use std::process::Command;

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
