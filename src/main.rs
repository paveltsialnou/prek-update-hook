use std::process::{exit, Command};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut fix = false;
    let mut passthrough: Vec<String> = Vec::new();

    for arg in &args {
        match arg.as_str() {
            "--fix" | "-f" => fix = true,
            "--help" | "-h" => {
                print_help();
                exit(0);
            }
            _ => passthrough.push(arg.clone()),
        }
    }

    let mut cmd = Command::new("prek");
    cmd.arg("update");
    if !fix {
        cmd.arg("--check");
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
         \x20 --fix, -f   Apply available updates instead of only checking\n\
         \x20 -h, --help  Print help\n\
         \n\
         All other options are forwarded to `prek update`."
    );
}
