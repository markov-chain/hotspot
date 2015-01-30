#![feature(io, path)]

use std::old_io as io;
use std::os;

macro_rules! cmd(
    ($name:expr) => (io::process::Command::new($name));
);

macro_rules! get(
    ($name:expr) => (os::getenv($name).unwrap_or("".to_string()));
);

macro_rules! run(
    ($command:expr) => (
        assert!($command.stdout(io::process::InheritFd(1))
                        .stderr(io::process::InheritFd(2))
                        .status().unwrap().success());
    );
);

fn main() {
    let build = Path::new(get!("CARGO_MANIFEST_DIR")).join("build");
    let into = Path::new(get!("OUT_DIR"));

    run!(cmd!("make").cwd(&build));

    println!("cargo:rustc-flags=-L {}", into.display());
}
