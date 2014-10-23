extern crate test;
extern crate hotspot;

use std::io::fs::PathExtensions;

use hotspot::Circuit;

#[bench]
fn new(bench: &mut test::Bencher) {
    let floorplan = find_fixture("032.flp");
    let config = find_fixture("hotspot.config");
    {
        let floorplan = floorplan.as_str().unwrap();
        let config = config.as_str().unwrap();

        bench.iter(|| {
            Circuit::new(floorplan, config, "").unwrap()
        });
    }
}

fn find_fixture(name: &'static str) -> Path {
    let path = Path::new("fixtures").join(name);
    assert!(path.exists());
    path
}
