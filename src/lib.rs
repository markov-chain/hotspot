//! An interface to [HotSpot][1].
//!
//! [1]: http://lava.cs.virginia.edu/HotSpot/

extern crate libc;

mod raw;

/// A thermal RC circuit.
pub struct Circuit {
    /// The number of active thermal nodes (processing elements).
    pub cores: uint,
    /// The number of thermal nodes, which is `4 * cores + 12`.
    pub nodes: uint,
    /// An `nodes`-element vector of thermal capacitance.
    pub capacitance: Vec<f64>,
    /// An `nodes`-by-`nodes` matrix of thermal conductance.
    pub conductance: Vec<f64>,
}

impl Circuit {
    /// Create a thermal RC circuit based on the block HotSpot model.
    ///
    /// The result is an equivalent thermal RC circuit constructed according to
    /// the block HotSpot model for the given floorplan file, configuration
    /// file, and parameter line. The parameter line bears the same meaning as
    /// the command-line arguments of the HotSpot tool. The names of parameters
    /// should not include dashes in front of them; for instance, `params` can
    /// be `"t_chip 0.00015 k_chip 100.0"`.
    ///
    /// It is important to note that the function relies on the original HotSpot
    /// library written in C. This library calls `exit(1)` whenever an input
    /// argument is invalid, which immediately terminates the calling program.
    /// Make sure all the input files exist.
    pub fn new(floorplan: &Path, config: &Path, params: &str) -> Result<Circuit, &'static str> {
        use std::ptr::copy_nonoverlapping_memory as copy;

        unsafe {
            let raw_circuit = raw::new_circuit(floorplan.to_c_str().as_ptr(),
                                               config.to_c_str().as_ptr(),
                                               params.to_c_str().as_ptr());
            if raw_circuit.is_null() {
                return Err("HotSpot failed to construct a thermal circuit");
            }

            let nc = (*raw_circuit).nodes as uint;

            let mut circuit = Circuit {
                cores: (*raw_circuit).cores as uint,
                nodes: nc,
                capacitance: Vec::from_elem(nc, 0.0),
                conductance: Vec::from_elem(nc * nc, 0.0),
            };

            copy(circuit.capacitance.as_mut_ptr(),
                 (*raw_circuit).capacitance as *const f64, nc);
            copy(circuit.conductance.as_mut_ptr(),
                 (*raw_circuit).conductance as *const f64, nc * nc);

            raw::free_circuit(raw_circuit);

            Ok(circuit)
        }
    }
}
