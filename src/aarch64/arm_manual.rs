//! ARMv7 NEON intrinsics

use super::*;

type p8 = u8;
type p16 = u16;
type p64 = u64;
type p128 = u128;

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1_lane_s8<const LANE: i32>(ptr: &i8, src: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vld1_lane_s8::<LANE>(ptr, src) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1q_lane_s8<const LANE: i32>(ptr: &i8, src: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vld1q_lane_s8::<LANE>(ptr, src) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1_lane_s16<const LANE: i32>(ptr: &i16, src: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vld1_lane_s16::<LANE>(ptr, src) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1q_lane_s16<const LANE: i32>(ptr: &i16, src: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vld1q_lane_s16::<LANE>(ptr, src) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1_lane_s32<const LANE: i32>(ptr: &i32, src: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vld1_lane_s32::<LANE>(ptr, src) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1q_lane_s32<const LANE: i32>(ptr: &i32, src: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vld1q_lane_s32::<LANE>(ptr, src) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1_lane_s64<const LANE: i32>(ptr: &i64, src: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vld1_lane_s64::<LANE>(ptr, src) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1q_lane_s64<const LANE: i32>(ptr: &i64, src: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vld1q_lane_s64::<LANE>(ptr, src) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1_lane_u8<const LANE: i32>(ptr: &u8, src: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vld1_lane_u8::<LANE>(ptr, src) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1q_lane_u8<const LANE: i32>(ptr: &u8, src: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vld1q_lane_u8::<LANE>(ptr, src) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1_lane_u16<const LANE: i32>(ptr: &u16, src: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vld1_lane_u16::<LANE>(ptr, src) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1q_lane_u16<const LANE: i32>(ptr: &u16, src: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vld1q_lane_u16::<LANE>(ptr, src) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1_lane_u32<const LANE: i32>(ptr: &u32, src: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vld1_lane_u32::<LANE>(ptr, src) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1q_lane_u32<const LANE: i32>(ptr: &u32, src: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vld1q_lane_u32::<LANE>(ptr, src) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1_lane_u64<const LANE: i32>(ptr: &u64, src: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vld1_lane_u64::<LANE>(ptr, src) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1q_lane_u64<const LANE: i32>(ptr: &u64, src: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vld1q_lane_u64::<LANE>(ptr, src) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1_lane_p8<const LANE: i32>(ptr: &p8, src: poly8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vld1_lane_p8::<LANE>(ptr, src) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1q_lane_p8<const LANE: i32>(ptr: &p8, src: poly8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vld1q_lane_p8::<LANE>(ptr, src) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1_lane_p16<const LANE: i32>(ptr: &p16, src: poly16x4_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vld1_lane_p16::<LANE>(ptr, src) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1q_lane_p16<const LANE: i32>(ptr: &p16, src: poly16x8_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vld1q_lane_p16::<LANE>(ptr, src) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon,aes")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1_lane_p64<const LANE: i32>(ptr: &p64, src: poly64x1_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vld1_lane_p64::<LANE>(ptr, src) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon,aes")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1q_lane_p64<const LANE: i32>(ptr: &p64, src: poly64x2_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vld1q_lane_p64::<LANE>(ptr, src) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1_lane_f32<const LANE: i32>(ptr: &f32, src: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vld1_lane_f32::<LANE>(ptr, src) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1q_lane_f32<const LANE: i32>(ptr: &f32, src: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vld1q_lane_f32::<LANE>(ptr, src) }
}

/// Load one single-element structure and Replicate to all lanes (of one
/// register).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1_dup_s8(ptr: &i8) -> int8x8_t {
  unsafe { core::arch::aarch64::vld1_dup_s8(ptr) }
}

/// Load one single-element structure and Replicate to all lanes (of one
/// register).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1q_dup_s8(ptr: &i8) -> int8x16_t {
  unsafe { core::arch::aarch64::vld1q_dup_s8(ptr) }
}

/// Load one single-element structure and Replicate to all lanes (of one
/// register).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1_dup_s16(ptr: &i16) -> int16x4_t {
  unsafe { core::arch::aarch64::vld1_dup_s16(ptr) }
}

/// Load one single-element structure and Replicate to all lanes (of one
/// register).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1q_dup_s16(ptr: &i16) -> int16x8_t {
  unsafe { core::arch::aarch64::vld1q_dup_s16(ptr) }
}

/// Load one single-element structure and Replicate to all lanes (of one
/// register).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1_dup_s32(ptr: &i32) -> int32x2_t {
  unsafe { core::arch::aarch64::vld1_dup_s32(ptr) }
}

/// Load one single-element structure and Replicate to all lanes (of one
/// register).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1q_dup_s32(ptr: &i32) -> int32x4_t {
  unsafe { core::arch::aarch64::vld1q_dup_s32(ptr) }
}

/// Load one single-element structure and Replicate to all lanes (of one
/// register).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1_dup_s64(ptr: &i64) -> int64x1_t {
  unsafe { core::arch::aarch64::vld1_dup_s64(ptr) }
}

/// Load one single-element structure and Replicate to all lanes (of one
/// register).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1q_dup_s64(ptr: &i64) -> int64x2_t {
  unsafe { core::arch::aarch64::vld1q_dup_s64(ptr) }
}

/// Load one single-element structure and Replicate to all lanes (of one
/// register).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1_dup_u8(ptr: &u8) -> uint8x8_t {
  unsafe { core::arch::aarch64::vld1_dup_u8(ptr) }
}

/// Load one single-element structure and Replicate to all lanes (of one
/// register).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1q_dup_u8(ptr: &u8) -> uint8x16_t {
  unsafe { core::arch::aarch64::vld1q_dup_u8(ptr) }
}

/// Load one single-element structure and Replicate to all lanes (of one
/// register).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1_dup_u16(ptr: &u16) -> uint16x4_t {
  unsafe { core::arch::aarch64::vld1_dup_u16(ptr) }
}

/// Load one single-element structure and Replicate to all lanes (of one
/// register).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1q_dup_u16(ptr: &u16) -> uint16x8_t {
  unsafe { core::arch::aarch64::vld1q_dup_u16(ptr) }
}

/// Load one single-element structure and Replicate to all lanes (of one
/// register).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1_dup_u32(ptr: &u32) -> uint32x2_t {
  unsafe { core::arch::aarch64::vld1_dup_u32(ptr) }
}

/// Load one single-element structure and Replicate to all lanes (of one
/// register).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1q_dup_u32(ptr: &u32) -> uint32x4_t {
  unsafe { core::arch::aarch64::vld1q_dup_u32(ptr) }
}

/// Load one single-element structure and Replicate to all lanes (of one
/// register).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1_dup_u64(ptr: &u64) -> uint64x1_t {
  unsafe { core::arch::aarch64::vld1_dup_u64(ptr) }
}

/// Load one single-element structure and Replicate to all lanes (of one
/// register).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1q_dup_u64(ptr: &u64) -> uint64x2_t {
  unsafe { core::arch::aarch64::vld1q_dup_u64(ptr) }
}

/// Load one single-element structure and Replicate to all lanes (of one
/// register).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1_dup_p8(ptr: &p8) -> poly8x8_t {
  unsafe { core::arch::aarch64::vld1_dup_p8(ptr) }
}

/// Load one single-element structure and Replicate to all lanes (of one
/// register).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1q_dup_p8(ptr: &p8) -> poly8x16_t {
  unsafe { core::arch::aarch64::vld1q_dup_p8(ptr) }
}

/// Load one single-element structure and Replicate to all lanes (of one
/// register).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1_dup_p16(ptr: &p16) -> poly16x4_t {
  unsafe { core::arch::aarch64::vld1_dup_p16(ptr) }
}

/// Load one single-element structure and Replicate to all lanes (of one
/// register).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1q_dup_p16(ptr: &p16) -> poly16x8_t {
  unsafe { core::arch::aarch64::vld1q_dup_p16(ptr) }
}

/// Load one single-element structure and Replicate to all lanes (of one
/// register).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1_dup_f32(ptr: &f32) -> float32x2_t {
  unsafe { core::arch::aarch64::vld1_dup_f32(ptr) }
}

/// Load one single-element structure and Replicate to all lanes (of one
/// register).
#[inline]
#[cfg(target_feature = "neon,aes")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1_dup_p64(ptr: &p64) -> poly64x1_t {
  unsafe { core::arch::aarch64::vld1_dup_p64(ptr) }
}

/// Load one single-element structure and Replicate to all lanes (of one
/// register).
#[inline]
#[cfg(target_feature = "neon,aes")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1q_dup_p64(ptr: &p64) -> poly64x2_t {
  unsafe { core::arch::aarch64::vld1q_dup_p64(ptr) }
}

/// Load one single-element structure and Replicate to all lanes (of one
/// register).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vld1q_dup_f32(ptr: &f32) -> float32x4_t {
  unsafe { core::arch::aarch64::vld1q_dup_f32(ptr) }
}

// signed absolute difference and accumulate (64-bit)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaba_s8(a: int8x8_t, b: int8x8_t, c: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vaba_s8(a, b, c) }
}
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaba_s16(a: int16x4_t, b: int16x4_t, c: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vaba_s16(a, b, c) }
}
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaba_s32(a: int32x2_t, b: int32x2_t, c: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vaba_s32(a, b, c) }
}
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaba_u8(a: uint8x8_t, b: uint8x8_t, c: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vaba_u8(a, b, c) }
}
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaba_u16(a: uint16x4_t, b: uint16x4_t, c: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vaba_u16(a, b, c) }
}
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaba_u32(a: uint32x2_t, b: uint32x2_t, c: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vaba_u32(a, b, c) }
}
// signed absolute difference and accumulate (128-bit)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vabaq_s8(a: int8x16_t, b: int8x16_t, c: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vabaq_s8(a, b, c) }
}
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vabaq_s16(a: int16x8_t, b: int16x8_t, c: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vabaq_s16(a, b, c) }
}
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vabaq_s32(a: int32x4_t, b: int32x4_t, c: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vabaq_s32(a, b, c) }
}
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vabaq_u8(a: uint8x16_t, b: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vabaq_u8(a, b, c) }
}
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vabaq_u16(a: uint16x8_t, b: uint16x8_t, c: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vabaq_u16(a, b, c) }
}
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vabaq_u32(a: uint32x4_t, b: uint32x4_t, c: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vabaq_u32(a, b, c) }
}

/// Absolute value (wrapping).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vabs_s8(a: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vabs_s8(a) }
}
/// Absolute value (wrapping).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vabs_s16(a: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vabs_s16(a) }
}
/// Absolute value (wrapping).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vabs_s32(a: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vabs_s32(a) }
}
/// Absolute value (wrapping).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vabsq_s8(a: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vabsq_s8(a) }
}
/// Absolute value (wrapping).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vabsq_s16(a: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vabsq_s16(a) }
}
/// Absolute value (wrapping).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vabsq_s32(a: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vabsq_s32(a) }
}

/// Add pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpadd_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vpadd_s16(a, b) }
}
/// Add pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpadd_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vpadd_s32(a, b) }
}
/// Add pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpadd_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vpadd_s8(a, b) }
}
/// Add pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpadd_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vpadd_u16(a, b) }
}
/// Add pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpadd_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vpadd_u32(a, b) }
}
/// Add pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpadd_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vpadd_u8(a, b) }
}

/// Vector add.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vadd_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vadd_s8(a, b) }
}

/// Vector add.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vaddq_s8(a, b) }
}

/// Vector add.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vadd_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vadd_s16(a, b) }
}

/// Vector add.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vaddq_s16(a, b) }
}

/// Vector add.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vadd_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vadd_s32(a, b) }
}

/// Vector add.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vaddq_s32(a, b) }
}

/// Vector add.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vaddq_s64(a, b) }
}

/// Vector add.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vadd_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vadd_u8(a, b) }
}

/// Vector add.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vaddq_u8(a, b) }
}

/// Vector add.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vadd_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vadd_u16(a, b) }
}

/// Vector add.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vaddq_u16(a, b) }
}

/// Vector add.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vadd_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vadd_u32(a, b) }
}

/// Vector add.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vaddq_u32(a, b) }
}

/// Vector add.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vaddq_u64(a, b) }
}

/// Vector add.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vadd_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vadd_f32(a, b) }
}

/// Vector add.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vaddq_f32(a, b) }
}

/// Signed Add Long (vector).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddl_s8(a: int8x8_t, b: int8x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vaddl_s8(a, b) }
}

/// Signed Add Long (vector).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddl_s16(a: int16x4_t, b: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vaddl_s16(a, b) }
}

/// Signed Add Long (vector).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddl_s32(a: int32x2_t, b: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vaddl_s32(a, b) }
}

/// Unsigned Add Long (vector).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddl_u8(a: uint8x8_t, b: uint8x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vaddl_u8(a, b) }
}

/// Unsigned Add Long (vector).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddl_u16(a: uint16x4_t, b: uint16x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vaddl_u16(a, b) }
}

/// Unsigned Add Long (vector).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddl_u32(a: uint32x2_t, b: uint32x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vaddl_u32(a, b) }
}

/// Signed Add Long (vector, high half).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddl_high_s8(a: int8x16_t, b: int8x16_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vaddl_high_s8(a, b) }
}

/// Signed Add Long (vector, high half).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddl_high_s16(a: int16x8_t, b: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vaddl_high_s16(a, b) }
}

/// Signed Add Long (vector, high half).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddl_high_s32(a: int32x4_t, b: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vaddl_high_s32(a, b) }
}

/// Unsigned Add Long (vector, high half).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddl_high_u8(a: uint8x16_t, b: uint8x16_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vaddl_high_u8(a, b) }
}

/// Unsigned Add Long (vector, high half).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddl_high_u16(a: uint16x8_t, b: uint16x8_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vaddl_high_u16(a, b) }
}

/// Unsigned Add Long (vector, high half).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddl_high_u32(a: uint32x4_t, b: uint32x4_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vaddl_high_u32(a, b) }
}

/// Signed Add Wide.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddw_s8(a: int16x8_t, b: int8x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vaddw_s8(a, b) }
}

/// Signed Add Wide.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddw_s16(a: int32x4_t, b: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vaddw_s16(a, b) }
}

/// Signed Add Wide.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddw_s32(a: int64x2_t, b: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vaddw_s32(a, b) }
}

/// Unsigned Add Wide.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddw_u8(a: uint16x8_t, b: uint8x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vaddw_u8(a, b) }
}

/// Unsigned Add Wide.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddw_u16(a: uint32x4_t, b: uint16x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vaddw_u16(a, b) }
}

/// Unsigned Add Wide.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddw_u32(a: uint64x2_t, b: uint32x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vaddw_u32(a, b) }
}

/// Signed Add Wide (high half).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddw_high_s8(a: int16x8_t, b: int8x16_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vaddw_high_s8(a, b) }
}

/// Signed Add Wide (high half).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddw_high_s16(a: int32x4_t, b: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vaddw_high_s16(a, b) }
}

/// Signed Add Wide (high half).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddw_high_s32(a: int64x2_t, b: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vaddw_high_s32(a, b) }
}

/// Unsigned Add Wide (high half).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddw_high_u8(a: uint16x8_t, b: uint8x16_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vaddw_high_u8(a, b) }
}

/// Unsigned Add Wide (high half).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddw_high_u16(a: uint32x4_t, b: uint16x8_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vaddw_high_u16(a, b) }
}

/// Unsigned Add Wide (high half).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddw_high_u32(a: uint64x2_t, b: uint32x4_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vaddw_high_u32(a, b) }
}

/// Add returning High Narrow.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddhn_s16(a: int16x8_t, b: int16x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vaddhn_s16(a, b) }
}

/// Add returning High Narrow.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddhn_s32(a: int32x4_t, b: int32x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vaddhn_s32(a, b) }
}

/// Add returning High Narrow.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddhn_s64(a: int64x2_t, b: int64x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vaddhn_s64(a, b) }
}

/// Add returning High Narrow.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddhn_u16(a: uint16x8_t, b: uint16x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vaddhn_u16(a, b) }
}

/// Add returning High Narrow.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddhn_u32(a: uint32x4_t, b: uint32x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vaddhn_u32(a, b) }
}

/// Add returning High Narrow.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddhn_u64(a: uint64x2_t, b: uint64x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vaddhn_u64(a, b) }
}

/// Add returning High Narrow (high half).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddhn_high_s16(r: int8x8_t, a: int16x8_t, b: int16x8_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vaddhn_high_s16(r, a, b) }
}

/// Add returning High Narrow (high half).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddhn_high_s32(r: int16x4_t, a: int32x4_t, b: int32x4_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vaddhn_high_s32(r, a, b) }
}

/// Add returning High Narrow (high half).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddhn_high_s64(r: int32x2_t, a: int64x2_t, b: int64x2_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vaddhn_high_s64(r, a, b) }
}

/// Add returning High Narrow (high half).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddhn_high_u16(r: uint8x8_t, a: uint16x8_t, b: uint16x8_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vaddhn_high_u16(r, a, b) }
}

/// Add returning High Narrow (high half).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddhn_high_u32(r: uint16x4_t, a: uint32x4_t, b: uint32x4_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vaddhn_high_u32(r, a, b) }
}

/// Add returning High Narrow (high half).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vaddhn_high_u64(r: uint32x2_t, a: uint64x2_t, b: uint64x2_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vaddhn_high_u64(r, a, b) }
}

/// Rounding Add returning High Narrow.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vraddhn_s16(a: int16x8_t, b: int16x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vraddhn_s16(a, b) }
}

/// Rounding Add returning High Narrow.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vraddhn_s32(a: int32x4_t, b: int32x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vraddhn_s32(a, b) }
}

/// Rounding Add returning High Narrow.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vraddhn_s64(a: int64x2_t, b: int64x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vraddhn_s64(a, b) }
}

/// Rounding Add returning High Narrow.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vraddhn_u16(a: uint16x8_t, b: uint16x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vraddhn_u16(a, b) }
}

/// Rounding Add returning High Narrow.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vraddhn_u32(a: uint32x4_t, b: uint32x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vraddhn_u32(a, b) }
}

/// Rounding Add returning High Narrow.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vraddhn_u64(a: uint64x2_t, b: uint64x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vraddhn_u64(a, b) }
}

/// Rounding Add returning High Narrow (high half).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vraddhn_high_s16(r: int8x8_t, a: int16x8_t, b: int16x8_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vraddhn_high_s16(r, a, b) }
}

/// Rounding Add returning High Narrow (high half).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vraddhn_high_s32(r: int16x4_t, a: int32x4_t, b: int32x4_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vraddhn_high_s32(r, a, b) }
}

/// Rounding Add returning High Narrow (high half).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vraddhn_high_s64(r: int32x2_t, a: int64x2_t, b: int64x2_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vraddhn_high_s64(r, a, b) }
}

/// Rounding Add returning High Narrow (high half).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vraddhn_high_u16(r: uint8x8_t, a: uint16x8_t, b: uint16x8_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vraddhn_high_u16(r, a, b) }
}

/// Rounding Add returning High Narrow (high half).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vraddhn_high_u32(r: uint16x4_t, a: uint32x4_t, b: uint32x4_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vraddhn_high_u32(r, a, b) }
}

/// Rounding Add returning High Narrow (high half).
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vraddhn_high_u64(r: uint32x2_t, a: uint64x2_t, b: uint64x2_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vraddhn_high_u64(r, a, b) }
}

/// Signed Add Long Pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpaddl_s8(a: int8x8_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vpaddl_s8(a) }
}

/// Signed Add Long Pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpaddl_s16(a: int16x4_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vpaddl_s16(a) }
}

/// Signed Add Long Pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpaddl_s32(a: int32x2_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vpaddl_s32(a) }
}

/// Signed Add Long Pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpaddlq_s8(a: int8x16_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vpaddlq_s8(a) }
}

/// Signed Add Long Pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpaddlq_s16(a: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vpaddlq_s16(a) }
}

/// Signed Add Long Pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpaddlq_s32(a: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vpaddlq_s32(a) }
}

/// Unsigned Add Long Pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpaddl_u8(a: uint8x8_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vpaddl_u8(a) }
}

/// Unsigned Add Long Pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpaddl_u16(a: uint16x4_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vpaddl_u16(a) }
}

/// Unsigned Add Long Pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpaddl_u32(a: uint32x2_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vpaddl_u32(a) }
}

/// Unsigned Add Long Pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpaddlq_u8(a: uint8x16_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vpaddlq_u8(a) }
}

/// Unsigned Add Long Pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpaddlq_u16(a: uint16x8_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vpaddlq_u16(a) }
}

/// Unsigned Add Long Pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpaddlq_u32(a: uint32x4_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vpaddlq_u32(a) }
}

/// Vector narrow integer.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmovn_s16(a: int16x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vmovn_s16(a) }
}

/// Vector narrow integer.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmovn_s32(a: int32x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vmovn_s32(a) }
}

/// Vector narrow integer.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmovn_s64(a: int64x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vmovn_s64(a) }
}

/// Vector narrow integer.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmovn_u16(a: uint16x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vmovn_u16(a) }
}

/// Vector narrow integer.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmovn_u32(a: uint32x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vmovn_u32(a) }
}

/// Vector narrow integer.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmovn_u64(a: uint64x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vmovn_u64(a) }
}

/// Vector long move.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmovl_s8(a: int8x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vmovl_s8(a) }
}

/// Vector long move.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmovl_s16(a: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmovl_s16(a) }
}

/// Vector long move.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmovl_s32(a: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vmovl_s32(a) }
}

/// Vector long move.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmovl_u8(a: uint8x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vmovl_u8(a) }
}

/// Vector long move.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmovl_u16(a: uint16x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmovl_u16(a) }
}

/// Vector long move.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmovl_u32(a: uint32x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmovl_u32(a) }
}

/// Vector bitwise not.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmvn_s8(a: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vmvn_s8(a) }
}

/// Vector bitwise not.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmvnq_s8(a: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vmvnq_s8(a) }
}

/// Vector bitwise not.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmvn_s16(a: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vmvn_s16(a) }
}

/// Vector bitwise not.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmvnq_s16(a: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vmvnq_s16(a) }
}

/// Vector bitwise not.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmvn_s32(a: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vmvn_s32(a) }
}

/// Vector bitwise not.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmvnq_s32(a: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmvnq_s32(a) }
}

/// Vector bitwise not.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmvn_u8(a: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vmvn_u8(a) }
}

/// Vector bitwise not.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmvnq_u8(a: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vmvnq_u8(a) }
}

/// Vector bitwise not.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmvn_u16(a: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vmvn_u16(a) }
}

/// Vector bitwise not.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmvnq_u16(a: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vmvnq_u16(a) }
}

/// Vector bitwise not.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmvn_u32(a: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vmvn_u32(a) }
}

/// Vector bitwise not.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmvnq_u32(a: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmvnq_u32(a) }
}

/// Vector bitwise not.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmvn_p8(a: poly8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vmvn_p8(a) }
}

/// Vector bitwise not.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmvnq_p8(a: poly8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vmvnq_p8(a) }
}

/// Vector bitwise bit clear
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbic_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vbic_s8(a, b) }
}

/// Vector bitwise bit clear
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbicq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vbicq_s8(a, b) }
}

/// Vector bitwise bit clear
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbic_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vbic_s16(a, b) }
}

/// Vector bitwise bit clear
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbicq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vbicq_s16(a, b) }
}

/// Vector bitwise bit clear
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbic_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vbic_s32(a, b) }
}

/// Vector bitwise bit clear
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbicq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vbicq_s32(a, b) }
}

/// Vector bitwise bit clear
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbic_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vbic_s64(a, b) }
}

/// Vector bitwise bit clear
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbicq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vbicq_s64(a, b) }
}

/// Vector bitwise bit clear
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbic_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vbic_u8(a, b) }
}

/// Vector bitwise bit clear
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbicq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vbicq_u8(a, b) }
}

/// Vector bitwise bit clear
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbic_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vbic_u16(a, b) }
}

/// Vector bitwise bit clear
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbicq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vbicq_u16(a, b) }
}

/// Vector bitwise bit clear
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbic_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vbic_u32(a, b) }
}

/// Vector bitwise bit clear
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbicq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vbicq_u32(a, b) }
}

/// Vector bitwise bit clear
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbic_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vbic_u64(a, b) }
}

/// Vector bitwise bit clear
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbicq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vbicq_u64(a, b) }
}

/// Bitwise Select instructions. This instruction sets each bit in the
/// destination SIMD&FP register to the corresponding bit from the first source
/// SIMD&FP register when the original destination bit was 1, otherwise from the
/// second source SIMD&FP register.

/// Bitwise Select.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbsl_s8(a: uint8x8_t, b: int8x8_t, c: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vbsl_s8(a, b, c) }
}

/// Bitwise Select.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbsl_s16(a: uint16x4_t, b: int16x4_t, c: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vbsl_s16(a, b, c) }
}

/// Bitwise Select.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbsl_s32(a: uint32x2_t, b: int32x2_t, c: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vbsl_s32(a, b, c) }
}

/// Bitwise Select.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbsl_s64(a: uint64x1_t, b: int64x1_t, c: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vbsl_s64(a, b, c) }
}

/// Bitwise Select.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbsl_u8(a: uint8x8_t, b: uint8x8_t, c: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vbsl_u8(a, b, c) }
}

/// Bitwise Select.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbsl_u16(a: uint16x4_t, b: uint16x4_t, c: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vbsl_u16(a, b, c) }
}

/// Bitwise Select.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbsl_u32(a: uint32x2_t, b: uint32x2_t, c: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vbsl_u32(a, b, c) }
}

/// Bitwise Select.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbsl_u64(a: uint64x1_t, b: uint64x1_t, c: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vbsl_u64(a, b, c) }
}

/// Bitwise Select.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbsl_f32(a: uint32x2_t, b: float32x2_t, c: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vbsl_f32(a, b, c) }
}

/// Bitwise Select.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbsl_p8(a: uint8x8_t, b: poly8x8_t, c: poly8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vbsl_p8(a, b, c) }
}

/// Bitwise Select.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbsl_p16(a: uint16x4_t, b: poly16x4_t, c: poly16x4_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vbsl_p16(a, b, c) }
}

/// Bitwise Select. (128-bit)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbslq_s8(a: uint8x16_t, b: int8x16_t, c: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vbslq_s8(a, b, c) }
}

/// Bitwise Select. (128-bit)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbslq_s16(a: uint16x8_t, b: int16x8_t, c: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vbslq_s16(a, b, c) }
}

/// Bitwise Select. (128-bit)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbslq_s32(a: uint32x4_t, b: int32x4_t, c: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vbslq_s32(a, b, c) }
}

/// Bitwise Select. (128-bit)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbslq_s64(a: uint64x2_t, b: int64x2_t, c: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vbslq_s64(a, b, c) }
}

/// Bitwise Select. (128-bit)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbslq_u8(a: uint8x16_t, b: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vbslq_u8(a, b, c) }
}

/// Bitwise Select. (128-bit)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbslq_u16(a: uint16x8_t, b: uint16x8_t, c: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vbslq_u16(a, b, c) }
}

/// Bitwise Select. (128-bit)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbslq_u32(a: uint32x4_t, b: uint32x4_t, c: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vbslq_u32(a, b, c) }
}

/// Bitwise Select. (128-bit)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbslq_u64(a: uint64x2_t, b: uint64x2_t, c: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vbslq_u64(a, b, c) }
}

/// Bitwise Select. (128-bit)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbslq_p8(a: uint8x16_t, b: poly8x16_t, c: poly8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vbslq_p8(a, b, c) }
}

/// Bitwise Select. (128-bit)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbslq_p16(a: uint16x8_t, b: poly16x8_t, c: poly16x8_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vbslq_p16(a, b, c) }
}

/// Bitwise Select. (128-bit)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vbslq_f32(a: uint32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vbslq_f32(a, b, c) }
}

/// Vector bitwise inclusive OR NOT
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vorn_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vorn_s8(a, b) }
}

/// Vector bitwise inclusive OR NOT
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vornq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vornq_s8(a, b) }
}

/// Vector bitwise inclusive OR NOT
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vorn_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vorn_s16(a, b) }
}

/// Vector bitwise inclusive OR NOT
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vornq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vornq_s16(a, b) }
}

/// Vector bitwise inclusive OR NOT
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vorn_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vorn_s32(a, b) }
}

/// Vector bitwise inclusive OR NOT
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vornq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vornq_s32(a, b) }
}

/// Vector bitwise inclusive OR NOT
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vorn_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vorn_s64(a, b) }
}

/// Vector bitwise inclusive OR NOT
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vornq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vornq_s64(a, b) }
}

/// Vector bitwise inclusive OR NOT
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vorn_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vorn_u8(a, b) }
}

/// Vector bitwise inclusive OR NOT
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vornq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vornq_u8(a, b) }
}

/// Vector bitwise inclusive OR NOT
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vorn_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vorn_u16(a, b) }
}

/// Vector bitwise inclusive OR NOT
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vornq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vornq_u16(a, b) }
}

/// Vector bitwise inclusive OR NOT
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vorn_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vorn_u32(a, b) }
}

/// Vector bitwise inclusive OR NOT
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vornq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vornq_u32(a, b) }
}

/// Vector bitwise inclusive OR NOT
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vorn_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vorn_u64(a, b) }
}

/// Vector bitwise inclusive OR NOT
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vornq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vornq_u64(a, b) }
}

/// Folding minimum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpmin_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vpmin_s8(a, b) }
}

/// Folding minimum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpmin_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vpmin_s16(a, b) }
}

/// Folding minimum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpmin_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vpmin_s32(a, b) }
}

/// Folding minimum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpmin_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vpmin_u8(a, b) }
}

/// Folding minimum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpmin_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vpmin_u16(a, b) }
}

/// Folding minimum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpmin_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vpmin_u32(a, b) }
}

/// Folding minimum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpmin_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vpmin_f32(a, b) }
}

/// Folding maximum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpmax_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vpmax_s8(a, b) }
}

/// Folding maximum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpmax_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vpmax_s16(a, b) }
}

/// Folding maximum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpmax_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vpmax_s32(a, b) }
}

/// Folding maximum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpmax_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vpmax_u8(a, b) }
}

/// Folding maximum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpmax_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vpmax_u16(a, b) }
}

/// Folding maximum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpmax_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vpmax_u32(a, b) }
}

/// Folding maximum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpmax_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vpmax_f32(a, b) }
}

/// Move vector element to general-purpose register
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vgetq_lane_u64<const IMM5: i32>(v: uint64x2_t) -> u64 {
  unsafe { core::arch::aarch64::vgetq_lane_u64::<IMM5>(v) }
}

/// Move vector element to general-purpose register
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_lane_u64<const IMM5: i32>(v: uint64x1_t) -> u64 {
  unsafe { core::arch::aarch64::vget_lane_u64::<IMM5>(v) }
}

/// Move vector element to general-purpose register
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_lane_u16<const IMM5: i32>(v: uint16x4_t) -> u16 {
  unsafe { core::arch::aarch64::vget_lane_u16::<IMM5>(v) }
}

/// Move vector element to general-purpose register
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_lane_s16<const IMM5: i32>(v: int16x4_t) -> i16 {
  unsafe { core::arch::aarch64::vget_lane_s16::<IMM5>(v) }
}

/// Move vector element to general-purpose register
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_lane_p16<const IMM5: i32>(v: poly16x4_t) -> p16 {
  unsafe { core::arch::aarch64::vget_lane_p16::<IMM5>(v) }
}

/// Move vector element to general-purpose register
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_lane_u32<const IMM5: i32>(v: uint32x2_t) -> u32 {
  unsafe { core::arch::aarch64::vget_lane_u32::<IMM5>(v) }
}

/// Move vector element to general-purpose register
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_lane_s32<const IMM5: i32>(v: int32x2_t) -> i32 {
  unsafe { core::arch::aarch64::vget_lane_s32::<IMM5>(v) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_lane_f32<const IMM5: i32>(v: float32x2_t) -> f32 {
  unsafe { core::arch::aarch64::vget_lane_f32::<IMM5>(v) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vgetq_lane_f32<const IMM5: i32>(v: float32x4_t) -> f32 {
  unsafe { core::arch::aarch64::vgetq_lane_f32::<IMM5>(v) }
}

/// Move vector element to general-purpose register
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_lane_p64<const IMM5: i32>(v: poly64x1_t) -> p64 {
  unsafe { core::arch::aarch64::vget_lane_p64::<IMM5>(v) }
}

/// Move vector element to general-purpose register
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vgetq_lane_p64<const IMM5: i32>(v: poly64x2_t) -> p64 {
  unsafe { core::arch::aarch64::vgetq_lane_p64::<IMM5>(v) }
}

/// Move vector element to general-purpose register
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_lane_s64<const IMM5: i32>(v: int64x1_t) -> i64 {
  unsafe { core::arch::aarch64::vget_lane_s64::<IMM5>(v) }
}

/// Move vector element to general-purpose register
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vgetq_lane_s64<const IMM5: i32>(v: int64x2_t) -> i64 {
  unsafe { core::arch::aarch64::vgetq_lane_s64::<IMM5>(v) }
}

/// Move vector element to general-purpose register
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vgetq_lane_u16<const IMM5: i32>(v: uint16x8_t) -> u16 {
  unsafe { core::arch::aarch64::vgetq_lane_u16::<IMM5>(v) }
}

/// Move vector element to general-purpose register
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vgetq_lane_u32<const IMM5: i32>(v: uint32x4_t) -> u32 {
  unsafe { core::arch::aarch64::vgetq_lane_u32::<IMM5>(v) }
}

/// Move vector element to general-purpose register
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vgetq_lane_s16<const IMM5: i32>(v: int16x8_t) -> i16 {
  unsafe { core::arch::aarch64::vgetq_lane_s16::<IMM5>(v) }
}

/// Move vector element to general-purpose register
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vgetq_lane_p16<const IMM5: i32>(v: poly16x8_t) -> p16 {
  unsafe { core::arch::aarch64::vgetq_lane_p16::<IMM5>(v) }
}

/// Move vector element to general-purpose register
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vgetq_lane_s32<const IMM5: i32>(v: int32x4_t) -> i32 {
  unsafe { core::arch::aarch64::vgetq_lane_s32::<IMM5>(v) }
}

/// Move vector element to general-purpose register
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_lane_u8<const IMM5: i32>(v: uint8x8_t) -> u8 {
  unsafe { core::arch::aarch64::vget_lane_u8::<IMM5>(v) }
}

/// Move vector element to general-purpose register
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_lane_s8<const IMM5: i32>(v: int8x8_t) -> i8 {
  unsafe { core::arch::aarch64::vget_lane_s8::<IMM5>(v) }
}

/// Move vector element to general-purpose register
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_lane_p8<const IMM5: i32>(v: poly8x8_t) -> p8 {
  unsafe { core::arch::aarch64::vget_lane_p8::<IMM5>(v) }
}

/// Move vector element to general-purpose register
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vgetq_lane_u8<const IMM5: i32>(v: uint8x16_t) -> u8 {
  unsafe { core::arch::aarch64::vgetq_lane_u8::<IMM5>(v) }
}

/// Move vector element to general-purpose register
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vgetq_lane_s8<const IMM5: i32>(v: int8x16_t) -> i8 {
  unsafe { core::arch::aarch64::vgetq_lane_s8::<IMM5>(v) }
}

/// Move vector element to general-purpose register
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vgetq_lane_p8<const IMM5: i32>(v: poly8x16_t) -> p8 {
  unsafe { core::arch::aarch64::vgetq_lane_p8::<IMM5>(v) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_high_s8(a: int8x16_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vget_high_s8(a) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_high_s16(a: int16x8_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vget_high_s16(a) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_high_s32(a: int32x4_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vget_high_s32(a) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_high_s64(a: int64x2_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vget_high_s64(a) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_high_u8(a: uint8x16_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vget_high_u8(a) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_high_u16(a: uint16x8_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vget_high_u16(a) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_high_u32(a: uint32x4_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vget_high_u32(a) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_high_u64(a: uint64x2_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vget_high_u64(a) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_high_p8(a: poly8x16_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vget_high_p8(a) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_high_p16(a: poly16x8_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vget_high_p16(a) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_high_f32(a: float32x4_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vget_high_f32(a) }
}

// /// Duplicate vector element to vector or scalar
// #[inline]
// #[cfg(target_feature = "neon")]
// #[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
// pub fn vget_low_s8(a: int8x16_t) -> int8x8_t {
//   unsafe { core::arch::aarch64::vget_low_s8(a) }
// }

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_low_s16(a: int16x8_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vget_low_s16(a) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_low_s32(a: int32x4_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vget_low_s32(a) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_low_s64(a: int64x2_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vget_low_s64(a) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_low_u8(a: uint8x16_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vget_low_u8(a) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_low_u16(a: uint16x8_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vget_low_u16(a) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_low_u32(a: uint32x4_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vget_low_u32(a) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_low_u64(a: uint64x2_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vget_low_u64(a) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_low_p8(a: poly8x16_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vget_low_p8(a) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_low_p16(a: poly16x8_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vget_low_p16(a) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vget_low_f32(a: float32x4_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vget_low_f32(a) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vdupq_n_s8(value: i8) -> int8x16_t {
  unsafe { core::arch::aarch64::vdupq_n_s8(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vdupq_n_s16(value: i16) -> int16x8_t {
  unsafe { core::arch::aarch64::vdupq_n_s16(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vdupq_n_s32(value: i32) -> int32x4_t {
  unsafe { core::arch::aarch64::vdupq_n_s32(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vdupq_n_s64(value: i64) -> int64x2_t {
  unsafe { core::arch::aarch64::vdupq_n_s64(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vdupq_n_u8(value: u8) -> uint8x16_t {
  unsafe { core::arch::aarch64::vdupq_n_u8(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vdupq_n_u16(value: u16) -> uint16x8_t {
  unsafe { core::arch::aarch64::vdupq_n_u16(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vdupq_n_u32(value: u32) -> uint32x4_t {
  unsafe { core::arch::aarch64::vdupq_n_u32(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vdupq_n_u64(value: u64) -> uint64x2_t {
  unsafe { core::arch::aarch64::vdupq_n_u64(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vdupq_n_p8(value: p8) -> poly8x16_t {
  unsafe { core::arch::aarch64::vdupq_n_p8(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vdupq_n_p16(value: p16) -> poly16x8_t {
  unsafe { core::arch::aarch64::vdupq_n_p16(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vdupq_n_f32(value: f32) -> float32x4_t {
  unsafe { core::arch::aarch64::vdupq_n_f32(value) }
}

// /// Duplicate vector element to vector or scalar
// ///
// /// Private vfp4 version used by FMA intriniscs because LLVM does
// /// not inline the non-vfp4 version in vfp4 functions.
// #[inline]
// #[cfg(target_feature = "neon")]
// #[cfg_attr(target_arch = "arm", cfg(target_feature = "vfp4"))]
// unsafe fn vdupq_n_f32_vfp4(value: f32) -> float32x4_t {
//   unsafe { core::arch::aarch64::vdupq_n_f32_vfp4(value) }
// }

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vdup_n_s8(value: i8) -> int8x8_t {
  unsafe { core::arch::aarch64::vdup_n_s8(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vdup_n_s16(value: i16) -> int16x4_t {
  unsafe { core::arch::aarch64::vdup_n_s16(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vdup_n_s32(value: i32) -> int32x2_t {
  unsafe { core::arch::aarch64::vdup_n_s32(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vdup_n_s64(value: i64) -> int64x1_t {
  unsafe { core::arch::aarch64::vdup_n_s64(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vdup_n_u8(value: u8) -> uint8x8_t {
  unsafe { core::arch::aarch64::vdup_n_u8(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vdup_n_u16(value: u16) -> uint16x4_t {
  unsafe { core::arch::aarch64::vdup_n_u16(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vdup_n_u32(value: u32) -> uint32x2_t {
  unsafe { core::arch::aarch64::vdup_n_u32(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vdup_n_u64(value: u64) -> uint64x1_t {
  unsafe { core::arch::aarch64::vdup_n_u64(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vdup_n_p8(value: p8) -> poly8x8_t {
  unsafe { core::arch::aarch64::vdup_n_p8(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vdup_n_p16(value: p16) -> poly16x4_t {
  unsafe { core::arch::aarch64::vdup_n_p16(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vdup_n_f32(value: f32) -> float32x2_t {
  unsafe { core::arch::aarch64::vdup_n_f32(value) }
}

// /// Duplicate vector element to vector or scalar
// ///
// /// Private vfp4 version used by FMA intriniscs because LLVM does
// /// not inline the non-vfp4 version in vfp4 functions.
// #[inline]
// #[cfg(target_feature = "neon")]
// #[cfg_attr(target_arch = "arm", cfg(target_feature = "vfp4"))]
// unsafe fn vdup_n_f32_vfp4(value: f32) -> float32x2_t {
//   unsafe { core::arch::aarch64::vdup_n_f32_vfp4(value) }
// }

/// Load SIMD&FP register (immediate offset)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vldrq_p128(a: &p128) -> p128 {
  unsafe { core::arch::aarch64::vldrq_p128(a) }
}

/// Store SIMD&FP register (immediate offset)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vstrq_p128(a: &mut p128, b: p128) {
  unsafe { core::arch::aarch64::vstrq_p128(a, b) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmov_n_s8(value: i8) -> int8x8_t {
  unsafe { core::arch::aarch64::vmov_n_s8(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmov_n_s16(value: i16) -> int16x4_t {
  unsafe { core::arch::aarch64::vmov_n_s16(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmov_n_s32(value: i32) -> int32x2_t {
  unsafe { core::arch::aarch64::vmov_n_s32(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmov_n_s64(value: i64) -> int64x1_t {
  unsafe { core::arch::aarch64::vmov_n_s64(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmov_n_u8(value: u8) -> uint8x8_t {
  unsafe { core::arch::aarch64::vmov_n_u8(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmov_n_u16(value: u16) -> uint16x4_t {
  unsafe { core::arch::aarch64::vmov_n_u16(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmov_n_u32(value: u32) -> uint32x2_t {
  unsafe { core::arch::aarch64::vmov_n_u32(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmov_n_u64(value: u64) -> uint64x1_t {
  unsafe { core::arch::aarch64::vmov_n_u64(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmov_n_p8(value: p8) -> poly8x8_t {
  unsafe { core::arch::aarch64::vmov_n_p8(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmov_n_p16(value: p16) -> poly16x4_t {
  unsafe { core::arch::aarch64::vmov_n_p16(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmov_n_f32(value: f32) -> float32x2_t {
  unsafe { core::arch::aarch64::vmov_n_f32(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmovq_n_s8(value: i8) -> int8x16_t {
  unsafe { core::arch::aarch64::vmovq_n_s8(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmovq_n_s16(value: i16) -> int16x8_t {
  unsafe { core::arch::aarch64::vmovq_n_s16(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmovq_n_s32(value: i32) -> int32x4_t {
  unsafe { core::arch::aarch64::vmovq_n_s32(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmovq_n_s64(value: i64) -> int64x2_t {
  unsafe { core::arch::aarch64::vmovq_n_s64(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmovq_n_u8(value: u8) -> uint8x16_t {
  unsafe { core::arch::aarch64::vmovq_n_u8(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmovq_n_u16(value: u16) -> uint16x8_t {
  unsafe { core::arch::aarch64::vmovq_n_u16(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmovq_n_u32(value: u32) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmovq_n_u32(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmovq_n_u64(value: u64) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmovq_n_u64(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmovq_n_p8(value: p8) -> poly8x16_t {
  unsafe { core::arch::aarch64::vmovq_n_p8(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmovq_n_p16(value: p16) -> poly16x8_t {
  unsafe { core::arch::aarch64::vmovq_n_p16(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vmovq_n_f32(value: f32) -> float32x4_t {
  unsafe { core::arch::aarch64::vmovq_n_f32(value) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vext_s64<const N: i32>(a: int64x1_t, _b: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vext_s64::<N>(a, _b) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vext_u64<const N: i32>(a: uint64x1_t, _b: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vext_u64::<N>(a, _b) }
}

/// Population count per byte.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vcnt_s8(a: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vcnt_s8(a) }
}
/// Population count per byte.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vcntq_s8(a: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vcntq_s8(a) }
}
/// Population count per byte.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vcnt_u8(a: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vcnt_u8(a) }
}
/// Population count per byte.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vcntq_u8(a: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vcntq_u8(a) }
}
/// Population count per byte.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vcnt_p8(a: poly8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vcnt_p8(a) }
}
/// Population count per byte.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vcntq_p8(a: poly8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vcntq_p8(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev16_s8(a: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vrev16_s8(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev16q_s8(a: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vrev16q_s8(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev16_u8(a: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vrev16_u8(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev16q_u8(a: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vrev16q_u8(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev16_p8(a: poly8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vrev16_p8(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev16q_p8(a: poly8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vrev16q_p8(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev32_s8(a: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vrev32_s8(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev32q_s8(a: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vrev32q_s8(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev32_u8(a: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vrev32_u8(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev32q_u8(a: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vrev32q_u8(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev32_s16(a: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vrev32_s16(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev32q_s16(a: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vrev32q_s16(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev32_p16(a: poly16x4_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vrev32_p16(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev32q_p16(a: poly16x8_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vrev32q_p16(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev32_u16(a: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vrev32_u16(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev32q_u16(a: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vrev32q_u16(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev32_p8(a: poly8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vrev32_p8(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev32q_p8(a: poly8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vrev32q_p8(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev64_s8(a: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vrev64_s8(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev64q_s8(a: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vrev64q_s8(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev64_s16(a: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vrev64_s16(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev64q_s16(a: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vrev64q_s16(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev64_s32(a: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vrev64_s32(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev64q_s32(a: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vrev64q_s32(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev64_u8(a: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vrev64_u8(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev64q_u8(a: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vrev64q_u8(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev64_u16(a: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vrev64_u16(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev64q_u16(a: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vrev64q_u16(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev64_u32(a: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vrev64_u32(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev64q_u32(a: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vrev64q_u32(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev64_f32(a: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vrev64_f32(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev64q_f32(a: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vrev64q_f32(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev64_p8(a: poly8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vrev64_p8(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev64q_p8(a: poly8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vrev64q_p8(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev64_p16(a: poly16x4_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vrev64_p16(a) }
}

/// Reversing vector elements (swap endianness)
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vrev64q_p16(a: poly16x8_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vrev64q_p16(a) }
}

/// Signed Add and Accumulate Long Pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpadal_s8(a: int16x4_t, b: int8x8_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vpadal_s8(a, b) }
}

/// Signed Add and Accumulate Long Pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpadal_s16(a: int32x2_t, b: int16x4_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vpadal_s16(a, b) }
}

/// Signed Add and Accumulate Long Pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpadal_s32(a: int64x1_t, b: int32x2_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vpadal_s32(a, b) }
}

/// Signed Add and Accumulate Long Pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpadalq_s8(a: int16x8_t, b: int8x16_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vpadalq_s8(a, b) }
}

/// Signed Add and Accumulate Long Pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpadalq_s16(a: int32x4_t, b: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vpadalq_s16(a, b) }
}

/// Signed Add and Accumulate Long Pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpadalq_s32(a: int64x2_t, b: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vpadalq_s32(a, b) }
}

/// Unsigned Add and Accumulate Long Pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpadal_u8(a: uint16x4_t, b: uint8x8_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vpadal_u8(a, b) }
}

/// Unsigned Add and Accumulate Long Pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpadal_u16(a: uint32x2_t, b: uint16x4_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vpadal_u16(a, b) }
}

/// Unsigned Add and Accumulate Long Pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpadal_u32(a: uint64x1_t, b: uint32x2_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vpadal_u32(a, b) }
}

/// Unsigned Add and Accumulate Long Pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpadalq_u8(a: uint16x8_t, b: uint8x16_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vpadalq_u8(a, b) }
}

/// Unsigned Add and Accumulate Long Pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpadalq_u16(a: uint32x4_t, b: uint16x8_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vpadalq_u16(a, b) }
}

/// Unsigned Add and Accumulate Long Pairwise.
#[inline]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v7"))]
pub fn vpadalq_u32(a: uint64x2_t, b: uint32x4_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vpadalq_u32(a, b) }
}

/// 8-bit integer matrix multiply-accumulate
#[inline]
#[cfg_attr(not(bootstrap), cfg(target_feature = "i8mm"))]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v8"))]
pub fn vmmlaq_s32(a: int32x4_t, b: int8x16_t, c: int8x16_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmmlaq_s32(a, b, c) }
}

/// 8-bit integer matrix multiply-accumulate
#[inline]
#[cfg_attr(not(bootstrap), cfg(target_feature = "i8mm"))]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v8"))]
pub fn vmmlaq_u32(a: uint32x4_t, b: uint8x16_t, c: uint8x16_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmmlaq_u32(a, b, c) }
}

/// Unsigned and signed 8-bit integer matrix multiply-accumulate
#[inline]
#[cfg_attr(not(bootstrap), cfg(target_feature = "i8mm"))]
#[cfg(target_feature = "neon")]
#[cfg_attr(target_arch = "arm", cfg(target_feature = "v8"))]
pub fn vusmmlaq_s32(a: int32x4_t, b: uint8x16_t, c: int8x16_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vusmmlaq_s32(a, b, c) }
}
