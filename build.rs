use std::{env, process};
use std::path::PathBuf;

macro_rules! cmd(
    ($name:expr) => (process::Command::new($name));
);

macro_rules! get(
    ($name:expr) => (env::var($name).unwrap_or(String::new()));
);

macro_rules! run(
    ($command:expr) => (
        assert!($command.stdout(process::Stdio::inherit())
                        .stderr(process::Stdio::inherit())
                        .status().unwrap().success());
    );
);

fn main() {
    let build = PathBuf::from(&get!("CARGO_MANIFEST_DIR")).join("build");
    let into = PathBuf::from(&get!("OUT_DIR"));

    run!(cmd!("make").current_dir(&build));

    println!("cargo:rustc-link-lib=static=circuit");
    println!("cargo:rustc-link-search={}", into.display());
}
