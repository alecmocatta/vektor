#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Read a hardware generated 64-bit random value and store the result in val.
/// Return 1 if a random value was generated, and 0 otherwise.
#[inline]
#[target_feature(enable = "rdrand")]
#[cfg_attr(test, assert_instr(rdrand))]
#[cfg_attr(feature = "cargo-clippy", allow(stutter))]
pub unsafe fn _rdrand64_step(val: &mut u64) -> i32 {
    crate::mem::transmute(crate::myarch::_rdrand64_step(crate::mem::transmute(val)))
}

/// Read a 64-bit NIST SP800-90B and SP800-90C compliant random value and store
/// in val. Return 1 if a random value was generated, and 0 otherwise.
#[inline]
#[target_feature(enable = "rdseed")]
#[cfg_attr(test, assert_instr(rdseed))]
pub unsafe fn _rdseed64_step(val: &mut u64) -> i32 {
    crate::mem::transmute(crate::myarch::_rdseed64_step(crate::mem::transmute(val)))
}

