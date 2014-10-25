extern crate test;
extern crate hotspot;

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

fn find_fixture(name: &str) -> Path {
    use std::io::fs::PathExtensions;
    let path = Path::new("tests").join_many(["fixtures", name]);
    assert!(path.exists());
    path
}
