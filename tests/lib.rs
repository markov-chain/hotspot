extern crate assert;
extern crate hotspot;

use hotspot::Circuit;

mod fixture;

#[test]
fn new() {
    let (floorplan, config) = (fixture::find("002.flp"), fixture::find("hotspot.config"));
    let circuit = Circuit::new(&floorplan, &config, "").unwrap();

    assert_eq!(circuit.cores, 2);
    assert_eq!(circuit.nodes, 20);
    assert::within(&circuit.capacitance, &fixture::C, 1e-13);
    assert::within(&circuit.capacitance, &fixture::C, 1e-13);
    assert::within(&circuit.conductance, &fixture::G, 1e-13);
}

#[test]
fn new_with_params() {
    let (floorplan, config) = (fixture::find("002.flp"), fixture::find("hotspot.config"));
    let circuit = Circuit::new(&floorplan, &config, "t_chip 0.00042 k_chip 42").unwrap();

    assert::within(&circuit.capacitance, &fixture::C42, 1e-13);
    assert::within(&circuit.conductance, &fixture::G42, 1e-13);
}
