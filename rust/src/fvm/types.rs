use std::sync::Mutex;

use safer_ffi::prelude::*;

use super::machine::CgoExecutor;

#[derive_ReprC]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FvmRegisteredVersion {
    V1,
}

#[derive_ReprC]
#[ReprC::opaque]
#[derive(Default)]
pub struct FvmMachine {
    pub(crate) machine: Option<Mutex<CgoExecutor>>,
}

#[derive_ReprC]
#[repr(C)]
#[derive(Default)]
pub struct FvmMachineExecuteResponse {
    pub exit_code: u64,
    pub return_val: Option<c_slice::Box<u8>>,
    pub gas_used: u64,
    pub penalty_hi: u64,
    pub penalty_lo: u64,
    pub miner_tip_hi: u64,
    pub miner_tip_lo: u64,
}
