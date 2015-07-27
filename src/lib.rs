//! Interface to [HotSpot][1].
//!
//! [1]: http://lava.cs.virginia.edu/HotSpot

extern crate libc;
extern crate matrix;

use matrix::format::{Compressed, Conventional, Diagonal};
use std::ffi::CString;
use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

mod ffi;

macro_rules! raise(
    ($kind:ident, $message:expr) => (
        return Err(Error::new(ErrorKind::$kind, $message))
    );
);

macro_rules! str_to_cstr(
    ($str:expr) => (match CString::new($str) {
        Ok(result) => result,
        _ => raise!(Other, "failed to process the arguments"),
    });
);

macro_rules! path_to_cstr(
    ($path:expr) => (match $path.to_str() {
        Some(path) => str_to_cstr!(path),
        _ => raise!(Other, "failed to process the arguments"),
    });
);

/// A thermal circuit.
pub struct Circuit {
    /// The number of processing elements.
    pub units: usize,
    /// The number of thermal nodes.
    pub nodes: usize,
    /// The thermal capacitance matrix.
    pub capacitance: Diagonal<f64>,
    /// The thermal conductance matrix.
    pub conductance: Compressed<f64>,
}

impl Circuit {
    /// Construct a thermal circuit.
    ///
    /// The only supported model is the block model.
    pub fn new<F: AsRef<Path>, C: AsRef<Path>>(floorplan: F, config: C) -> Result<Circuit> {
        use std::slice::from_raw_parts;

        let (floorplan, config) = (floorplan.as_ref(), config.as_ref());
        if fs::metadata(floorplan).is_err() {
            raise!(NotFound, "the floorplan file does not exist");
        }
        if fs::metadata(config).is_err() {
            raise!(NotFound, "the configuration file does not exist");
        }

        unsafe {
            let floorplan = path_to_cstr!(floorplan);
            let config = path_to_cstr!(config);

            let circuit = ffi::new_Circuit(floorplan.as_ptr(), config.as_ptr());
            if circuit.is_null() {
                raise!(Other, "failed to construct a thermal circuit");
            }

            let circuit = &*circuit;

            let units = circuit.units as usize;
            let nodes = circuit.nodes as usize;

            let capacitance = from_raw_parts(circuit.capacitance as *const _, nodes);
            let capacitance = Diagonal::from_slice(nodes, capacitance);

            let conductance = from_raw_parts(circuit.conductance as *const _, nodes * nodes);
            let conductance = Conventional::from_slice(nodes, conductance);
            let conductance = Compressed::from(conductance);

            ffi::drop_Circuit(circuit as *const _ as *mut _);

            Ok(Circuit {
                units: units,
                nodes: nodes,
                capacitance: capacitance,
                conductance: conductance,
            })
        }
    }
}
