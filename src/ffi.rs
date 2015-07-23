use libc::{c_char, c_double, size_t};

#[repr(C)]
pub struct Circuit {
    pub units: size_t,
    pub nodes: size_t,
    pub capacitance: *mut c_double,
    pub conductance: *mut c_double,
}

extern {
    pub fn new_Circuit(floorplan: *const c_char, config: *const c_char) -> *mut Circuit;
    pub fn drop_Circuit(circuit: *mut Circuit);
}
