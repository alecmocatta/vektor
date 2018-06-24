#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Reads the current value of the processor’s time-stamp counter.
///
/// The processor monotonically increments the time-stamp counter MSR
/// every clock cycle and resets it to 0 whenever the processor is
/// reset.
///
/// The RDTSC instruction is not a serializing instruction. It does
/// not necessarily wait until all previous instructions have been
/// executed before reading the counter. Similarly, subsequent
/// instructions may begin execution before the read operation is
/// performed.
///
/// On processors that support the Intel 64 architecture, the
/// high-order 32 bits of each of RAX and RDX are cleared.
#[inline]
#[cfg_attr(test, assert_instr(rdtsc))]
pub unsafe fn _rdtsc() -> i64 {
    crate::mem::transmute(crate::myarch::_rdtsc())
}

/// Reads the current value of the processor’s time-stamp counter and
/// the `IA32_TSC_AUX MSR`.
///
/// The processor monotonically increments the time-stamp counter MSR
/// every clock cycle and resets it to 0 whenever the processor is
/// reset.
///
/// The RDTSCP instruction waits until all previous instructions have
/// been executed before reading the counter. However, subsequent
/// instructions may begin execution before the read operation is
/// performed.
///
/// On processors that support the Intel 64 architecture, the
/// high-order 32 bits of each of RAX, RDX, and RCX are cleared.
#[inline]
#[cfg_attr(test, assert_instr(rdtscp))]
pub unsafe fn __rdtscp(aux: *mut u32) -> u64 {
    crate::mem::transmute(crate::myarch::__rdtscp(crate::mem::transmute(aux)))
}

