extern crate assert;
extern crate hotspot;
extern crate matrix;

use hotspot::Circuit;
use matrix::format::Conventional;

mod fixture;

#[test]
fn new() {
    let circuit = Circuit::new(fixture::find("002.flp"), fixture::find("hotspot.config")).unwrap();

    assert_eq!(circuit.units, 2);
    assert_eq!(circuit.nodes, 20);
    assert::close(&*circuit.capacitance, &fixture::C[..], 1e-13);
    assert::close(&*Conventional::from(&circuit.conductance), &fixture::G[..], 1e-13);
    assert_eq!(circuit.conductance.nonzeros, 80);
}
