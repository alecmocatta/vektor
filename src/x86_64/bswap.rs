#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Return an integer with the reversed byte order of x
#[inline]
#[cfg_attr(test, assert_instr(bswap))]
pub unsafe fn _bswap64(x: i64) -> i64 {
    crate::mem::transmute(crate::myarch::_bswap64(crate::mem::transmute(x)))
}

