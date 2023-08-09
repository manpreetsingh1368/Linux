#![allow(non_camel_case_types)]

pub const _ALPHA_AGP_BACKEND_H: u32 = 1;

pub union AlphaAgpMode {
    bits: AlphaAgpModeBits,
    lw: u32,
}

pub struct AlphaAgpInfo {
    hose: *mut PciController,
    aperture: AlphaAperture,
    capability: AlphaAgpMode,
    mode: AlphaAgpMode,
    private: *mut std::ffi::c_void,
    ops: *mut AlphaAgpOps,
}

pub struct AlphaAgpOps {
    setup: Option<extern "C" fn(*mut AlphaAgpInfo) -> i32>,
    cleanup: Option<extern "C" fn(*mut AlphaAgpInfo)>,
    configure: Option<extern "C" fn(*mut AlphaAgpInfo) -> i32>,
    bind: Option<extern "C" fn(*mut AlphaAgpInfo, libc::off_t, *mut AgpMemory) -> i32>,
    unbind: Option<extern "C" fn(*mut AlphaAgpInfo, libc::off_t, *mut AgpMemory) -> i32>,
    translate: Option<extern "C" fn(*mut AlphaAgpInfo, dma_addr_t) -> libc::c_ulong>,
}

#[repr(C)]
pub struct AlphaAgpModeBits {
    rate: u32,
    reserved0: u32,
    fw: u32,
    fourgb: u32,
    reserved1: u32,
    enable: u32,
    sba: u32,
    reserved2: u32,
    rq: u32,
}

#[repr(C)]
pub struct AlphaAperture {
    bus_base: dma_addr_t,
    size: libc::c_ulong,
    sysdata: *mut std::ffi::c_void,
}

pub type dma_addr_t = libc::c_ulong;

// Placeholder struct definitions
pub struct AgpMemory {
    // Define the struct fields here as needed
}

pub struct PciController {
    // Define the struct fields here as needed
}
//Please replace the placeholders (AgpMemory and PciController) with the actual definitions from your codebase (require more enhancement here)
