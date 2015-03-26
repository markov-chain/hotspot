#![feature(convert, test, path_ext)]

extern crate test;

extern crate hotspot;

use std::path::PathBuf;

use hotspot::Circuit;

#[bench]
fn new(bench: &mut test::Bencher) {
    let floorplan = find_fixture("032.flp");
    let config = find_fixture("hotspot.config");

    bench.iter(|| {
        Circuit::new(&floorplan, &config, "").unwrap()
    });
}

fn find_fixture(name: &str) -> PathBuf {
    use std::fs::PathExt;
    let path = PathBuf::from("tests").join("fixtures").join(name);
    assert!(path.exists());
    path
}
