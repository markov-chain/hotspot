use libc::{c_char, c_double, size_t};

#[repr(C)]
pub struct Circuit {
    pub cores: size_t,
    pub nodes: size_t,
    pub capacitance: *mut c_double,
    pub conductance: *mut c_double,
}

extern {
    pub fn new_circuit(floorplan: *const c_char, config: *const c_char,
                       params: *const c_char) -> *mut Circuit;
    pub fn free_circuit(circuit: *mut Circuit);
}
