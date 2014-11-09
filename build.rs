#![feature(macro_rules)]

macro_rules! cmd(
    ($name:expr) => (::std::io::process::Command::new($name));
)

macro_rules! get(
    ($name:expr) => (::std::os::getenv($name).unwrap_or("".to_string()));
)

macro_rules! run(
    ($command:expr) => (
        assert!($command.stdout(::std::io::process::InheritFd(1))
                        .stderr(::std::io::process::InheritFd(2))
                        .status().unwrap().success());
    );
)

fn main() {
    let build = Path::new(get!("CARGO_MANIFEST_DIR")).join("build");
    let into = Path::new(get!("OUT_DIR"));

    run!(cmd!("make").cwd(&build));

    println!("cargo:rustc-flags=-L {}", into.display());
    println!("cargo:rustc-flags=-l circuit:static");
}
