// #![allow(unused_imports)]
// use crate::myarch::*;
// use crate::simd::*;

// #[target_feature(enable = "sse2")]
// pub unsafe fn get_m128d(a: f64x2, idx: usize) -> f64 {
//     crate::mem::transmute(crate::myarch::get_m128d(crate::mem::transmute(a), crate::mem::transmute(idx)))
// }

// #[target_feature(enable = "sse")]
// pub unsafe fn get_m128(a: f32x4, idx: usize) -> f32 {
//     crate::mem::transmute(crate::myarch::get_m128(crate::mem::transmute(a), crate::mem::transmute(idx)))
// }

// // not actually an intrinsic but useful in various tests as we proted from
// // `i64x2::new` which is backwards from `_mm_set_epi64x`
// #[target_feature(enable = "sse2")]
// pub unsafe fn _mm_setr_epi64x(a: i64, b: i64) -> i64x2 {
//     crate::mem::transmute(crate::myarch::_mm_setr_epi64x(crate::mem::transmute(a), crate::mem::transmute(b)))
// }

// #[target_feature(enable = "avx")]
// pub unsafe fn get_m256d(a: f64x4, idx: usize) -> f64 {
//     crate::mem::transmute(crate::myarch::get_m256d(crate::mem::transmute(a), crate::mem::transmute(idx)))
// }

// #[target_feature(enable = "avx")]
// pub unsafe fn get_m256(a: f32x8, idx: usize) -> f32 {
//     crate::mem::transmute(crate::myarch::get_m256(crate::mem::transmute(a), crate::mem::transmute(idx)))
// }
