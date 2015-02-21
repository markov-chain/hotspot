#![feature(env, path, process)]

use std::{env, process};
use std::path::PathBuf;

macro_rules! cmd(
    ($name:expr) => (process::Command::new($name));
);

macro_rules! get(
    ($name:expr) => (env::var($name).unwrap_or("".to_string()));
);

macro_rules! run(
    ($command:expr) => (
        assert!($command.stdout(process::Stdio::inherit())
                        .stderr(process::Stdio::inherit())
                        .status().unwrap().success());
    );
);

fn main() {
    let mut build = PathBuf::new(&get!("CARGO_MANIFEST_DIR"));
    build.push("build");

    let into = PathBuf::new(&get!("OUT_DIR"));

    run!(cmd!("make").current_dir(&build));

    println!("cargo:rustc-flags=-L {:}", into.to_str().unwrap());
}
