#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Return an integer with the reversed byte order of x
#[inline]
#[cfg_attr(test, assert_instr(bswap))]
pub unsafe fn _bswap(x: i32) -> i32 {
    crate::mem::transmute(crate::myarch::_bswap(crate::mem::transmute(x)))
}

