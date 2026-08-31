use prek_update_hook::{run, ExitCode};
use std::process::exit;

fn main() {
    match run(std::env::args()) {
        ExitCode::Success => exit(0),
        ExitCode::UpdateNeeded => exit(1),
        ExitCode::Error(msg) => {
            eprintln!("{msg}");
            exit(2);
        }
    }
}
