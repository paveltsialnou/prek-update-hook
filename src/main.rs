use std::process::{exit, Command};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut fix = false;
    let mut exclude_repos: Vec<String> = Vec::new();
    let mut passthrough: Vec<String> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--fix" | "-f" => fix = true,
            "--help" | "-h" => {
                print_help();
                exit(0);
            }
            "--exclude-repo" => {
                i += 1;
                if i < args.len() {
                    exclude_repos.push(args[i].clone());
                } else {
                    eprintln!("error: --exclude-repo requires a value");
                    exit(2);
                }
            }
            arg if arg.starts_with("--exclude-repo=") => {
                if let Some(val) = arg.strip_prefix("--exclude-repo=") {
                    exclude_repos.push(val.to_string());
                }
            }
            _ => passthrough.push(args[i].clone()),
        }
        i += 1;
    }

    let mut cmd = Command::new("prek");
    cmd.arg("update");
    if !fix {
        cmd.arg("--check");
    }
    for repo in &exclude_repos {
        cmd.arg("--exclude-repo").arg(repo);
    }
    cmd.args(&passthrough);

    let status = cmd.status().unwrap_or_else(|e| {
        eprintln!("error: failed to run prek: {e}");
        exit(2);
    });

    exit(status.code().unwrap_or(1));
}

fn print_help() {
    println!(
        "prek-update-hook: check or update hook repositories pinned in the prek config.\n\
         \n\
         Usage: prek-update-hook [OPTIONS]\n\
         \n\
         Options:\n\
         \x20 --fix, -f              Apply available updates instead of only checking\n\
         \x20 --exclude-repo=<REPO>  Skip this repo during --fix (may be repeated)\n\
         \x20 -h, --help             Print help\n\
         \n\
         All other options are forwarded to `prek update`."
    );
}
