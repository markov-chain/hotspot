#![feature(test)]

extern crate test;
extern crate hotspot;

use hotspot::Circuit;

#[allow(dead_code)]
#[path="../tests/fixture.rs"]
mod fixture;

#[bench]
fn new(bench: &mut test::Bencher) {
    let (floorplan, config) = (fixture::find("032.flp"), fixture::find("hotspot.config"));

    bench.iter(|| {
        Circuit::new(&floorplan, &config, "").unwrap()
    });
}
