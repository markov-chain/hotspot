//! Construction of thermal RC circuits for multiprocessor systems based on the block model of
//! [HotSpot][1].
//!
//! [1]: http://lava.cs.virginia.edu/HotSpot/

extern crate libc;

use std::ffi::CString;
use std::path::Path;

mod raw;

/// A thermal RC circuit.
pub struct Circuit {
    /// The number of active thermal nodes (processing elements).
    pub cores: usize,
    /// The number of thermal nodes, which is `4 * cores + 12`.
    pub nodes: usize,
    /// An `nodes`-element vector of thermal capacitance.
    pub capacitance: Vec<f64>,
    /// An `nodes`-by-`nodes` matrix of thermal conductance.
    pub conductance: Vec<f64>,
}

impl Circuit {
    /// Create a thermal RC circuit based on the block HotSpot model.
    ///
    /// The result is an equivalent thermal RC circuit constructed according to the block HotSpot
    /// model for the given floorplan file, configuration file, and parameter line. The parameter
    /// line bears the same meaning as the command-line arguments of the HotSpot tool. The names of
    /// parameters should not include dashes in front of them; for instance, `params` can be
    /// `"t_chip 0.00015 k_chip 100.0"`.
    ///
    /// It is important to note that the function relies on the original HotSpot library written in
    /// C. This library calls `exit(1)` whenever an input argument is invalid, which immediately
    /// terminates the calling program. Make sure all the input files exist.
    pub fn new(floorplan: &Path, config: &Path, params: &str) -> Result<Circuit, &'static str> {
        use std::iter::repeat;
        use std::ptr::copy_nonoverlapping as copy;

        macro_rules! str_to_c_str(
            ($str:expr) => (
                match CString::new($str) {
                    Ok(result) => result,
                    Err(_) => return Err("failed to process the arguments"),
                }
            );
        );

        macro_rules! path_to_c_str(
            ($path:expr) => (
                match $path.to_str() {
                    Some(path) => str_to_c_str!(path),
                    None => return Err("failed to process the arguments"),
                }
            );
        );

        unsafe {
            let floorplan = path_to_c_str!(floorplan);
            let config = path_to_c_str!(config);
            let params = str_to_c_str!(params);

            let c_circuit = raw::new_circuit(floorplan.as_ptr(), config.as_ptr(), params.as_ptr());
            if c_circuit.is_null() {
                return Err("failed to construct a thermal circuit");
            }

            let nc = (*c_circuit).nodes as usize;

            let mut circuit = Circuit {
                cores: (*c_circuit).cores as usize,
                nodes: nc,
                capacitance: repeat(0.0).take(nc).collect::<Vec<_>>(),
                conductance: repeat(0.0).take(nc * nc).collect::<Vec<_>>(),
            };

            copy((*c_circuit).capacitance as *const _, circuit.capacitance.as_mut_ptr(), nc);
            copy((*c_circuit).conductance as *const _, circuit.conductance.as_mut_ptr(), nc * nc);

            raw::free_circuit(c_circuit);

            Ok(circuit)
        }
    }
}
