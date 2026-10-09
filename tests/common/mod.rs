#![allow(dead_code)] // each integration test target includes this module but uses only some helpers

use std::process::Command;

pub fn args_from_str(s: &str) -> Vec<String> {
    let mut v: Vec<String> = vec!["prek-update-hook".into()];
    v.extend(s.split_whitespace().map(String::from));
    v
}

pub fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_prek-update-hook"))
}
