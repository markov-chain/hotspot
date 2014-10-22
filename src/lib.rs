//! An interface to [HotSpot][1].
//!
//! [1]: http://lava.cs.virginia.edu/HotSpot/

extern crate libc;

use libc::{c_char, c_double, size_t};

#[repr(C)]
struct CCircuit {
    cores: size_t,
    nodes: size_t,
    capacitance: *mut c_double,
    conductance: *mut c_double,
}

#[link(name = "hotspot", kind = "static")]
extern {
    fn new_circuit(floorplan: *const c_char, config: *const c_char,
                   params: *const c_char) -> *mut CCircuit;
    fn free_circuit(circuit: *mut CCircuit);
}

/// A thermal RC circuit based on the block HotSpot model.
pub struct Circuit {
    /// The number of cores (active thermal nodes).
    pub cores: uint,
    /// The number of thermal nodes, which is `4 * cores + 12`.
    pub nodes: uint,
    /// An `nodes`-element vector of thermal capacitance.
    pub capacitance: Vec<f64>,
    /// An `nodes`-by-`nodes` matrix of thermal conductance.
    pub conductance: Vec<f64>,
}

impl Circuit {
    /// Creates a thermal RC circuit based on the block HotSpot model.
    ///
    /// The result is an equivalent thermal RC circuit constructed according to
    /// the block HotSpot model for the given floorplan file, configuration
    /// file, and parameter line. The parameter line bears the same meaning as
    /// the command-line arguments of the HotSpot tool. The names of parameters
    /// should not include dashes in front of them; for instance, `params` can
    /// be `"t_chip 0.00015 k_chip 100.0"`.
    pub fn new(floorplan: &str, config: &str, params: &str) -> Result<Circuit, ()> {
        unsafe {
            let c_circuit = new_circuit(floorplan.to_c_str().as_ptr(),
                                        config.to_c_str().as_ptr(),
                                        params.to_c_str().as_ptr());
            if c_circuit.is_null() {
                return Err(());
            }

            let nodes = (*c_circuit).nodes as uint;

            let mut circuit = Circuit {
                cores: (*c_circuit).cores as uint,
                nodes: nodes,
                capacitance: Vec::from_elem(nodes, 0.0),
                conductance: Vec::from_elem(nodes * nodes, 0.0),
            };

            std::ptr::copy_nonoverlapping_memory(circuit.capacitance.as_mut_ptr(),
                                                 (*c_circuit).capacitance as *const f64,
                                                 nodes);
            std::ptr::copy_nonoverlapping_memory(circuit.conductance.as_mut_ptr(),
                                                 (*c_circuit).conductance as *const f64,
                                                 nodes * nodes);
            free_circuit(c_circuit);

            Ok(circuit)
        }
    }
}
