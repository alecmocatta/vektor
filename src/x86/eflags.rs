#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Reads EFLAGS.
#[cfg(target_arch = "x86")]
#[inline(always)]
pub unsafe fn __readeflags() -> u32 {
    crate::mem::transmute(crate::myarch::__readeflags())
}

/// Reads EFLAGS.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __readeflags() -> u64 {
    crate::mem::transmute(crate::myarch::__readeflags())
}

