#![feature(phase)]

#[phase(plugin)] extern crate assert;

extern crate hotspot;

use hotspot::Circuit;

#[test]
fn new() {
    let (floorplan, config) = (find_fixture("002.flp"), find_fixture("hotspot.config"));
    let circuit = Circuit::new(floorplan.as_str().unwrap(),
                               config.as_str().unwrap(), "").unwrap();

    assert_eq!(circuit.cores, 2);
    assert_eq!(circuit.nodes, 20);
    assert_close!(circuit.capacitance, fixture::capacitance);
    assert_close!(circuit.conductance, fixture::conductance);
}

fn find_fixture(name: &str) -> Path {
    use std::io::fs::PathExtensions;
    let path = Path::new("tests").join_many(["fixtures", name]);
    assert!(path.exists());
    path
}

#[allow(non_upper_case_globals)]
mod fixture {
    pub const capacitance: [f64, ..20] = [
        3.496500000000000e-04, 3.496500000000000e-04, 1.065600000000000e-04,
        1.065600000000000e-04, 4.728600000000000e-03, 4.728600000000000e-03,
        8.457534000000000e-02, 8.457534000000000e-02, 2.458872000000000e-01,
        2.458872000000000e-01, 2.813517000000000e-01, 2.813517000000000e-01,
        4.397917680000000e+00, 4.397917680000000e+00, 5.032232730000000e+00,
        5.032232730000000e+00, 1.427208862500000e+01, 1.427208862500000e+01,
        1.427208862500000e+01, 1.427208862500000e+01,
    ];

    pub const conductance: [f64, ..400] = [
        2.681666666666667e+00, -1.500000000000000e-02, -2.666666666666667e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00, -1.500000000000000e-02,
        2.681666666666667e+00,  0.000000000000000e+00, -2.666666666666667e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00, -2.666666666666667e+00,  0.000000000000000e+00,
        3.466746666666666e+00, -8.000000000000001e-05, -7.999999999999998e-01,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00, -2.666666666666667e+00, -8.000000000000001e-05,
        3.466746666666666e+00,  0.000000000000000e+00, -7.999999999999998e-01,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00, -7.999999999999998e-01,  0.000000000000000e+00,
        3.563636363636363e+00, -3.999999999999999e-01, -1.600000000000000e+00,
        0.000000000000000e+00, -3.272727272727273e-01,  0.000000000000000e+00,
       -2.181818181818182e-01, -2.181818181818182e-01,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00, -7.999999999999998e-01, -3.999999999999999e-01,
        3.563636363636363e+00,  0.000000000000000e+00, -1.600000000000000e+00,
        0.000000000000000e+00, -3.272727272727273e-01, -2.181818181818182e-01,
       -2.181818181818182e-01,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00, -1.600000000000000e+00,  0.000000000000000e+00,
        9.639693957467317e+00, -2.760000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
       -2.258181818181818e+00,  0.000000000000000e+00, -1.505454545454545e+00,
       -1.505454545454545e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00, -1.600000000000000e+00, -2.760000000000000e+00,
        9.639693957467317e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
       -2.258181818181818e+00, -1.505454545454545e+00, -1.505454545454545e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00, -3.272727272727273e-01,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        8.352727272727272e+01,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00, -8.319999999999999e+01,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00, -3.272727272727273e-01,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        8.352727272727272e+01,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00, -8.319999999999999e+01,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
       -2.181818181818182e-01, -2.181818181818182e-01,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        9.563636363636363e+01,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00, -9.519999999999999e+01,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00, -2.181818181818182e-01,
       -2.181818181818182e-01,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        9.563636363636363e+01,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00, -9.519999999999999e+01,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
       -2.258181818181818e+00,  0.000000000000000e+00, -8.319999999999999e+01,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        9.172845925267396e+01,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00, -5.718918918918919e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
       -2.258181818181818e+00,  0.000000000000000e+00, -8.319999999999999e+01,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        9.172845925267396e+01,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00, -5.718918918918919e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00, -1.505454545454545e+00, -1.505454545454545e+00,
        0.000000000000000e+00,  0.000000000000000e+00, -9.519999999999999e+01,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        1.043853802128951e+02,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00, -5.543589743589744e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
       -1.505454545454545e+00, -1.505454545454545e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00, -9.519999999999999e+01,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        1.043853802128951e+02,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00, -5.543589743589744e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00, -5.718918918918919e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        7.508183332437806e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00, -5.718918918918919e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        7.508183332437806e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00, -5.543589743589744e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        7.332854157108631e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        0.000000000000000e+00,  0.000000000000000e+00, -5.543589743589744e+00,
        0.000000000000000e+00,  0.000000000000000e+00,  0.000000000000000e+00,
        7.332854157108631e+00,
    ];
}
