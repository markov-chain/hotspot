extern crate assert;
extern crate hotspot;
extern crate matrix;

use hotspot::Circuit;
use matrix::Dense;

mod fixture;

#[test]
fn new() {
    let circuit = Circuit::new(fixture::find("002.flp"), fixture::find("hotspot.config")).unwrap();

    assert_eq!(circuit.cores, 2);
    assert_eq!(circuit.nodes, 20);
    assert::close(&*circuit.capacitance, &fixture::C[..], 1e-13);
    assert::close(&*Dense::from(&circuit.conductance), &fixture::G[..], 1e-13);
    assert_eq!(circuit.conductance.nonzeros, 80);
}
