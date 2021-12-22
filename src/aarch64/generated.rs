// This code is automatically generated. DO NOT MODIFY.
//
// Instead, modify `crates/stdarch-gen/neon.spec` and run the following command
// to re-generate this file:
//
// ```
// OUT_DIR=`pwd`/crates/core_arch cargo run -p stdarch-gen -- crates/stdarch-gen/neon.spec
// ```

pub use core::arch::aarch64::{float32x2_t, float32x2x2_t, float32x2x3_t, float32x2x4_t, float32x4_t, float32x4x2_t, float32x4x3_t, float32x4x4_t, float64x1_t, float64x1x2_t, float64x1x3_t, float64x1x4_t, float64x2_t, float64x2x2_t, float64x2x3_t, float64x2x4_t, int16x4_t, int16x4x2_t, int16x4x3_t, int16x4x4_t, int16x8_t, int16x8x2_t, int16x8x3_t, int16x8x4_t, int32x2_t, int32x2x2_t, int32x2x3_t, int32x2x4_t, int32x4_t, int32x4x2_t, int32x4x3_t, int32x4x4_t, int64x1_t, int64x1x2_t, int64x1x3_t, int64x1x4_t, int64x2_t, int64x2x2_t, int64x2x3_t, int64x2x4_t, int8x16_t, int8x16x2_t, int8x16x3_t, int8x16x4_t, int8x8_t, int8x8x2_t, int8x8x3_t, int8x8x4_t, poly16x4_t, poly16x4x2_t, poly16x4x3_t, poly16x4x4_t, poly16x8_t, poly16x8x2_t, poly16x8x3_t, poly16x8x4_t, poly64x1_t, poly64x1x2_t, poly64x1x3_t, poly64x1x4_t, poly64x2_t, poly64x2x2_t, poly64x2x3_t, poly64x2x4_t, poly8x16_t, poly8x16x2_t, poly8x16x3_t, poly8x16x4_t, poly8x8_t, poly8x8x2_t, poly8x8x3_t, poly8x8x4_t, uint16x4_t, uint16x4x2_t, uint16x4x3_t, uint16x4x4_t, uint16x8_t, uint16x8x2_t, uint16x8x3_t, uint16x8x4_t, uint32x2_t, uint32x2x2_t, uint32x2x3_t, uint32x2x4_t, uint32x4_t, uint32x4x2_t, uint32x4x3_t, uint32x4x4_t, uint64x1_t, uint64x1x2_t, uint64x1x3_t, uint64x1x4_t, uint64x2_t, uint64x2x2_t, uint64x2x3_t, uint64x2x4_t, uint8x16_t, uint8x16x2_t, uint8x16x3_t, uint8x16x4_t, uint8x8_t, uint8x8x2_t, uint8x8x3_t, uint8x8x4_t};

type p8 = u8;
type p16 = u16;
type p128 = u128;

/// Vector bitwise and
#[inline]
#[cfg(target_feature = "neon")]
pub fn vand_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vand_s8(a, b) }
}

/// Vector bitwise and
#[inline]
#[cfg(target_feature = "neon")]
pub fn vandq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vandq_s8(a, b) }
}

/// Vector bitwise and
#[inline]
#[cfg(target_feature = "neon")]
pub fn vand_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vand_s16(a, b) }
}

/// Vector bitwise and
#[inline]
#[cfg(target_feature = "neon")]
pub fn vandq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vandq_s16(a, b) }
}

/// Vector bitwise and
#[inline]
#[cfg(target_feature = "neon")]
pub fn vand_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vand_s32(a, b) }
}

/// Vector bitwise and
#[inline]
#[cfg(target_feature = "neon")]
pub fn vandq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vandq_s32(a, b) }
}

/// Vector bitwise and
#[inline]
#[cfg(target_feature = "neon")]
pub fn vand_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vand_u8(a, b) }
}

/// Vector bitwise and
#[inline]
#[cfg(target_feature = "neon")]
pub fn vandq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vandq_u8(a, b) }
}

/// Vector bitwise and
#[inline]
#[cfg(target_feature = "neon")]
pub fn vand_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vand_u16(a, b) }
}

/// Vector bitwise and
#[inline]
#[cfg(target_feature = "neon")]
pub fn vandq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vandq_u16(a, b) }
}

/// Vector bitwise and
#[inline]
#[cfg(target_feature = "neon")]
pub fn vand_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vand_u32(a, b) }
}

/// Vector bitwise and
#[inline]
#[cfg(target_feature = "neon")]
pub fn vandq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vandq_u32(a, b) }
}

/// Vector bitwise and
#[inline]
#[cfg(target_feature = "neon")]
pub fn vand_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vand_s64(a, b) }
}

/// Vector bitwise and
#[inline]
#[cfg(target_feature = "neon")]
pub fn vandq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vandq_s64(a, b) }
}

/// Vector bitwise and
#[inline]
#[cfg(target_feature = "neon")]
pub fn vand_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vand_u64(a, b) }
}

/// Vector bitwise and
#[inline]
#[cfg(target_feature = "neon")]
pub fn vandq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vandq_u64(a, b) }
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vorr_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vorr_s8(a, b) }
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vorrq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vorrq_s8(a, b) }
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vorr_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vorr_s16(a, b) }
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vorrq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vorrq_s16(a, b) }
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vorr_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vorr_s32(a, b) }
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vorrq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vorrq_s32(a, b) }
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vorr_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vorr_u8(a, b) }
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vorrq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vorrq_u8(a, b) }
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vorr_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vorr_u16(a, b) }
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vorrq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vorrq_u16(a, b) }
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vorr_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vorr_u32(a, b) }
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vorrq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vorrq_u32(a, b) }
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vorr_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vorr_s64(a, b) }
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vorrq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vorrq_s64(a, b) }
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vorr_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vorr_u64(a, b) }
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vorrq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vorrq_u64(a, b) }
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn veor_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::veor_s8(a, b) }
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn veorq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::veorq_s8(a, b) }
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn veor_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::veor_s16(a, b) }
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn veorq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::veorq_s16(a, b) }
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn veor_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::veor_s32(a, b) }
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn veorq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::veorq_s32(a, b) }
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn veor_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::veor_u8(a, b) }
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn veorq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::veorq_u8(a, b) }
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn veor_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::veor_u16(a, b) }
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn veorq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::veorq_u16(a, b) }
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn veor_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::veor_u32(a, b) }
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn veorq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::veorq_u32(a, b) }
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn veor_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::veor_s64(a, b) }
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn veorq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::veorq_s64(a, b) }
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn veor_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::veor_u64(a, b) }
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn veorq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::veorq_u64(a, b) }
}

/// Absolute difference between the arguments
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabd_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vabd_s8(a, b) }
}

/// Absolute difference between the arguments
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabdq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vabdq_s8(a, b) }
}

/// Absolute difference between the arguments
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabd_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vabd_s16(a, b) }
}

/// Absolute difference between the arguments
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabdq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vabdq_s16(a, b) }
}

/// Absolute difference between the arguments
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabd_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vabd_s32(a, b) }
}

/// Absolute difference between the arguments
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabdq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vabdq_s32(a, b) }
}

/// Absolute difference between the arguments
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabd_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vabd_u8(a, b) }
}

/// Absolute difference between the arguments
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabdq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vabdq_u8(a, b) }
}

/// Absolute difference between the arguments
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabd_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vabd_u16(a, b) }
}

/// Absolute difference between the arguments
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabdq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vabdq_u16(a, b) }
}

/// Absolute difference between the arguments
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabd_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vabd_u32(a, b) }
}

/// Absolute difference between the arguments
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabdq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vabdq_u32(a, b) }
}

/// Absolute difference between the arguments of Floating
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabd_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vabd_f32(a, b) }
}

/// Absolute difference between the arguments of Floating
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabdq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vabdq_f32(a, b) }
}

/// Unsigned Absolute difference Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabdl_u8(a: uint8x8_t, b: uint8x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vabdl_u8(a, b) }
}

/// Unsigned Absolute difference Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabdl_u16(a: uint16x4_t, b: uint16x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vabdl_u16(a, b) }
}

/// Unsigned Absolute difference Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabdl_u32(a: uint32x2_t, b: uint32x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vabdl_u32(a, b) }
}

/// Signed Absolute difference Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabdl_s8(a: int8x8_t, b: int8x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vabdl_s8(a, b) }
}

/// Signed Absolute difference Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabdl_s16(a: int16x4_t, b: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vabdl_s16(a, b) }
}

/// Signed Absolute difference Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabdl_s32(a: int32x2_t, b: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vabdl_s32(a, b) }
}

/// Compare bitwise Equal (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceq_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vceq_u8(a, b) }
}

/// Compare bitwise Equal (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vceqq_u8(a, b) }
}

/// Compare bitwise Equal (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceq_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vceq_u16(a, b) }
}

/// Compare bitwise Equal (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vceqq_u16(a, b) }
}

/// Compare bitwise Equal (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceq_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vceq_u32(a, b) }
}

/// Compare bitwise Equal (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vceqq_u32(a, b) }
}

/// Compare bitwise Equal (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceq_s8(a: int8x8_t, b: int8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vceq_s8(a, b) }
}

/// Compare bitwise Equal (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqq_s8(a: int8x16_t, b: int8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vceqq_s8(a, b) }
}

/// Compare bitwise Equal (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceq_s16(a: int16x4_t, b: int16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vceq_s16(a, b) }
}

/// Compare bitwise Equal (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqq_s16(a: int16x8_t, b: int16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vceqq_s16(a, b) }
}

/// Compare bitwise Equal (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceq_s32(a: int32x2_t, b: int32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vceq_s32(a, b) }
}

/// Compare bitwise Equal (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqq_s32(a: int32x4_t, b: int32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vceqq_s32(a, b) }
}

/// Compare bitwise Equal (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceq_p8(a: poly8x8_t, b: poly8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vceq_p8(a, b) }
}

/// Compare bitwise Equal (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqq_p8(a: poly8x16_t, b: poly8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vceqq_p8(a, b) }
}

/// Floating-point compare equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceq_f32(a: float32x2_t, b: float32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vceq_f32(a, b) }
}

/// Floating-point compare equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqq_f32(a: float32x4_t, b: float32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vceqq_f32(a, b) }
}

/// Signed compare bitwise Test bits nonzero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtst_s8(a: int8x8_t, b: int8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vtst_s8(a, b) }
}

/// Signed compare bitwise Test bits nonzero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtstq_s8(a: int8x16_t, b: int8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vtstq_s8(a, b) }
}

/// Signed compare bitwise Test bits nonzero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtst_s16(a: int16x4_t, b: int16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vtst_s16(a, b) }
}

/// Signed compare bitwise Test bits nonzero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtstq_s16(a: int16x8_t, b: int16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vtstq_s16(a, b) }
}

/// Signed compare bitwise Test bits nonzero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtst_s32(a: int32x2_t, b: int32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vtst_s32(a, b) }
}

/// Signed compare bitwise Test bits nonzero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtstq_s32(a: int32x4_t, b: int32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vtstq_s32(a, b) }
}

/// Signed compare bitwise Test bits nonzero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtst_p8(a: poly8x8_t, b: poly8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vtst_p8(a, b) }
}

/// Signed compare bitwise Test bits nonzero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtstq_p8(a: poly8x16_t, b: poly8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vtstq_p8(a, b) }
}

/// Signed compare bitwise Test bits nonzero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtst_p16(a: poly16x4_t, b: poly16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vtst_p16(a, b) }
}

/// Signed compare bitwise Test bits nonzero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtstq_p16(a: poly16x8_t, b: poly16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vtstq_p16(a, b) }
}

/// Unsigned compare bitwise Test bits nonzero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtst_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vtst_u8(a, b) }
}

/// Unsigned compare bitwise Test bits nonzero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtstq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vtstq_u8(a, b) }
}

/// Unsigned compare bitwise Test bits nonzero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtst_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vtst_u16(a, b) }
}

/// Unsigned compare bitwise Test bits nonzero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtstq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vtstq_u16(a, b) }
}

/// Unsigned compare bitwise Test bits nonzero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtst_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vtst_u32(a, b) }
}

/// Unsigned compare bitwise Test bits nonzero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtstq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vtstq_u32(a, b) }
}

/// Floating-point absolute value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabs_f32(a: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vabs_f32(a) }
}

/// Floating-point absolute value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabsq_f32(a: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vabsq_f32(a) }
}

/// Compare signed greater than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgt_s8(a: int8x8_t, b: int8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vcgt_s8(a, b) }
}

/// Compare signed greater than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtq_s8(a: int8x16_t, b: int8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vcgtq_s8(a, b) }
}

/// Compare signed greater than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgt_s16(a: int16x4_t, b: int16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vcgt_s16(a, b) }
}

/// Compare signed greater than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtq_s16(a: int16x8_t, b: int16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vcgtq_s16(a, b) }
}

/// Compare signed greater than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgt_s32(a: int32x2_t, b: int32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcgt_s32(a, b) }
}

/// Compare signed greater than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtq_s32(a: int32x4_t, b: int32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcgtq_s32(a, b) }
}

/// Compare unsigned highe
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgt_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vcgt_u8(a, b) }
}

/// Compare unsigned highe
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vcgtq_u8(a, b) }
}

/// Compare unsigned highe
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgt_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vcgt_u16(a, b) }
}

/// Compare unsigned highe
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vcgtq_u16(a, b) }
}

/// Compare unsigned highe
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgt_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcgt_u32(a, b) }
}

/// Compare unsigned highe
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcgtq_u32(a, b) }
}

/// Floating-point compare greater than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgt_f32(a: float32x2_t, b: float32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcgt_f32(a, b) }
}

/// Floating-point compare greater than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtq_f32(a: float32x4_t, b: float32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcgtq_f32(a, b) }
}

/// Compare signed less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclt_s8(a: int8x8_t, b: int8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vclt_s8(a, b) }
}

/// Compare signed less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltq_s8(a: int8x16_t, b: int8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vcltq_s8(a, b) }
}

/// Compare signed less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclt_s16(a: int16x4_t, b: int16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vclt_s16(a, b) }
}

/// Compare signed less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltq_s16(a: int16x8_t, b: int16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vcltq_s16(a, b) }
}

/// Compare signed less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclt_s32(a: int32x2_t, b: int32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vclt_s32(a, b) }
}

/// Compare signed less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltq_s32(a: int32x4_t, b: int32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcltq_s32(a, b) }
}

/// Compare unsigned less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclt_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vclt_u8(a, b) }
}

/// Compare unsigned less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vcltq_u8(a, b) }
}

/// Compare unsigned less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclt_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vclt_u16(a, b) }
}

/// Compare unsigned less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vcltq_u16(a, b) }
}

/// Compare unsigned less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclt_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vclt_u32(a, b) }
}

/// Compare unsigned less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcltq_u32(a, b) }
}

/// Floating-point compare less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclt_f32(a: float32x2_t, b: float32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vclt_f32(a, b) }
}

/// Floating-point compare less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltq_f32(a: float32x4_t, b: float32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcltq_f32(a, b) }
}

/// Compare signed less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcle_s8(a: int8x8_t, b: int8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vcle_s8(a, b) }
}

/// Compare signed less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcleq_s8(a: int8x16_t, b: int8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vcleq_s8(a, b) }
}

/// Compare signed less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcle_s16(a: int16x4_t, b: int16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vcle_s16(a, b) }
}

/// Compare signed less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcleq_s16(a: int16x8_t, b: int16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vcleq_s16(a, b) }
}

/// Compare signed less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcle_s32(a: int32x2_t, b: int32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcle_s32(a, b) }
}

/// Compare signed less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcleq_s32(a: int32x4_t, b: int32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcleq_s32(a, b) }
}

/// Compare unsigned less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcle_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vcle_u8(a, b) }
}

/// Compare unsigned less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcleq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vcleq_u8(a, b) }
}

/// Compare unsigned less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcle_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vcle_u16(a, b) }
}

/// Compare unsigned less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcleq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vcleq_u16(a, b) }
}

/// Compare unsigned less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcle_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcle_u32(a, b) }
}

/// Compare unsigned less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcleq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcleq_u32(a, b) }
}

/// Floating-point compare less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcle_f32(a: float32x2_t, b: float32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcle_f32(a, b) }
}

/// Floating-point compare less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcleq_f32(a: float32x4_t, b: float32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcleq_f32(a, b) }
}

/// Compare signed greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcge_s8(a: int8x8_t, b: int8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vcge_s8(a, b) }
}

/// Compare signed greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgeq_s8(a: int8x16_t, b: int8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vcgeq_s8(a, b) }
}

/// Compare signed greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcge_s16(a: int16x4_t, b: int16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vcge_s16(a, b) }
}

/// Compare signed greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgeq_s16(a: int16x8_t, b: int16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vcgeq_s16(a, b) }
}

/// Compare signed greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcge_s32(a: int32x2_t, b: int32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcge_s32(a, b) }
}

/// Compare signed greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgeq_s32(a: int32x4_t, b: int32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcgeq_s32(a, b) }
}

/// Compare unsigned greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcge_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vcge_u8(a, b) }
}

/// Compare unsigned greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgeq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vcgeq_u8(a, b) }
}

/// Compare unsigned greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcge_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vcge_u16(a, b) }
}

/// Compare unsigned greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgeq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vcgeq_u16(a, b) }
}

/// Compare unsigned greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcge_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcge_u32(a, b) }
}

/// Compare unsigned greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgeq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcgeq_u32(a, b) }
}

/// Floating-point compare greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcge_f32(a: float32x2_t, b: float32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcge_f32(a, b) }
}

/// Floating-point compare greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgeq_f32(a: float32x4_t, b: float32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcgeq_f32(a, b) }
}

/// Count leading sign bits
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcls_s8(a: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vcls_s8(a) }
}

/// Count leading sign bits
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclsq_s8(a: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vclsq_s8(a) }
}

/// Count leading sign bits
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcls_s16(a: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vcls_s16(a) }
}

/// Count leading sign bits
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclsq_s16(a: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vclsq_s16(a) }
}

/// Count leading sign bits
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcls_s32(a: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vcls_s32(a) }
}

/// Count leading sign bits
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclsq_s32(a: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vclsq_s32(a) }
}

/// Count leading sign bits
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcls_u8(a: uint8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vcls_u8(a) }
}

/// Count leading sign bits
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclsq_u8(a: uint8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vclsq_u8(a) }
}

/// Count leading sign bits
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcls_u16(a: uint16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vcls_u16(a) }
}

/// Count leading sign bits
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclsq_u16(a: uint16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vclsq_u16(a) }
}

/// Count leading sign bits
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcls_u32(a: uint32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vcls_u32(a) }
}

/// Count leading sign bits
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclsq_u32(a: uint32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vclsq_u32(a) }
}

/// Count leading zero bits
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclz_s8(a: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vclz_s8(a) }
}

/// Count leading zero bits
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclzq_s8(a: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vclzq_s8(a) }
}

/// Count leading zero bits
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclz_s16(a: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vclz_s16(a) }
}

/// Count leading zero bits
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclzq_s16(a: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vclzq_s16(a) }
}

/// Count leading zero bits
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclz_s32(a: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vclz_s32(a) }
}

/// Count leading zero bits
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclzq_s32(a: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vclzq_s32(a) }
}

/// Count leading zero bits
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclz_u8(a: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vclz_u8(a) }
}

/// Count leading zero bits
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclzq_u8(a: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vclzq_u8(a) }
}

/// Count leading zero bits
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclz_u16(a: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vclz_u16(a) }
}

/// Count leading zero bits
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclzq_u16(a: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vclzq_u16(a) }
}

/// Count leading zero bits
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclz_u32(a: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vclz_u32(a) }
}

/// Count leading zero bits
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclzq_u32(a: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vclzq_u32(a) }
}

/// Floating-point absolute compare greater than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcagt_f32(a: float32x2_t, b: float32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcagt_f32(a, b) }
}

/// Floating-point absolute compare greater than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcagtq_f32(a: float32x4_t, b: float32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcagtq_f32(a, b) }
}

/// Floating-point absolute compare greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcage_f32(a: float32x2_t, b: float32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcage_f32(a, b) }
}

/// Floating-point absolute compare greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcageq_f32(a: float32x4_t, b: float32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcageq_f32(a, b) }
}

/// Floating-point absolute compare less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcalt_f32(a: float32x2_t, b: float32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcalt_f32(a, b) }
}

/// Floating-point absolute compare less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcaltq_f32(a: float32x4_t, b: float32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcaltq_f32(a, b) }
}

/// Floating-point absolute compare less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcale_f32(a: float32x2_t, b: float32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcale_f32(a, b) }
}

/// Floating-point absolute compare less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcaleq_f32(a: float32x4_t, b: float32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcaleq_f32(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcreate_s8(a: u64) -> int8x8_t {
  unsafe { core::arch::aarch64::vcreate_s8(a) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcreate_s16(a: u64) -> int16x4_t {
  unsafe { core::arch::aarch64::vcreate_s16(a) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcreate_s32(a: u64) -> int32x2_t {
  unsafe { core::arch::aarch64::vcreate_s32(a) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcreate_s64(a: u64) -> int64x1_t {
  unsafe { core::arch::aarch64::vcreate_s64(a) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcreate_u8(a: u64) -> uint8x8_t {
  unsafe { core::arch::aarch64::vcreate_u8(a) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcreate_u16(a: u64) -> uint16x4_t {
  unsafe { core::arch::aarch64::vcreate_u16(a) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcreate_u32(a: u64) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcreate_u32(a) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcreate_u64(a: u64) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcreate_u64(a) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcreate_p8(a: u64) -> poly8x8_t {
  unsafe { core::arch::aarch64::vcreate_p8(a) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcreate_p16(a: u64) -> poly16x4_t {
  unsafe { core::arch::aarch64::vcreate_p16(a) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vcreate_p64(a: u64) -> poly64x1_t {
  unsafe { core::arch::aarch64::vcreate_p64(a) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcreate_f32(a: u64) -> float32x2_t {
  unsafe { core::arch::aarch64::vcreate_f32(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvt_f32_s32(a: int32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vcvt_f32_s32(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtq_f32_s32(a: int32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vcvtq_f32_s32(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvt_f32_u32(a: uint32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vcvt_f32_u32(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtq_f32_u32(a: uint32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vcvtq_f32_u32(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvt_n_f32_s32<const N: i32>(a: int32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vcvt_n_f32_s32::<N>(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtq_n_f32_s32<const N: i32>(a: int32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vcvtq_n_f32_s32::<N>(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvt_n_f32_u32<const N: i32>(a: uint32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vcvt_n_f32_u32::<N>(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtq_n_f32_u32<const N: i32>(a: uint32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vcvtq_n_f32_u32::<N>(a) }
}

/// Floating-point convert to fixed-point, rounding toward zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvt_n_s32_f32<const N: i32>(a: float32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vcvt_n_s32_f32::<N>(a) }
}

/// Floating-point convert to fixed-point, rounding toward zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtq_n_s32_f32<const N: i32>(a: float32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vcvtq_n_s32_f32::<N>(a) }
}

/// Floating-point convert to fixed-point, rounding toward zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvt_n_u32_f32<const N: i32>(a: float32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcvt_n_u32_f32::<N>(a) }
}

/// Floating-point convert to fixed-point, rounding toward zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtq_n_u32_f32<const N: i32>(a: float32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcvtq_n_u32_f32::<N>(a) }
}

/// Floating-point convert to signed fixed-point, rounding toward zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvt_s32_f32(a: float32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vcvt_s32_f32(a) }
}

/// Floating-point convert to signed fixed-point, rounding toward zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtq_s32_f32(a: float32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vcvtq_s32_f32(a) }
}

/// Floating-point convert to unsigned fixed-point, rounding toward zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvt_u32_f32(a: float32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcvt_u32_f32(a) }
}

/// Floating-point convert to unsigned fixed-point, rounding toward zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtq_u32_f32(a: float32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcvtq_u32_f32(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_lane_s8<const N: i32>(a: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vdup_lane_s8::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_laneq_s8<const N: i32>(a: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vdupq_laneq_s8::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_lane_s16<const N: i32>(a: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vdup_lane_s16::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_laneq_s16<const N: i32>(a: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vdupq_laneq_s16::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_lane_s32<const N: i32>(a: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vdup_lane_s32::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_laneq_s32<const N: i32>(a: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vdupq_laneq_s32::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_laneq_s8<const N: i32>(a: int8x16_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vdup_laneq_s8::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_laneq_s16<const N: i32>(a: int16x8_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vdup_laneq_s16::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_laneq_s32<const N: i32>(a: int32x4_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vdup_laneq_s32::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_lane_s8<const N: i32>(a: int8x8_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vdupq_lane_s8::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_lane_s16<const N: i32>(a: int16x4_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vdupq_lane_s16::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_lane_s32<const N: i32>(a: int32x2_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vdupq_lane_s32::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_lane_u8<const N: i32>(a: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vdup_lane_u8::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_laneq_u8<const N: i32>(a: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vdupq_laneq_u8::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_lane_u16<const N: i32>(a: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vdup_lane_u16::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_laneq_u16<const N: i32>(a: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vdupq_laneq_u16::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_lane_u32<const N: i32>(a: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vdup_lane_u32::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_laneq_u32<const N: i32>(a: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vdupq_laneq_u32::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_laneq_u8<const N: i32>(a: uint8x16_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vdup_laneq_u8::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_laneq_u16<const N: i32>(a: uint16x8_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vdup_laneq_u16::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_laneq_u32<const N: i32>(a: uint32x4_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vdup_laneq_u32::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_lane_u8<const N: i32>(a: uint8x8_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vdupq_lane_u8::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_lane_u16<const N: i32>(a: uint16x4_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vdupq_lane_u16::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_lane_u32<const N: i32>(a: uint32x2_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vdupq_lane_u32::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_lane_p8<const N: i32>(a: poly8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vdup_lane_p8::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_laneq_p8<const N: i32>(a: poly8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vdupq_laneq_p8::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_lane_p16<const N: i32>(a: poly16x4_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vdup_lane_p16::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_laneq_p16<const N: i32>(a: poly16x8_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vdupq_laneq_p16::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_laneq_p8<const N: i32>(a: poly8x16_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vdup_laneq_p8::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_laneq_p16<const N: i32>(a: poly16x8_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vdup_laneq_p16::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_lane_p8<const N: i32>(a: poly8x8_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vdupq_lane_p8::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_lane_p16<const N: i32>(a: poly16x4_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vdupq_lane_p16::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_laneq_s64<const N: i32>(a: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vdupq_laneq_s64::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_lane_s64<const N: i32>(a: int64x1_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vdupq_lane_s64::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_laneq_u64<const N: i32>(a: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vdupq_laneq_u64::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_lane_u64<const N: i32>(a: uint64x1_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vdupq_lane_u64::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_lane_f32<const N: i32>(a: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vdup_lane_f32::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_laneq_f32<const N: i32>(a: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vdupq_laneq_f32::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_laneq_f32<const N: i32>(a: float32x4_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vdup_laneq_f32::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_lane_f32<const N: i32>(a: float32x2_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vdupq_lane_f32::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_lane_s64<const N: i32>(a: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vdup_lane_s64::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_lane_u64<const N: i32>(a: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vdup_lane_u64::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_laneq_s64<const N: i32>(a: int64x2_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vdup_laneq_s64::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_laneq_u64<const N: i32>(a: uint64x2_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vdup_laneq_u64::<N>(a) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vext_s8<const N: i32>(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vext_s8::<N>(a, b) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vextq_s8<const N: i32>(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vextq_s8::<N>(a, b) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vext_s16<const N: i32>(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vext_s16::<N>(a, b) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vextq_s16<const N: i32>(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vextq_s16::<N>(a, b) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vext_s32<const N: i32>(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vext_s32::<N>(a, b) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vextq_s32<const N: i32>(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vextq_s32::<N>(a, b) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vext_u8<const N: i32>(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vext_u8::<N>(a, b) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vextq_u8<const N: i32>(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vextq_u8::<N>(a, b) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vext_u16<const N: i32>(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vext_u16::<N>(a, b) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vextq_u16<const N: i32>(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vextq_u16::<N>(a, b) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vext_u32<const N: i32>(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vext_u32::<N>(a, b) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vextq_u32<const N: i32>(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vextq_u32::<N>(a, b) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vext_p8<const N: i32>(a: poly8x8_t, b: poly8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vext_p8::<N>(a, b) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vextq_p8<const N: i32>(a: poly8x16_t, b: poly8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vextq_p8::<N>(a, b) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vext_p16<const N: i32>(a: poly16x4_t, b: poly16x4_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vext_p16::<N>(a, b) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vextq_p16<const N: i32>(a: poly16x8_t, b: poly16x8_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vextq_p16::<N>(a, b) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vextq_s64<const N: i32>(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vextq_s64::<N>(a, b) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vextq_u64<const N: i32>(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vextq_u64::<N>(a, b) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vext_f32<const N: i32>(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vext_f32::<N>(a, b) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vextq_f32<const N: i32>(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vextq_f32::<N>(a, b) }
}

/// Multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmla_s8(a: int8x8_t, b: int8x8_t, c: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vmla_s8(a, b, c) }
}

/// Multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlaq_s8(a: int8x16_t, b: int8x16_t, c: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vmlaq_s8(a, b, c) }
}

/// Multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmla_s16(a: int16x4_t, b: int16x4_t, c: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vmla_s16(a, b, c) }
}

/// Multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlaq_s16(a: int16x8_t, b: int16x8_t, c: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vmlaq_s16(a, b, c) }
}

/// Multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmla_s32(a: int32x2_t, b: int32x2_t, c: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vmla_s32(a, b, c) }
}

/// Multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlaq_s32(a: int32x4_t, b: int32x4_t, c: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmlaq_s32(a, b, c) }
}

/// Multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmla_u8(a: uint8x8_t, b: uint8x8_t, c: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vmla_u8(a, b, c) }
}

/// Multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlaq_u8(a: uint8x16_t, b: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vmlaq_u8(a, b, c) }
}

/// Multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmla_u16(a: uint16x4_t, b: uint16x4_t, c: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vmla_u16(a, b, c) }
}

/// Multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlaq_u16(a: uint16x8_t, b: uint16x8_t, c: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vmlaq_u16(a, b, c) }
}

/// Multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmla_u32(a: uint32x2_t, b: uint32x2_t, c: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vmla_u32(a, b, c) }
}

/// Multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlaq_u32(a: uint32x4_t, b: uint32x4_t, c: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmlaq_u32(a, b, c) }
}

/// Floating-point multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmla_f32(a: float32x2_t, b: float32x2_t, c: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vmla_f32(a, b, c) }
}

/// Floating-point multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlaq_f32(a: float32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vmlaq_f32(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmla_n_s16(a: int16x4_t, b: int16x4_t, c: i16) -> int16x4_t {
  unsafe { core::arch::aarch64::vmla_n_s16(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlaq_n_s16(a: int16x8_t, b: int16x8_t, c: i16) -> int16x8_t {
  unsafe { core::arch::aarch64::vmlaq_n_s16(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmla_n_s32(a: int32x2_t, b: int32x2_t, c: i32) -> int32x2_t {
  unsafe { core::arch::aarch64::vmla_n_s32(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlaq_n_s32(a: int32x4_t, b: int32x4_t, c: i32) -> int32x4_t {
  unsafe { core::arch::aarch64::vmlaq_n_s32(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmla_n_u16(a: uint16x4_t, b: uint16x4_t, c: u16) -> uint16x4_t {
  unsafe { core::arch::aarch64::vmla_n_u16(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlaq_n_u16(a: uint16x8_t, b: uint16x8_t, c: u16) -> uint16x8_t {
  unsafe { core::arch::aarch64::vmlaq_n_u16(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmla_n_u32(a: uint32x2_t, b: uint32x2_t, c: u32) -> uint32x2_t {
  unsafe { core::arch::aarch64::vmla_n_u32(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlaq_n_u32(a: uint32x4_t, b: uint32x4_t, c: u32) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmlaq_n_u32(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmla_n_f32(a: float32x2_t, b: float32x2_t, c: f32) -> float32x2_t {
  unsafe { core::arch::aarch64::vmla_n_f32(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlaq_n_f32(a: float32x4_t, b: float32x4_t, c: f32) -> float32x4_t {
  unsafe { core::arch::aarch64::vmlaq_n_f32(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmla_lane_s16<const LANE: i32>(a: int16x4_t, b: int16x4_t, c: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vmla_lane_s16::<LANE>(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmla_laneq_s16<const LANE: i32>(a: int16x4_t, b: int16x4_t, c: int16x8_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vmla_laneq_s16::<LANE>(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlaq_lane_s16<const LANE: i32>(a: int16x8_t, b: int16x8_t, c: int16x4_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vmlaq_lane_s16::<LANE>(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlaq_laneq_s16<const LANE: i32>(a: int16x8_t, b: int16x8_t, c: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vmlaq_laneq_s16::<LANE>(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmla_lane_s32<const LANE: i32>(a: int32x2_t, b: int32x2_t, c: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vmla_lane_s32::<LANE>(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmla_laneq_s32<const LANE: i32>(a: int32x2_t, b: int32x2_t, c: int32x4_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vmla_laneq_s32::<LANE>(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlaq_lane_s32<const LANE: i32>(a: int32x4_t, b: int32x4_t, c: int32x2_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmlaq_lane_s32::<LANE>(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlaq_laneq_s32<const LANE: i32>(a: int32x4_t, b: int32x4_t, c: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmlaq_laneq_s32::<LANE>(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmla_lane_u16<const LANE: i32>(a: uint16x4_t, b: uint16x4_t, c: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vmla_lane_u16::<LANE>(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmla_laneq_u16<const LANE: i32>(a: uint16x4_t, b: uint16x4_t, c: uint16x8_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vmla_laneq_u16::<LANE>(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlaq_lane_u16<const LANE: i32>(a: uint16x8_t, b: uint16x8_t, c: uint16x4_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vmlaq_lane_u16::<LANE>(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlaq_laneq_u16<const LANE: i32>(a: uint16x8_t, b: uint16x8_t, c: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vmlaq_laneq_u16::<LANE>(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmla_lane_u32<const LANE: i32>(a: uint32x2_t, b: uint32x2_t, c: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vmla_lane_u32::<LANE>(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmla_laneq_u32<const LANE: i32>(a: uint32x2_t, b: uint32x2_t, c: uint32x4_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vmla_laneq_u32::<LANE>(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlaq_lane_u32<const LANE: i32>(a: uint32x4_t, b: uint32x4_t, c: uint32x2_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmlaq_lane_u32::<LANE>(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlaq_laneq_u32<const LANE: i32>(a: uint32x4_t, b: uint32x4_t, c: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmlaq_laneq_u32::<LANE>(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmla_lane_f32<const LANE: i32>(a: float32x2_t, b: float32x2_t, c: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vmla_lane_f32::<LANE>(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmla_laneq_f32<const LANE: i32>(a: float32x2_t, b: float32x2_t, c: float32x4_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vmla_laneq_f32::<LANE>(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlaq_lane_f32<const LANE: i32>(a: float32x4_t, b: float32x4_t, c: float32x2_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vmlaq_lane_f32::<LANE>(a, b, c) }
}

/// Vector multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlaq_laneq_f32<const LANE: i32>(a: float32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vmlaq_laneq_f32::<LANE>(a, b, c) }
}

/// Signed multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_s8(a: int16x8_t, b: int8x8_t, c: int8x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vmlal_s8(a, b, c) }
}

/// Signed multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_s16(a: int32x4_t, b: int16x4_t, c: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmlal_s16(a, b, c) }
}

/// Signed multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_s32(a: int64x2_t, b: int32x2_t, c: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vmlal_s32(a, b, c) }
}

/// Unsigned multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_u8(a: uint16x8_t, b: uint8x8_t, c: uint8x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vmlal_u8(a, b, c) }
}

/// Unsigned multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_u16(a: uint32x4_t, b: uint16x4_t, c: uint16x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmlal_u16(a, b, c) }
}

/// Unsigned multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_u32(a: uint64x2_t, b: uint32x2_t, c: uint32x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmlal_u32(a, b, c) }
}

/// Vector widening multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_n_s16(a: int32x4_t, b: int16x4_t, c: i16) -> int32x4_t {
  unsafe { core::arch::aarch64::vmlal_n_s16(a, b, c) }
}

/// Vector widening multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_n_s32(a: int64x2_t, b: int32x2_t, c: i32) -> int64x2_t {
  unsafe { core::arch::aarch64::vmlal_n_s32(a, b, c) }
}

/// Vector widening multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_n_u16(a: uint32x4_t, b: uint16x4_t, c: u16) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmlal_n_u16(a, b, c) }
}

/// Vector widening multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_n_u32(a: uint64x2_t, b: uint32x2_t, c: u32) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmlal_n_u32(a, b, c) }
}

/// Vector widening multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_lane_s16<const LANE: i32>(a: int32x4_t, b: int16x4_t, c: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmlal_lane_s16::<LANE>(a, b, c) }
}

/// Vector widening multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_laneq_s16<const LANE: i32>(a: int32x4_t, b: int16x4_t, c: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmlal_laneq_s16::<LANE>(a, b, c) }
}

/// Vector widening multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_lane_s32<const LANE: i32>(a: int64x2_t, b: int32x2_t, c: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vmlal_lane_s32::<LANE>(a, b, c) }
}

/// Vector widening multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_laneq_s32<const LANE: i32>(a: int64x2_t, b: int32x2_t, c: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vmlal_laneq_s32::<LANE>(a, b, c) }
}

/// Vector widening multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_lane_u16<const LANE: i32>(a: uint32x4_t, b: uint16x4_t, c: uint16x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmlal_lane_u16::<LANE>(a, b, c) }
}

/// Vector widening multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_laneq_u16<const LANE: i32>(a: uint32x4_t, b: uint16x4_t, c: uint16x8_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmlal_laneq_u16::<LANE>(a, b, c) }
}

/// Vector widening multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_lane_u32<const LANE: i32>(a: uint64x2_t, b: uint32x2_t, c: uint32x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmlal_lane_u32::<LANE>(a, b, c) }
}

/// Vector widening multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_laneq_u32<const LANE: i32>(a: uint64x2_t, b: uint32x2_t, c: uint32x4_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmlal_laneq_u32::<LANE>(a, b, c) }
}

/// Multiply-subtract from accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmls_s8(a: int8x8_t, b: int8x8_t, c: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vmls_s8(a, b, c) }
}

/// Multiply-subtract from accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsq_s8(a: int8x16_t, b: int8x16_t, c: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vmlsq_s8(a, b, c) }
}

/// Multiply-subtract from accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmls_s16(a: int16x4_t, b: int16x4_t, c: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vmls_s16(a, b, c) }
}

/// Multiply-subtract from accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsq_s16(a: int16x8_t, b: int16x8_t, c: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vmlsq_s16(a, b, c) }
}

/// Multiply-subtract from accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmls_s32(a: int32x2_t, b: int32x2_t, c: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vmls_s32(a, b, c) }
}

/// Multiply-subtract from accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsq_s32(a: int32x4_t, b: int32x4_t, c: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmlsq_s32(a, b, c) }
}

/// Multiply-subtract from accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmls_u8(a: uint8x8_t, b: uint8x8_t, c: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vmls_u8(a, b, c) }
}

/// Multiply-subtract from accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsq_u8(a: uint8x16_t, b: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vmlsq_u8(a, b, c) }
}

/// Multiply-subtract from accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmls_u16(a: uint16x4_t, b: uint16x4_t, c: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vmls_u16(a, b, c) }
}

/// Multiply-subtract from accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsq_u16(a: uint16x8_t, b: uint16x8_t, c: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vmlsq_u16(a, b, c) }
}

/// Multiply-subtract from accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmls_u32(a: uint32x2_t, b: uint32x2_t, c: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vmls_u32(a, b, c) }
}

/// Multiply-subtract from accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsq_u32(a: uint32x4_t, b: uint32x4_t, c: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmlsq_u32(a, b, c) }
}

/// Floating-point multiply-subtract from accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmls_f32(a: float32x2_t, b: float32x2_t, c: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vmls_f32(a, b, c) }
}

/// Floating-point multiply-subtract from accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsq_f32(a: float32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vmlsq_f32(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmls_n_s16(a: int16x4_t, b: int16x4_t, c: i16) -> int16x4_t {
  unsafe { core::arch::aarch64::vmls_n_s16(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsq_n_s16(a: int16x8_t, b: int16x8_t, c: i16) -> int16x8_t {
  unsafe { core::arch::aarch64::vmlsq_n_s16(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmls_n_s32(a: int32x2_t, b: int32x2_t, c: i32) -> int32x2_t {
  unsafe { core::arch::aarch64::vmls_n_s32(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsq_n_s32(a: int32x4_t, b: int32x4_t, c: i32) -> int32x4_t {
  unsafe { core::arch::aarch64::vmlsq_n_s32(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmls_n_u16(a: uint16x4_t, b: uint16x4_t, c: u16) -> uint16x4_t {
  unsafe { core::arch::aarch64::vmls_n_u16(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsq_n_u16(a: uint16x8_t, b: uint16x8_t, c: u16) -> uint16x8_t {
  unsafe { core::arch::aarch64::vmlsq_n_u16(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmls_n_u32(a: uint32x2_t, b: uint32x2_t, c: u32) -> uint32x2_t {
  unsafe { core::arch::aarch64::vmls_n_u32(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsq_n_u32(a: uint32x4_t, b: uint32x4_t, c: u32) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmlsq_n_u32(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmls_n_f32(a: float32x2_t, b: float32x2_t, c: f32) -> float32x2_t {
  unsafe { core::arch::aarch64::vmls_n_f32(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsq_n_f32(a: float32x4_t, b: float32x4_t, c: f32) -> float32x4_t {
  unsafe { core::arch::aarch64::vmlsq_n_f32(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmls_lane_s16<const LANE: i32>(a: int16x4_t, b: int16x4_t, c: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vmls_lane_s16::<LANE>(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmls_laneq_s16<const LANE: i32>(a: int16x4_t, b: int16x4_t, c: int16x8_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vmls_laneq_s16::<LANE>(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsq_lane_s16<const LANE: i32>(a: int16x8_t, b: int16x8_t, c: int16x4_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vmlsq_lane_s16::<LANE>(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsq_laneq_s16<const LANE: i32>(a: int16x8_t, b: int16x8_t, c: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vmlsq_laneq_s16::<LANE>(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmls_lane_s32<const LANE: i32>(a: int32x2_t, b: int32x2_t, c: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vmls_lane_s32::<LANE>(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmls_laneq_s32<const LANE: i32>(a: int32x2_t, b: int32x2_t, c: int32x4_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vmls_laneq_s32::<LANE>(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsq_lane_s32<const LANE: i32>(a: int32x4_t, b: int32x4_t, c: int32x2_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmlsq_lane_s32::<LANE>(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsq_laneq_s32<const LANE: i32>(a: int32x4_t, b: int32x4_t, c: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmlsq_laneq_s32::<LANE>(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmls_lane_u16<const LANE: i32>(a: uint16x4_t, b: uint16x4_t, c: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vmls_lane_u16::<LANE>(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmls_laneq_u16<const LANE: i32>(a: uint16x4_t, b: uint16x4_t, c: uint16x8_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vmls_laneq_u16::<LANE>(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsq_lane_u16<const LANE: i32>(a: uint16x8_t, b: uint16x8_t, c: uint16x4_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vmlsq_lane_u16::<LANE>(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsq_laneq_u16<const LANE: i32>(a: uint16x8_t, b: uint16x8_t, c: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vmlsq_laneq_u16::<LANE>(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmls_lane_u32<const LANE: i32>(a: uint32x2_t, b: uint32x2_t, c: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vmls_lane_u32::<LANE>(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmls_laneq_u32<const LANE: i32>(a: uint32x2_t, b: uint32x2_t, c: uint32x4_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vmls_laneq_u32::<LANE>(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsq_lane_u32<const LANE: i32>(a: uint32x4_t, b: uint32x4_t, c: uint32x2_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmlsq_lane_u32::<LANE>(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsq_laneq_u32<const LANE: i32>(a: uint32x4_t, b: uint32x4_t, c: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmlsq_laneq_u32::<LANE>(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmls_lane_f32<const LANE: i32>(a: float32x2_t, b: float32x2_t, c: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vmls_lane_f32::<LANE>(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmls_laneq_f32<const LANE: i32>(a: float32x2_t, b: float32x2_t, c: float32x4_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vmls_laneq_f32::<LANE>(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsq_lane_f32<const LANE: i32>(a: float32x4_t, b: float32x4_t, c: float32x2_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vmlsq_lane_f32::<LANE>(a, b, c) }
}

/// Vector multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsq_laneq_f32<const LANE: i32>(a: float32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vmlsq_laneq_f32::<LANE>(a, b, c) }
}

/// Signed multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_s8(a: int16x8_t, b: int8x8_t, c: int8x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vmlsl_s8(a, b, c) }
}

/// Signed multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_s16(a: int32x4_t, b: int16x4_t, c: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmlsl_s16(a, b, c) }
}

/// Signed multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_s32(a: int64x2_t, b: int32x2_t, c: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vmlsl_s32(a, b, c) }
}

/// Unsigned multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_u8(a: uint16x8_t, b: uint8x8_t, c: uint8x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vmlsl_u8(a, b, c) }
}

/// Unsigned multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_u16(a: uint32x4_t, b: uint16x4_t, c: uint16x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmlsl_u16(a, b, c) }
}

/// Unsigned multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_u32(a: uint64x2_t, b: uint32x2_t, c: uint32x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmlsl_u32(a, b, c) }
}

/// Vector widening multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_n_s16(a: int32x4_t, b: int16x4_t, c: i16) -> int32x4_t {
  unsafe { core::arch::aarch64::vmlsl_n_s16(a, b, c) }
}

/// Vector widening multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_n_s32(a: int64x2_t, b: int32x2_t, c: i32) -> int64x2_t {
  unsafe { core::arch::aarch64::vmlsl_n_s32(a, b, c) }
}

/// Vector widening multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_n_u16(a: uint32x4_t, b: uint16x4_t, c: u16) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmlsl_n_u16(a, b, c) }
}

/// Vector widening multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_n_u32(a: uint64x2_t, b: uint32x2_t, c: u32) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmlsl_n_u32(a, b, c) }
}

/// Vector widening multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_lane_s16<const LANE: i32>(a: int32x4_t, b: int16x4_t, c: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmlsl_lane_s16::<LANE>(a, b, c) }
}

/// Vector widening multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_laneq_s16<const LANE: i32>(a: int32x4_t, b: int16x4_t, c: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmlsl_laneq_s16::<LANE>(a, b, c) }
}

/// Vector widening multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_lane_s32<const LANE: i32>(a: int64x2_t, b: int32x2_t, c: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vmlsl_lane_s32::<LANE>(a, b, c) }
}

/// Vector widening multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_laneq_s32<const LANE: i32>(a: int64x2_t, b: int32x2_t, c: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vmlsl_laneq_s32::<LANE>(a, b, c) }
}

/// Vector widening multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_lane_u16<const LANE: i32>(a: uint32x4_t, b: uint16x4_t, c: uint16x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmlsl_lane_u16::<LANE>(a, b, c) }
}

/// Vector widening multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_laneq_u16<const LANE: i32>(a: uint32x4_t, b: uint16x4_t, c: uint16x8_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmlsl_laneq_u16::<LANE>(a, b, c) }
}

/// Vector widening multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_lane_u32<const LANE: i32>(a: uint64x2_t, b: uint32x2_t, c: uint32x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmlsl_lane_u32::<LANE>(a, b, c) }
}

/// Vector widening multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_laneq_u32<const LANE: i32>(a: uint64x2_t, b: uint32x2_t, c: uint32x4_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmlsl_laneq_u32::<LANE>(a, b, c) }
}

/// Negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vneg_s8(a: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vneg_s8(a) }
}

/// Negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vnegq_s8(a: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vnegq_s8(a) }
}

/// Negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vneg_s16(a: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vneg_s16(a) }
}

/// Negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vnegq_s16(a: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vnegq_s16(a) }
}

/// Negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vneg_s32(a: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vneg_s32(a) }
}

/// Negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vnegq_s32(a: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vnegq_s32(a) }
}

/// Negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vneg_f32(a: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vneg_f32(a) }
}

/// Negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vnegq_f32(a: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vnegq_f32(a) }
}

/// Signed saturating negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqneg_s8(a: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vqneg_s8(a) }
}

/// Signed saturating negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqnegq_s8(a: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vqnegq_s8(a) }
}

/// Signed saturating negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqneg_s16(a: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vqneg_s16(a) }
}

/// Signed saturating negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqnegq_s16(a: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vqnegq_s16(a) }
}

/// Signed saturating negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqneg_s32(a: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vqneg_s32(a) }
}

/// Signed saturating negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqnegq_s32(a: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqnegq_s32(a) }
}

/// Saturating subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqsub_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vqsub_u8(a, b) }
}

/// Saturating subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqsubq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vqsubq_u8(a, b) }
}

/// Saturating subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqsub_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vqsub_u16(a, b) }
}

/// Saturating subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqsubq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vqsubq_u16(a, b) }
}

/// Saturating subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqsub_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vqsub_u32(a, b) }
}

/// Saturating subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqsubq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vqsubq_u32(a, b) }
}

/// Saturating subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqsub_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vqsub_u64(a, b) }
}

/// Saturating subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqsubq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vqsubq_u64(a, b) }
}

/// Saturating subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqsub_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vqsub_s8(a, b) }
}

/// Saturating subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqsubq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vqsubq_s8(a, b) }
}

/// Saturating subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqsub_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vqsub_s16(a, b) }
}

/// Saturating subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqsubq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vqsubq_s16(a, b) }
}

/// Saturating subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqsub_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vqsub_s32(a, b) }
}

/// Saturating subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqsubq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqsubq_s32(a, b) }
}

/// Saturating subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqsub_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vqsub_s64(a, b) }
}

/// Saturating subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqsubq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqsubq_s64(a, b) }
}

/// Halving add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vhadd_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vhadd_u8(a, b) }
}

/// Halving add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vhaddq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vhaddq_u8(a, b) }
}

/// Halving add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vhadd_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vhadd_u16(a, b) }
}

/// Halving add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vhaddq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vhaddq_u16(a, b) }
}

/// Halving add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vhadd_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vhadd_u32(a, b) }
}

/// Halving add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vhaddq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vhaddq_u32(a, b) }
}

/// Halving add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vhadd_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vhadd_s8(a, b) }
}

/// Halving add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vhaddq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vhaddq_s8(a, b) }
}

/// Halving add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vhadd_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vhadd_s16(a, b) }
}

/// Halving add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vhaddq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vhaddq_s16(a, b) }
}

/// Halving add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vhadd_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vhadd_s32(a, b) }
}

/// Halving add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vhaddq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vhaddq_s32(a, b) }
}

/// Rounding halving add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrhadd_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vrhadd_u8(a, b) }
}

/// Rounding halving add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrhaddq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vrhaddq_u8(a, b) }
}

/// Rounding halving add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrhadd_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vrhadd_u16(a, b) }
}

/// Rounding halving add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrhaddq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vrhaddq_u16(a, b) }
}

/// Rounding halving add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrhadd_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vrhadd_u32(a, b) }
}

/// Rounding halving add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrhaddq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vrhaddq_u32(a, b) }
}

/// Rounding halving add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrhadd_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vrhadd_s8(a, b) }
}

/// Rounding halving add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrhaddq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vrhaddq_s8(a, b) }
}

/// Rounding halving add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrhadd_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vrhadd_s16(a, b) }
}

/// Rounding halving add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrhaddq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vrhaddq_s16(a, b) }
}

/// Rounding halving add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrhadd_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vrhadd_s32(a, b) }
}

/// Rounding halving add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrhaddq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vrhaddq_s32(a, b) }
}

/// Floating-point round to integral, to nearest with ties to even
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndn_f32(a: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vrndn_f32(a) }
}

/// Floating-point round to integral, to nearest with ties to even
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndnq_f32(a: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vrndnq_f32(a) }
}

/// Saturating add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqadd_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vqadd_u8(a, b) }
}

/// Saturating add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqaddq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vqaddq_u8(a, b) }
}

/// Saturating add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqadd_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vqadd_u16(a, b) }
}

/// Saturating add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqaddq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vqaddq_u16(a, b) }
}

/// Saturating add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqadd_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vqadd_u32(a, b) }
}

/// Saturating add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqaddq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vqaddq_u32(a, b) }
}

/// Saturating add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqadd_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vqadd_u64(a, b) }
}

/// Saturating add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqaddq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vqaddq_u64(a, b) }
}

/// Saturating add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqadd_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vqadd_s8(a, b) }
}

/// Saturating add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqaddq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vqaddq_s8(a, b) }
}

/// Saturating add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqadd_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vqadd_s16(a, b) }
}

/// Saturating add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqaddq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vqaddq_s16(a, b) }
}

/// Saturating add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqadd_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vqadd_s32(a, b) }
}

/// Saturating add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqaddq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqaddq_s32(a, b) }
}

/// Saturating add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqadd_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vqadd_s64(a, b) }
}

/// Saturating add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqaddq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqaddq_s64(a, b) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_s8_x2(a: &[i8; 16]) -> int8x8x2_t {
  unsafe { core::arch::aarch64::vld1_s8_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_s16_x2(a: &[i16; 8]) -> int16x4x2_t {
  unsafe { core::arch::aarch64::vld1_s16_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_s32_x2(a: &[i32; 4]) -> int32x2x2_t {
  unsafe { core::arch::aarch64::vld1_s32_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_s64_x2(a: &[i64; 2]) -> int64x1x2_t {
  unsafe { core::arch::aarch64::vld1_s64_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_s8_x2(a: &[i8; 32]) -> int8x16x2_t {
  unsafe { core::arch::aarch64::vld1q_s8_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_s16_x2(a: &[i16; 16]) -> int16x8x2_t {
  unsafe { core::arch::aarch64::vld1q_s16_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_s32_x2(a: &[i32; 8]) -> int32x4x2_t {
  unsafe { core::arch::aarch64::vld1q_s32_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_s64_x2(a: &[i64; 4]) -> int64x2x2_t {
  unsafe { core::arch::aarch64::vld1q_s64_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_s8_x3(a: &[i8; 24]) -> int8x8x3_t {
  unsafe { core::arch::aarch64::vld1_s8_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_s16_x3(a: &[i16; 12]) -> int16x4x3_t {
  unsafe { core::arch::aarch64::vld1_s16_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_s32_x3(a: &[i32; 6]) -> int32x2x3_t {
  unsafe { core::arch::aarch64::vld1_s32_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_s64_x3(a: &[i64; 3]) -> int64x1x3_t {
  unsafe { core::arch::aarch64::vld1_s64_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_s8_x3(a: &[i8; 48]) -> int8x16x3_t {
  unsafe { core::arch::aarch64::vld1q_s8_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_s16_x3(a: &[i16; 24]) -> int16x8x3_t {
  unsafe { core::arch::aarch64::vld1q_s16_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_s32_x3(a: &[i32; 12]) -> int32x4x3_t {
  unsafe { core::arch::aarch64::vld1q_s32_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_s64_x3(a: &[i64; 6]) -> int64x2x3_t {
  unsafe { core::arch::aarch64::vld1q_s64_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_s8_x4(a: &[i8; 32]) -> int8x8x4_t {
  unsafe { core::arch::aarch64::vld1_s8_x4(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_s16_x4(a: &[i16; 16]) -> int16x4x4_t {
  unsafe { core::arch::aarch64::vld1_s16_x4(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_s32_x4(a: &[i32; 8]) -> int32x2x4_t {
  unsafe { core::arch::aarch64::vld1_s32_x4(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_s64_x4(a: &[i64; 4]) -> int64x1x4_t {
  unsafe { core::arch::aarch64::vld1_s64_x4(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_s8_x4(a: &[i8; 64]) -> int8x16x4_t {
  unsafe { core::arch::aarch64::vld1q_s8_x4(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_s16_x4(a: &[i16; 32]) -> int16x8x4_t {
  unsafe { core::arch::aarch64::vld1q_s16_x4(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_s32_x4(a: &[i32; 16]) -> int32x4x4_t {
  unsafe { core::arch::aarch64::vld1q_s32_x4(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_s64_x4(a: &[i64; 8]) -> int64x2x4_t {
  unsafe { core::arch::aarch64::vld1q_s64_x4(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_u8_x2(a: &[u8; 16]) -> uint8x8x2_t {
  unsafe { core::arch::aarch64::vld1_u8_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_u16_x2(a: &[u16; 8]) -> uint16x4x2_t {
  unsafe { core::arch::aarch64::vld1_u16_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_u32_x2(a: &[u32; 4]) -> uint32x2x2_t {
  unsafe { core::arch::aarch64::vld1_u32_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_u64_x2(a: &[u64; 2]) -> uint64x1x2_t {
  unsafe { core::arch::aarch64::vld1_u64_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_u8_x2(a: &[u8; 32]) -> uint8x16x2_t {
  unsafe { core::arch::aarch64::vld1q_u8_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_u16_x2(a: &[u16; 16]) -> uint16x8x2_t {
  unsafe { core::arch::aarch64::vld1q_u16_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_u32_x2(a: &[u32; 8]) -> uint32x4x2_t {
  unsafe { core::arch::aarch64::vld1q_u32_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_u64_x2(a: &[u64; 4]) -> uint64x2x2_t {
  unsafe { core::arch::aarch64::vld1q_u64_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_u8_x3(a: &[u8; 24]) -> uint8x8x3_t {
  unsafe { core::arch::aarch64::vld1_u8_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_u16_x3(a: &[u16; 12]) -> uint16x4x3_t {
  unsafe { core::arch::aarch64::vld1_u16_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_u32_x3(a: &[u32; 6]) -> uint32x2x3_t {
  unsafe { core::arch::aarch64::vld1_u32_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_u64_x3(a: &[u64; 3]) -> uint64x1x3_t {
  unsafe { core::arch::aarch64::vld1_u64_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_u8_x3(a: &[u8; 48]) -> uint8x16x3_t {
  unsafe { core::arch::aarch64::vld1q_u8_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_u16_x3(a: &[u16; 24]) -> uint16x8x3_t {
  unsafe { core::arch::aarch64::vld1q_u16_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_u32_x3(a: &[u32; 12]) -> uint32x4x3_t {
  unsafe { core::arch::aarch64::vld1q_u32_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_u64_x3(a: &[u64; 6]) -> uint64x2x3_t {
  unsafe { core::arch::aarch64::vld1q_u64_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_u8_x4(a: &[u8; 32]) -> uint8x8x4_t {
  unsafe { core::arch::aarch64::vld1_u8_x4(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_u16_x4(a: &[u16; 16]) -> uint16x4x4_t {
  unsafe { core::arch::aarch64::vld1_u16_x4(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_u32_x4(a: &[u32; 8]) -> uint32x2x4_t {
  unsafe { core::arch::aarch64::vld1_u32_x4(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_u64_x4(a: &[u64; 4]) -> uint64x1x4_t {
  unsafe { core::arch::aarch64::vld1_u64_x4(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_u8_x4(a: &[u8; 64]) -> uint8x16x4_t {
  unsafe { core::arch::aarch64::vld1q_u8_x4(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_u16_x4(a: &[u16; 32]) -> uint16x8x4_t {
  unsafe { core::arch::aarch64::vld1q_u16_x4(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_u32_x4(a: &[u32; 16]) -> uint32x4x4_t {
  unsafe { core::arch::aarch64::vld1q_u32_x4(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_u64_x4(a: &[u64; 8]) -> uint64x2x4_t {
  unsafe { core::arch::aarch64::vld1q_u64_x4(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_p8_x2(a: &[p8; 16]) -> poly8x8x2_t {
  unsafe { core::arch::aarch64::vld1_p8_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_p8_x3(a: &[p8; 24]) -> poly8x8x3_t {
  unsafe { core::arch::aarch64::vld1_p8_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_p8_x4(a: &[p8; 32]) -> poly8x8x4_t {
  unsafe { core::arch::aarch64::vld1_p8_x4(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_p8_x2(a: &[p8; 32]) -> poly8x16x2_t {
  unsafe { core::arch::aarch64::vld1q_p8_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_p8_x3(a: &[p8; 48]) -> poly8x16x3_t {
  unsafe { core::arch::aarch64::vld1q_p8_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_p8_x4(a: &[p8; 64]) -> poly8x16x4_t {
  unsafe { core::arch::aarch64::vld1q_p8_x4(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_p16_x2(a: &[p16; 8]) -> poly16x4x2_t {
  unsafe { core::arch::aarch64::vld1_p16_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_p16_x3(a: &[p16; 12]) -> poly16x4x3_t {
  unsafe { core::arch::aarch64::vld1_p16_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_p16_x4(a: &[p16; 16]) -> poly16x4x4_t {
  unsafe { core::arch::aarch64::vld1_p16_x4(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_p16_x2(a: &[p16; 16]) -> poly16x8x2_t {
  unsafe { core::arch::aarch64::vld1q_p16_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_p16_x3(a: &[p16; 24]) -> poly16x8x3_t {
  unsafe { core::arch::aarch64::vld1q_p16_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_p16_x4(a: &[p16; 32]) -> poly16x8x4_t {
  unsafe { core::arch::aarch64::vld1q_p16_x4(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld1_p64_x2(a: &[p64; 2]) -> poly64x1x2_t {
  unsafe { core::arch::aarch64::vld1_p64_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld1_p64_x3(a: &[p64; 3]) -> poly64x1x3_t {
  unsafe { core::arch::aarch64::vld1_p64_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld1_p64_x4(a: &[p64; 4]) -> poly64x1x4_t {
  unsafe { core::arch::aarch64::vld1_p64_x4(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld1q_p64_x2(a: &[p64; 4]) -> poly64x2x2_t {
  unsafe { core::arch::aarch64::vld1q_p64_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld1q_p64_x3(a: &[p64; 6]) -> poly64x2x3_t {
  unsafe { core::arch::aarch64::vld1q_p64_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld1q_p64_x4(a: &[p64; 8]) -> poly64x2x4_t {
  unsafe { core::arch::aarch64::vld1q_p64_x4(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_f32_x2(a: &[f32; 4]) -> float32x2x2_t {
  unsafe { core::arch::aarch64::vld1_f32_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_f32_x2(a: &[f32; 8]) -> float32x4x2_t {
  unsafe { core::arch::aarch64::vld1q_f32_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_f32_x3(a: &[f32; 6]) -> float32x2x3_t {
  unsafe { core::arch::aarch64::vld1_f32_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_f32_x3(a: &[f32; 12]) -> float32x4x3_t {
  unsafe { core::arch::aarch64::vld1q_f32_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_f32_x4(a: &[f32; 8]) -> float32x2x4_t {
  unsafe { core::arch::aarch64::vld1_f32_x4(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_f32_x4(a: &[f32; 16]) -> float32x4x4_t {
  unsafe { core::arch::aarch64::vld1q_f32_x4(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_s8(a: &[i8; 16]) -> int8x8x2_t {
  unsafe { core::arch::aarch64::vld2_s8(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_s16(a: &[i16; 8]) -> int16x4x2_t {
  unsafe { core::arch::aarch64::vld2_s16(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_s32(a: &[i32; 4]) -> int32x2x2_t {
  unsafe { core::arch::aarch64::vld2_s32(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_s8(a: &[i8; 32]) -> int8x16x2_t {
  unsafe { core::arch::aarch64::vld2q_s8(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_s16(a: &[i16; 16]) -> int16x8x2_t {
  unsafe { core::arch::aarch64::vld2q_s16(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_s32(a: &[i32; 8]) -> int32x4x2_t {
  unsafe { core::arch::aarch64::vld2q_s32(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_s64(a: &[i64; 2]) -> int64x1x2_t {
  unsafe { core::arch::aarch64::vld2_s64(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_u8(a: &[u8; 16]) -> uint8x8x2_t {
  unsafe { core::arch::aarch64::vld2_u8(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_u16(a: &[u16; 8]) -> uint16x4x2_t {
  unsafe { core::arch::aarch64::vld2_u16(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_u32(a: &[u32; 4]) -> uint32x2x2_t {
  unsafe { core::arch::aarch64::vld2_u32(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_u8(a: &[u8; 32]) -> uint8x16x2_t {
  unsafe { core::arch::aarch64::vld2q_u8(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_u16(a: &[u16; 16]) -> uint16x8x2_t {
  unsafe { core::arch::aarch64::vld2q_u16(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_u32(a: &[u32; 8]) -> uint32x4x2_t {
  unsafe { core::arch::aarch64::vld2q_u32(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_p8(a: &[p8; 16]) -> poly8x8x2_t {
  unsafe { core::arch::aarch64::vld2_p8(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_p16(a: &[p16; 8]) -> poly16x4x2_t {
  unsafe { core::arch::aarch64::vld2_p16(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_p8(a: &[p8; 32]) -> poly8x16x2_t {
  unsafe { core::arch::aarch64::vld2q_p8(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_p16(a: &[p16; 16]) -> poly16x8x2_t {
  unsafe { core::arch::aarch64::vld2q_p16(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_u64(a: &[u64; 2]) -> uint64x1x2_t {
  unsafe { core::arch::aarch64::vld2_u64(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld2_p64(a: &[p64; 2]) -> poly64x1x2_t {
  unsafe { core::arch::aarch64::vld2_p64(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_f32(a: &[f32; 4]) -> float32x2x2_t {
  unsafe { core::arch::aarch64::vld2_f32(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_f32(a: &[f32; 8]) -> float32x4x2_t {
  unsafe { core::arch::aarch64::vld2q_f32(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_dup_s8(a: &[i8; 2]) -> int8x8x2_t {
  unsafe { core::arch::aarch64::vld2_dup_s8(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_dup_s16(a: &[i16; 2]) -> int16x4x2_t {
  unsafe { core::arch::aarch64::vld2_dup_s16(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_dup_s32(a: &[i32; 2]) -> int32x2x2_t {
  unsafe { core::arch::aarch64::vld2_dup_s32(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_dup_s8(a: &[i8; 2]) -> int8x16x2_t {
  unsafe { core::arch::aarch64::vld2q_dup_s8(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_dup_s16(a: &[i16; 2]) -> int16x8x2_t {
  unsafe { core::arch::aarch64::vld2q_dup_s16(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_dup_s32(a: &[i32; 2]) -> int32x4x2_t {
  unsafe { core::arch::aarch64::vld2q_dup_s32(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_dup_s64(a: &[i64; 2]) -> int64x1x2_t {
  unsafe { core::arch::aarch64::vld2_dup_s64(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_dup_u8(a: &[u8; 2]) -> uint8x8x2_t {
  unsafe { core::arch::aarch64::vld2_dup_u8(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_dup_u16(a: &[u16; 2]) -> uint16x4x2_t {
  unsafe { core::arch::aarch64::vld2_dup_u16(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_dup_u32(a: &[u32; 2]) -> uint32x2x2_t {
  unsafe { core::arch::aarch64::vld2_dup_u32(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_dup_u8(a: &[u8; 2]) -> uint8x16x2_t {
  unsafe { core::arch::aarch64::vld2q_dup_u8(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_dup_u16(a: &[u16; 2]) -> uint16x8x2_t {
  unsafe { core::arch::aarch64::vld2q_dup_u16(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_dup_u32(a: &[u32; 2]) -> uint32x4x2_t {
  unsafe { core::arch::aarch64::vld2q_dup_u32(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_dup_p8(a: &[p8; 2]) -> poly8x8x2_t {
  unsafe { core::arch::aarch64::vld2_dup_p8(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_dup_p16(a: &[p16; 2]) -> poly16x4x2_t {
  unsafe { core::arch::aarch64::vld2_dup_p16(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_dup_p8(a: &[p8; 2]) -> poly8x16x2_t {
  unsafe { core::arch::aarch64::vld2q_dup_p8(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_dup_p16(a: &[p16; 2]) -> poly16x8x2_t {
  unsafe { core::arch::aarch64::vld2q_dup_p16(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_dup_u64(a: &[u64; 2]) -> uint64x1x2_t {
  unsafe { core::arch::aarch64::vld2_dup_u64(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld2_dup_p64(a: &[p64; 2]) -> poly64x1x2_t {
  unsafe { core::arch::aarch64::vld2_dup_p64(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_dup_f32(a: &[f32; 2]) -> float32x2x2_t {
  unsafe { core::arch::aarch64::vld2_dup_f32(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_dup_f32(a: &[f32; 2]) -> float32x4x2_t {
  unsafe { core::arch::aarch64::vld2q_dup_f32(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_lane_s8<const LANE: i32>(a: &[i8; 2], b: int8x8x2_t) -> int8x8x2_t {
  unsafe { core::arch::aarch64::vld2_lane_s8::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_lane_s16<const LANE: i32>(a: &[i16; 2], b: int16x4x2_t) -> int16x4x2_t {
  unsafe { core::arch::aarch64::vld2_lane_s16::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_lane_s32<const LANE: i32>(a: &[i32; 2], b: int32x2x2_t) -> int32x2x2_t {
  unsafe { core::arch::aarch64::vld2_lane_s32::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_lane_s16<const LANE: i32>(a: &[i16; 2], b: int16x8x2_t) -> int16x8x2_t {
  unsafe { core::arch::aarch64::vld2q_lane_s16::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_lane_s32<const LANE: i32>(a: &[i32; 2], b: int32x4x2_t) -> int32x4x2_t {
  unsafe { core::arch::aarch64::vld2q_lane_s32::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_lane_u8<const LANE: i32>(a: &[u8; 2], b: uint8x8x2_t) -> uint8x8x2_t {
  unsafe { core::arch::aarch64::vld2_lane_u8::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_lane_u16<const LANE: i32>(a: &[u16; 2], b: uint16x4x2_t) -> uint16x4x2_t {
  unsafe { core::arch::aarch64::vld2_lane_u16::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_lane_u32<const LANE: i32>(a: &[u32; 2], b: uint32x2x2_t) -> uint32x2x2_t {
  unsafe { core::arch::aarch64::vld2_lane_u32::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_lane_u16<const LANE: i32>(a: &[u16; 2], b: uint16x8x2_t) -> uint16x8x2_t {
  unsafe { core::arch::aarch64::vld2q_lane_u16::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_lane_u32<const LANE: i32>(a: &[u32; 2], b: uint32x4x2_t) -> uint32x4x2_t {
  unsafe { core::arch::aarch64::vld2q_lane_u32::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_lane_p8<const LANE: i32>(a: &[p8; 2], b: poly8x8x2_t) -> poly8x8x2_t {
  unsafe { core::arch::aarch64::vld2_lane_p8::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_lane_p16<const LANE: i32>(a: &[p16; 2], b: poly16x4x2_t) -> poly16x4x2_t {
  unsafe { core::arch::aarch64::vld2_lane_p16::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_lane_p16<const LANE: i32>(a: &[p16; 2], b: poly16x8x2_t) -> poly16x8x2_t {
  unsafe { core::arch::aarch64::vld2q_lane_p16::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_lane_f32<const LANE: i32>(a: &[f32; 2], b: float32x2x2_t) -> float32x2x2_t {
  unsafe { core::arch::aarch64::vld2_lane_f32::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_lane_f32<const LANE: i32>(a: &[f32; 2], b: float32x4x2_t) -> float32x4x2_t {
  unsafe { core::arch::aarch64::vld2q_lane_f32::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_s8(a: &[i8; 24]) -> int8x8x3_t {
  unsafe { core::arch::aarch64::vld3_s8(a.as_ptr()) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_s16(a: &[i16; 12]) -> int16x4x3_t {
  unsafe { core::arch::aarch64::vld3_s16(a.as_ptr()) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_s32(a: &[i32; 6]) -> int32x2x3_t {
  unsafe { core::arch::aarch64::vld3_s32(a.as_ptr()) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_s8(a: &[i8; 48]) -> int8x16x3_t {
  unsafe { core::arch::aarch64::vld3q_s8(a.as_ptr()) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_s16(a: &[i16; 24]) -> int16x8x3_t {
  unsafe { core::arch::aarch64::vld3q_s16(a.as_ptr()) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_s32(a: &[i32; 12]) -> int32x4x3_t {
  unsafe { core::arch::aarch64::vld3q_s32(a.as_ptr()) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_s64(a: &[i64; 3]) -> int64x1x3_t {
  unsafe { core::arch::aarch64::vld3_s64(a.as_ptr()) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_u8(a: &[u8; 24]) -> uint8x8x3_t {
  unsafe { core::arch::aarch64::vld3_u8(a.as_ptr()) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_u16(a: &[u16; 12]) -> uint16x4x3_t {
  unsafe { core::arch::aarch64::vld3_u16(a.as_ptr()) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_u32(a: &[u32; 6]) -> uint32x2x3_t {
  unsafe { core::arch::aarch64::vld3_u32(a.as_ptr()) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_u8(a: &[u8; 48]) -> uint8x16x3_t {
  unsafe { core::arch::aarch64::vld3q_u8(a.as_ptr()) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_u16(a: &[u16; 24]) -> uint16x8x3_t {
  unsafe { core::arch::aarch64::vld3q_u16(a.as_ptr()) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_u32(a: &[u32; 12]) -> uint32x4x3_t {
  unsafe { core::arch::aarch64::vld3q_u32(a.as_ptr()) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_p8(a: &[p8; 24]) -> poly8x8x3_t {
  unsafe { core::arch::aarch64::vld3_p8(a.as_ptr()) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_p16(a: &[p16; 12]) -> poly16x4x3_t {
  unsafe { core::arch::aarch64::vld3_p16(a.as_ptr()) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_p8(a: &[p8; 48]) -> poly8x16x3_t {
  unsafe { core::arch::aarch64::vld3q_p8(a.as_ptr()) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_p16(a: &[p16; 24]) -> poly16x8x3_t {
  unsafe { core::arch::aarch64::vld3q_p16(a.as_ptr()) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_u64(a: &[u64; 3]) -> uint64x1x3_t {
  unsafe { core::arch::aarch64::vld3_u64(a.as_ptr()) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld3_p64(a: &[p64; 3]) -> poly64x1x3_t {
  unsafe { core::arch::aarch64::vld3_p64(a.as_ptr()) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_f32(a: &[f32; 6]) -> float32x2x3_t {
  unsafe { core::arch::aarch64::vld3_f32(a.as_ptr()) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_f32(a: &[f32; 12]) -> float32x4x3_t {
  unsafe { core::arch::aarch64::vld3q_f32(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_dup_s8(a: &[i8; 3]) -> int8x8x3_t {
  unsafe { core::arch::aarch64::vld3_dup_s8(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_dup_s16(a: &[i16; 3]) -> int16x4x3_t {
  unsafe { core::arch::aarch64::vld3_dup_s16(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_dup_s32(a: &[i32; 3]) -> int32x2x3_t {
  unsafe { core::arch::aarch64::vld3_dup_s32(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_dup_s8(a: &[i8; 3]) -> int8x16x3_t {
  unsafe { core::arch::aarch64::vld3q_dup_s8(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_dup_s16(a: &[i16; 3]) -> int16x8x3_t {
  unsafe { core::arch::aarch64::vld3q_dup_s16(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_dup_s32(a: &[i32; 3]) -> int32x4x3_t {
  unsafe { core::arch::aarch64::vld3q_dup_s32(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_dup_s64(a: &[i64; 3]) -> int64x1x3_t {
  unsafe { core::arch::aarch64::vld3_dup_s64(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_dup_u8(a: &[u8; 3]) -> uint8x8x3_t {
  unsafe { core::arch::aarch64::vld3_dup_u8(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_dup_u16(a: &[u16; 3]) -> uint16x4x3_t {
  unsafe { core::arch::aarch64::vld3_dup_u16(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_dup_u32(a: &[u32; 3]) -> uint32x2x3_t {
  unsafe { core::arch::aarch64::vld3_dup_u32(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_dup_u8(a: &[u8; 3]) -> uint8x16x3_t {
  unsafe { core::arch::aarch64::vld3q_dup_u8(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_dup_u16(a: &[u16; 3]) -> uint16x8x3_t {
  unsafe { core::arch::aarch64::vld3q_dup_u16(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_dup_u32(a: &[u32; 3]) -> uint32x4x3_t {
  unsafe { core::arch::aarch64::vld3q_dup_u32(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_dup_p8(a: &[p8; 3]) -> poly8x8x3_t {
  unsafe { core::arch::aarch64::vld3_dup_p8(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_dup_p16(a: &[p16; 3]) -> poly16x4x3_t {
  unsafe { core::arch::aarch64::vld3_dup_p16(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_dup_p8(a: &[p8; 3]) -> poly8x16x3_t {
  unsafe { core::arch::aarch64::vld3q_dup_p8(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_dup_p16(a: &[p16; 3]) -> poly16x8x3_t {
  unsafe { core::arch::aarch64::vld3q_dup_p16(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_dup_u64(a: &[u64; 3]) -> uint64x1x3_t {
  unsafe { core::arch::aarch64::vld3_dup_u64(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld3_dup_p64(a: &[p64; 3]) -> poly64x1x3_t {
  unsafe { core::arch::aarch64::vld3_dup_p64(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_dup_f32(a: &[f32; 3]) -> float32x2x3_t {
  unsafe { core::arch::aarch64::vld3_dup_f32(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_dup_f32(a: &[f32; 3]) -> float32x4x3_t {
  unsafe { core::arch::aarch64::vld3q_dup_f32(a.as_ptr()) }
}

/// Load multiple 3-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_lane_s8<const LANE: i32>(a: &[i8; 3], b: int8x8x3_t) -> int8x8x3_t {
  unsafe { core::arch::aarch64::vld3_lane_s8::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_lane_s16<const LANE: i32>(a: &[i16; 3], b: int16x4x3_t) -> int16x4x3_t {
  unsafe { core::arch::aarch64::vld3_lane_s16::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_lane_s32<const LANE: i32>(a: &[i32; 3], b: int32x2x3_t) -> int32x2x3_t {
  unsafe { core::arch::aarch64::vld3_lane_s32::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_lane_s16<const LANE: i32>(a: &[i16; 3], b: int16x8x3_t) -> int16x8x3_t {
  unsafe { core::arch::aarch64::vld3q_lane_s16::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_lane_s32<const LANE: i32>(a: &[i32; 3], b: int32x4x3_t) -> int32x4x3_t {
  unsafe { core::arch::aarch64::vld3q_lane_s32::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_lane_u8<const LANE: i32>(a: &[u8; 3], b: uint8x8x3_t) -> uint8x8x3_t {
  unsafe { core::arch::aarch64::vld3_lane_u8::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_lane_u16<const LANE: i32>(a: &[u16; 3], b: uint16x4x3_t) -> uint16x4x3_t {
  unsafe { core::arch::aarch64::vld3_lane_u16::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_lane_u32<const LANE: i32>(a: &[u32; 3], b: uint32x2x3_t) -> uint32x2x3_t {
  unsafe { core::arch::aarch64::vld3_lane_u32::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_lane_u16<const LANE: i32>(a: &[u16; 3], b: uint16x8x3_t) -> uint16x8x3_t {
  unsafe { core::arch::aarch64::vld3q_lane_u16::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_lane_u32<const LANE: i32>(a: &[u32; 3], b: uint32x4x3_t) -> uint32x4x3_t {
  unsafe { core::arch::aarch64::vld3q_lane_u32::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_lane_p8<const LANE: i32>(a: &[p8; 3], b: poly8x8x3_t) -> poly8x8x3_t {
  unsafe { core::arch::aarch64::vld3_lane_p8::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_lane_p16<const LANE: i32>(a: &[p16; 3], b: poly16x4x3_t) -> poly16x4x3_t {
  unsafe { core::arch::aarch64::vld3_lane_p16::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_lane_p16<const LANE: i32>(a: &[p16; 3], b: poly16x8x3_t) -> poly16x8x3_t {
  unsafe { core::arch::aarch64::vld3q_lane_p16::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_lane_f32<const LANE: i32>(a: &[f32; 3], b: float32x2x3_t) -> float32x2x3_t {
  unsafe { core::arch::aarch64::vld3_lane_f32::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_lane_f32<const LANE: i32>(a: &[f32; 3], b: float32x4x3_t) -> float32x4x3_t {
  unsafe { core::arch::aarch64::vld3q_lane_f32::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_s8(a: &[i8; 32]) -> int8x8x4_t {
  unsafe { core::arch::aarch64::vld4_s8(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_s16(a: &[i16; 16]) -> int16x4x4_t {
  unsafe { core::arch::aarch64::vld4_s16(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_s32(a: &[i32; 8]) -> int32x2x4_t {
  unsafe { core::arch::aarch64::vld4_s32(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_s8(a: &[i8; 64]) -> int8x16x4_t {
  unsafe { core::arch::aarch64::vld4q_s8(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_s16(a: &[i16; 32]) -> int16x8x4_t {
  unsafe { core::arch::aarch64::vld4q_s16(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_s32(a: &[i32; 16]) -> int32x4x4_t {
  unsafe { core::arch::aarch64::vld4q_s32(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_s64(a: &[i64; 4]) -> int64x1x4_t {
  unsafe { core::arch::aarch64::vld4_s64(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_u8(a: &[u8; 32]) -> uint8x8x4_t {
  unsafe { core::arch::aarch64::vld4_u8(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_u16(a: &[u16; 16]) -> uint16x4x4_t {
  unsafe { core::arch::aarch64::vld4_u16(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_u32(a: &[u32; 8]) -> uint32x2x4_t {
  unsafe { core::arch::aarch64::vld4_u32(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_u8(a: &[u8; 64]) -> uint8x16x4_t {
  unsafe { core::arch::aarch64::vld4q_u8(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_u16(a: &[u16; 32]) -> uint16x8x4_t {
  unsafe { core::arch::aarch64::vld4q_u16(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_u32(a: &[u32; 16]) -> uint32x4x4_t {
  unsafe { core::arch::aarch64::vld4q_u32(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_p8(a: &[p8; 32]) -> poly8x8x4_t {
  unsafe { core::arch::aarch64::vld4_p8(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_p16(a: &[p16; 16]) -> poly16x4x4_t {
  unsafe { core::arch::aarch64::vld4_p16(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_p8(a: &[p8; 64]) -> poly8x16x4_t {
  unsafe { core::arch::aarch64::vld4q_p8(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_p16(a: &[p16; 32]) -> poly16x8x4_t {
  unsafe { core::arch::aarch64::vld4q_p16(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_u64(a: &[u64; 4]) -> uint64x1x4_t {
  unsafe { core::arch::aarch64::vld4_u64(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld4_p64(a: &[p64; 4]) -> poly64x1x4_t {
  unsafe { core::arch::aarch64::vld4_p64(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_f32(a: &[f32; 8]) -> float32x2x4_t {
  unsafe { core::arch::aarch64::vld4_f32(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_f32(a: &[f32; 16]) -> float32x4x4_t {
  unsafe { core::arch::aarch64::vld4q_f32(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_dup_s8(a: &[i8; 4]) -> int8x8x4_t {
  unsafe { core::arch::aarch64::vld4_dup_s8(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_dup_s16(a: &[i16; 4]) -> int16x4x4_t {
  unsafe { core::arch::aarch64::vld4_dup_s16(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_dup_s32(a: &[i32; 4]) -> int32x2x4_t {
  unsafe { core::arch::aarch64::vld4_dup_s32(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_dup_s8(a: &[i8; 4]) -> int8x16x4_t {
  unsafe { core::arch::aarch64::vld4q_dup_s8(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_dup_s16(a: &[i16; 4]) -> int16x8x4_t {
  unsafe { core::arch::aarch64::vld4q_dup_s16(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_dup_s32(a: &[i32; 4]) -> int32x4x4_t {
  unsafe { core::arch::aarch64::vld4q_dup_s32(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_dup_s64(a: &[i64; 4]) -> int64x1x4_t {
  unsafe { core::arch::aarch64::vld4_dup_s64(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_dup_u8(a: &[u8; 4]) -> uint8x8x4_t {
  unsafe { core::arch::aarch64::vld4_dup_u8(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_dup_u16(a: &[u16; 4]) -> uint16x4x4_t {
  unsafe { core::arch::aarch64::vld4_dup_u16(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_dup_u32(a: &[u32; 4]) -> uint32x2x4_t {
  unsafe { core::arch::aarch64::vld4_dup_u32(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_dup_u8(a: &[u8; 4]) -> uint8x16x4_t {
  unsafe { core::arch::aarch64::vld4q_dup_u8(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_dup_u16(a: &[u16; 4]) -> uint16x8x4_t {
  unsafe { core::arch::aarch64::vld4q_dup_u16(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_dup_u32(a: &[u32; 4]) -> uint32x4x4_t {
  unsafe { core::arch::aarch64::vld4q_dup_u32(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_dup_p8(a: &[p8; 4]) -> poly8x8x4_t {
  unsafe { core::arch::aarch64::vld4_dup_p8(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_dup_p16(a: &[p16; 4]) -> poly16x4x4_t {
  unsafe { core::arch::aarch64::vld4_dup_p16(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_dup_p8(a: &[p8; 4]) -> poly8x16x4_t {
  unsafe { core::arch::aarch64::vld4q_dup_p8(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_dup_p16(a: &[p16; 4]) -> poly16x8x4_t {
  unsafe { core::arch::aarch64::vld4q_dup_p16(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_dup_u64(a: &[u64; 4]) -> uint64x1x4_t {
  unsafe { core::arch::aarch64::vld4_dup_u64(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld4_dup_p64(a: &[p64; 4]) -> poly64x1x4_t {
  unsafe { core::arch::aarch64::vld4_dup_p64(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_dup_f32(a: &[f32; 4]) -> float32x2x4_t {
  unsafe { core::arch::aarch64::vld4_dup_f32(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_dup_f32(a: &[f32; 4]) -> float32x4x4_t {
  unsafe { core::arch::aarch64::vld4q_dup_f32(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_lane_s8<const LANE: i32>(a: &[i8; 4], b: int8x8x4_t) -> int8x8x4_t {
  unsafe { core::arch::aarch64::vld4_lane_s8::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_lane_s16<const LANE: i32>(a: &[i16; 4], b: int16x4x4_t) -> int16x4x4_t {
  unsafe { core::arch::aarch64::vld4_lane_s16::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_lane_s32<const LANE: i32>(a: &[i32; 4], b: int32x2x4_t) -> int32x2x4_t {
  unsafe { core::arch::aarch64::vld4_lane_s32::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_lane_s16<const LANE: i32>(a: &[i16; 4], b: int16x8x4_t) -> int16x8x4_t {
  unsafe { core::arch::aarch64::vld4q_lane_s16::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_lane_s32<const LANE: i32>(a: &[i32; 4], b: int32x4x4_t) -> int32x4x4_t {
  unsafe { core::arch::aarch64::vld4q_lane_s32::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_lane_u8<const LANE: i32>(a: &[u8; 4], b: uint8x8x4_t) -> uint8x8x4_t {
  unsafe { core::arch::aarch64::vld4_lane_u8::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_lane_u16<const LANE: i32>(a: &[u16; 4], b: uint16x4x4_t) -> uint16x4x4_t {
  unsafe { core::arch::aarch64::vld4_lane_u16::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_lane_u32<const LANE: i32>(a: &[u32; 4], b: uint32x2x4_t) -> uint32x2x4_t {
  unsafe { core::arch::aarch64::vld4_lane_u32::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_lane_u16<const LANE: i32>(a: &[u16; 4], b: uint16x8x4_t) -> uint16x8x4_t {
  unsafe { core::arch::aarch64::vld4q_lane_u16::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_lane_u32<const LANE: i32>(a: &[u32; 4], b: uint32x4x4_t) -> uint32x4x4_t {
  unsafe { core::arch::aarch64::vld4q_lane_u32::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_lane_p8<const LANE: i32>(a: &[p8; 4], b: poly8x8x4_t) -> poly8x8x4_t {
  unsafe { core::arch::aarch64::vld4_lane_p8::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_lane_p16<const LANE: i32>(a: &[p16; 4], b: poly16x4x4_t) -> poly16x4x4_t {
  unsafe { core::arch::aarch64::vld4_lane_p16::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_lane_p16<const LANE: i32>(a: &[p16; 4], b: poly16x8x4_t) -> poly16x8x4_t {
  unsafe { core::arch::aarch64::vld4q_lane_p16::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_lane_f32<const LANE: i32>(a: &[f32; 4], b: float32x2x4_t) -> float32x2x4_t {
  unsafe { core::arch::aarch64::vld4_lane_f32::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_lane_f32<const LANE: i32>(a: &[f32; 4], b: float32x4x4_t) -> float32x4x4_t {
  unsafe { core::arch::aarch64::vld4q_lane_f32::<LANE>(a.as_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_lane_s8<const LANE: i32>(a: &mut i8, b: int8x8_t) {
  unsafe { core::arch::aarch64::vst1_lane_s8::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_lane_s16<const LANE: i32>(a: &mut i16, b: int16x4_t) {
  unsafe { core::arch::aarch64::vst1_lane_s16::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_lane_s32<const LANE: i32>(a: &mut i32, b: int32x2_t) {
  unsafe { core::arch::aarch64::vst1_lane_s32::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_lane_s64<const LANE: i32>(a: &mut i64, b: int64x1_t) {
  unsafe { core::arch::aarch64::vst1_lane_s64::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_lane_s8<const LANE: i32>(a: &mut i8, b: int8x16_t) {
  unsafe { core::arch::aarch64::vst1q_lane_s8::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_lane_s16<const LANE: i32>(a: &mut i16, b: int16x8_t) {
  unsafe { core::arch::aarch64::vst1q_lane_s16::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_lane_s32<const LANE: i32>(a: &mut i32, b: int32x4_t) {
  unsafe { core::arch::aarch64::vst1q_lane_s32::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_lane_s64<const LANE: i32>(a: &mut i64, b: int64x2_t) {
  unsafe { core::arch::aarch64::vst1q_lane_s64::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_lane_u8<const LANE: i32>(a: &mut u8, b: uint8x8_t) {
  unsafe { core::arch::aarch64::vst1_lane_u8::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_lane_u16<const LANE: i32>(a: &mut u16, b: uint16x4_t) {
  unsafe { core::arch::aarch64::vst1_lane_u16::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_lane_u32<const LANE: i32>(a: &mut u32, b: uint32x2_t) {
  unsafe { core::arch::aarch64::vst1_lane_u32::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_lane_u64<const LANE: i32>(a: &mut u64, b: uint64x1_t) {
  unsafe { core::arch::aarch64::vst1_lane_u64::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_lane_u8<const LANE: i32>(a: &mut u8, b: uint8x16_t) {
  unsafe { core::arch::aarch64::vst1q_lane_u8::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_lane_u16<const LANE: i32>(a: &mut u16, b: uint16x8_t) {
  unsafe { core::arch::aarch64::vst1q_lane_u16::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_lane_u32<const LANE: i32>(a: &mut u32, b: uint32x4_t) {
  unsafe { core::arch::aarch64::vst1q_lane_u32::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_lane_u64<const LANE: i32>(a: &mut u64, b: uint64x2_t) {
  unsafe { core::arch::aarch64::vst1q_lane_u64::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_lane_p8<const LANE: i32>(a: &mut p8, b: poly8x8_t) {
  unsafe { core::arch::aarch64::vst1_lane_p8::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_lane_p16<const LANE: i32>(a: &mut p16, b: poly16x4_t) {
  unsafe { core::arch::aarch64::vst1_lane_p16::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_lane_p8<const LANE: i32>(a: &mut p8, b: poly8x16_t) {
  unsafe { core::arch::aarch64::vst1q_lane_p8::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_lane_p16<const LANE: i32>(a: &mut p16, b: poly16x8_t) {
  unsafe { core::arch::aarch64::vst1q_lane_p16::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vst1_lane_p64<const LANE: i32>(a: &mut p64, b: poly64x1_t) {
  unsafe { core::arch::aarch64::vst1_lane_p64::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vst1q_lane_p64<const LANE: i32>(a: &mut p64, b: poly64x2_t) {
  unsafe { core::arch::aarch64::vst1q_lane_p64::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_lane_f32<const LANE: i32>(a: &mut f32, b: float32x2_t) {
  unsafe { core::arch::aarch64::vst1_lane_f32::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_lane_f32<const LANE: i32>(a: &mut f32, b: float32x4_t) {
  unsafe { core::arch::aarch64::vst1q_lane_f32::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_s8_x2(a: &mut [i8; 16], b: int8x8x2_t) {
  unsafe { core::arch::aarch64::vst1_s8_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_s16_x2(a: &mut [i16; 8], b: int16x4x2_t) {
  unsafe { core::arch::aarch64::vst1_s16_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_s32_x2(a: &mut [i32; 4], b: int32x2x2_t) {
  unsafe { core::arch::aarch64::vst1_s32_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_s64_x2(a: &mut [i64; 2], b: int64x1x2_t) {
  unsafe { core::arch::aarch64::vst1_s64_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_s8_x2(a: &mut [i8; 32], b: int8x16x2_t) {
  unsafe { core::arch::aarch64::vst1q_s8_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_s16_x2(a: &mut [i16; 16], b: int16x8x2_t) {
  unsafe { core::arch::aarch64::vst1q_s16_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_s32_x2(a: &mut [i32; 8], b: int32x4x2_t) {
  unsafe { core::arch::aarch64::vst1q_s32_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_s64_x2(a: &mut [i64; 4], b: int64x2x2_t) {
  unsafe { core::arch::aarch64::vst1q_s64_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_s8_x3(a: &mut [i8; 24], b: int8x8x3_t) {
  unsafe { core::arch::aarch64::vst1_s8_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_s16_x3(a: &mut [i16; 12], b: int16x4x3_t) {
  unsafe { core::arch::aarch64::vst1_s16_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_s32_x3(a: &mut [i32; 6], b: int32x2x3_t) {
  unsafe { core::arch::aarch64::vst1_s32_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_s64_x3(a: &mut [i64; 3], b: int64x1x3_t) {
  unsafe { core::arch::aarch64::vst1_s64_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_s8_x3(a: &mut [i8; 48], b: int8x16x3_t) {
  unsafe { core::arch::aarch64::vst1q_s8_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_s16_x3(a: &mut [i16; 24], b: int16x8x3_t) {
  unsafe { core::arch::aarch64::vst1q_s16_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_s32_x3(a: &mut [i32; 12], b: int32x4x3_t) {
  unsafe { core::arch::aarch64::vst1q_s32_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_s64_x3(a: &mut [i64; 6], b: int64x2x3_t) {
  unsafe { core::arch::aarch64::vst1q_s64_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_s8_x4(a: &mut [i8; 32], b: int8x8x4_t) {
  unsafe { core::arch::aarch64::vst1_s8_x4(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_s16_x4(a: &mut [i16; 16], b: int16x4x4_t) {
  unsafe { core::arch::aarch64::vst1_s16_x4(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_s32_x4(a: &mut [i32; 8], b: int32x2x4_t) {
  unsafe { core::arch::aarch64::vst1_s32_x4(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_s64_x4(a: &mut [i64; 4], b: int64x1x4_t) {
  unsafe { core::arch::aarch64::vst1_s64_x4(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_s8_x4(a: &mut [i8; 64], b: int8x16x4_t) {
  unsafe { core::arch::aarch64::vst1q_s8_x4(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_s16_x4(a: &mut [i16; 32], b: int16x8x4_t) {
  unsafe { core::arch::aarch64::vst1q_s16_x4(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_s32_x4(a: &mut [i32; 16], b: int32x4x4_t) {
  unsafe { core::arch::aarch64::vst1q_s32_x4(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_s64_x4(a: &mut [i64; 8], b: int64x2x4_t) {
  unsafe { core::arch::aarch64::vst1q_s64_x4(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_u8_x2(a: &mut [u8; 16], b: uint8x8x2_t) {
  unsafe { core::arch::aarch64::vst1_u8_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_u16_x2(a: &mut [u16; 8], b: uint16x4x2_t) {
  unsafe { core::arch::aarch64::vst1_u16_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_u32_x2(a: &mut [u32; 4], b: uint32x2x2_t) {
  unsafe { core::arch::aarch64::vst1_u32_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_u64_x2(a: &mut [u64; 2], b: uint64x1x2_t) {
  unsafe { core::arch::aarch64::vst1_u64_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_u8_x2(a: &mut [u8; 32], b: uint8x16x2_t) {
  unsafe { core::arch::aarch64::vst1q_u8_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_u16_x2(a: &mut [u16; 16], b: uint16x8x2_t) {
  unsafe { core::arch::aarch64::vst1q_u16_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_u32_x2(a: &mut [u32; 8], b: uint32x4x2_t) {
  unsafe { core::arch::aarch64::vst1q_u32_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_u64_x2(a: &mut [u64; 4], b: uint64x2x2_t) {
  unsafe { core::arch::aarch64::vst1q_u64_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_u8_x3(a: &mut [u8; 24], b: uint8x8x3_t) {
  unsafe { core::arch::aarch64::vst1_u8_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_u16_x3(a: &mut [u16; 12], b: uint16x4x3_t) {
  unsafe { core::arch::aarch64::vst1_u16_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_u32_x3(a: &mut [u32; 6], b: uint32x2x3_t) {
  unsafe { core::arch::aarch64::vst1_u32_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_u64_x3(a: &mut [u64; 3], b: uint64x1x3_t) {
  unsafe { core::arch::aarch64::vst1_u64_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_u8_x3(a: &mut [u8; 48], b: uint8x16x3_t) {
  unsafe { core::arch::aarch64::vst1q_u8_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_u16_x3(a: &mut [u16; 24], b: uint16x8x3_t) {
  unsafe { core::arch::aarch64::vst1q_u16_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_u32_x3(a: &mut [u32; 12], b: uint32x4x3_t) {
  unsafe { core::arch::aarch64::vst1q_u32_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_u64_x3(a: &mut [u64; 6], b: uint64x2x3_t) {
  unsafe { core::arch::aarch64::vst1q_u64_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_u8_x4(a: &mut [u8; 32], b: uint8x8x4_t) {
  unsafe { core::arch::aarch64::vst1_u8_x4(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_u16_x4(a: &mut [u16; 16], b: uint16x4x4_t) {
  unsafe { core::arch::aarch64::vst1_u16_x4(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_u32_x4(a: &mut [u32; 8], b: uint32x2x4_t) {
  unsafe { core::arch::aarch64::vst1_u32_x4(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_u64_x4(a: &mut [u64; 4], b: uint64x1x4_t) {
  unsafe { core::arch::aarch64::vst1_u64_x4(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_u8_x4(a: &mut [u8; 64], b: uint8x16x4_t) {
  unsafe { core::arch::aarch64::vst1q_u8_x4(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_u16_x4(a: &mut [u16; 32], b: uint16x8x4_t) {
  unsafe { core::arch::aarch64::vst1q_u16_x4(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_u32_x4(a: &mut [u32; 16], b: uint32x4x4_t) {
  unsafe { core::arch::aarch64::vst1q_u32_x4(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_u64_x4(a: &mut [u64; 8], b: uint64x2x4_t) {
  unsafe { core::arch::aarch64::vst1q_u64_x4(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_p8_x2(a: &mut [p8; 16], b: poly8x8x2_t) {
  unsafe { core::arch::aarch64::vst1_p8_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_p8_x3(a: &mut [p8; 24], b: poly8x8x3_t) {
  unsafe { core::arch::aarch64::vst1_p8_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_p8_x4(a: &mut [p8; 32], b: poly8x8x4_t) {
  unsafe { core::arch::aarch64::vst1_p8_x4(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_p8_x2(a: &mut [p8; 32], b: poly8x16x2_t) {
  unsafe { core::arch::aarch64::vst1q_p8_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_p8_x3(a: &mut [p8; 48], b: poly8x16x3_t) {
  unsafe { core::arch::aarch64::vst1q_p8_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_p8_x4(a: &mut [p8; 64], b: poly8x16x4_t) {
  unsafe { core::arch::aarch64::vst1q_p8_x4(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_p16_x2(a: &mut [p16; 8], b: poly16x4x2_t) {
  unsafe { core::arch::aarch64::vst1_p16_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_p16_x3(a: &mut [p16; 12], b: poly16x4x3_t) {
  unsafe { core::arch::aarch64::vst1_p16_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_p16_x4(a: &mut [p16; 16], b: poly16x4x4_t) {
  unsafe { core::arch::aarch64::vst1_p16_x4(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_p16_x2(a: &mut [p16; 16], b: poly16x8x2_t) {
  unsafe { core::arch::aarch64::vst1q_p16_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_p16_x3(a: &mut [p16; 24], b: poly16x8x3_t) {
  unsafe { core::arch::aarch64::vst1q_p16_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_p16_x4(a: &mut [p16; 32], b: poly16x8x4_t) {
  unsafe { core::arch::aarch64::vst1q_p16_x4(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vst1_p64_x2(a: &mut [p64; 2], b: poly64x1x2_t) {
  unsafe { core::arch::aarch64::vst1_p64_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vst1_p64_x3(a: &mut [p64; 3], b: poly64x1x3_t) {
  unsafe { core::arch::aarch64::vst1_p64_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vst1_p64_x4(a: &mut [p64; 4], b: poly64x1x4_t) {
  unsafe { core::arch::aarch64::vst1_p64_x4(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vst1q_p64_x2(a: &mut [p64; 4], b: poly64x2x2_t) {
  unsafe { core::arch::aarch64::vst1q_p64_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vst1q_p64_x3(a: &mut [p64; 6], b: poly64x2x3_t) {
  unsafe { core::arch::aarch64::vst1q_p64_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vst1q_p64_x4(a: &mut [p64; 8], b: poly64x2x4_t) {
  unsafe { core::arch::aarch64::vst1q_p64_x4(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_f32_x2(a: &mut [f32; 4], b: float32x2x2_t) {
  unsafe { core::arch::aarch64::vst1_f32_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_f32_x2(a: &mut [f32; 8], b: float32x4x2_t) {
  unsafe { core::arch::aarch64::vst1q_f32_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_f32_x3(a: &mut [f32; 6], b: float32x2x3_t) {
  unsafe { core::arch::aarch64::vst1_f32_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_f32_x3(a: &mut [f32; 12], b: float32x4x3_t) {
  unsafe { core::arch::aarch64::vst1q_f32_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_f32_x4(a: &mut [f32; 8], b: float32x2x4_t) {
  unsafe { core::arch::aarch64::vst1_f32_x4(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_f32_x4(a: &mut [f32; 16], b: float32x4x4_t) {
  unsafe { core::arch::aarch64::vst1q_f32_x4(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2_s8(a: &mut [i8; 16], b: int8x8x2_t) {
  unsafe { core::arch::aarch64::vst2_s8(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2_s16(a: &mut [i16; 8], b: int16x4x2_t) {
  unsafe { core::arch::aarch64::vst2_s16(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2_s32(a: &mut [i32; 4], b: int32x2x2_t) {
  unsafe { core::arch::aarch64::vst2_s32(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2q_s8(a: &mut [i8; 32], b: int8x16x2_t) {
  unsafe { core::arch::aarch64::vst2q_s8(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2q_s16(a: &mut [i16; 16], b: int16x8x2_t) {
  unsafe { core::arch::aarch64::vst2q_s16(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2q_s32(a: &mut [i32; 8], b: int32x4x2_t) {
  unsafe { core::arch::aarch64::vst2q_s32(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2_s64(a: &mut [i64; 2], b: int64x1x2_t) {
  unsafe { core::arch::aarch64::vst2_s64(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2_u8(a: &mut [u8; 16], b: uint8x8x2_t) {
  unsafe { core::arch::aarch64::vst2_u8(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2_u16(a: &mut [u16; 8], b: uint16x4x2_t) {
  unsafe { core::arch::aarch64::vst2_u16(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2_u32(a: &mut [u32; 4], b: uint32x2x2_t) {
  unsafe { core::arch::aarch64::vst2_u32(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2q_u8(a: &mut [u8; 32], b: uint8x16x2_t) {
  unsafe { core::arch::aarch64::vst2q_u8(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2q_u16(a: &mut [u16; 16], b: uint16x8x2_t) {
  unsafe { core::arch::aarch64::vst2q_u16(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2q_u32(a: &mut [u32; 8], b: uint32x4x2_t) {
  unsafe { core::arch::aarch64::vst2q_u32(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2_p8(a: &mut [p8; 16], b: poly8x8x2_t) {
  unsafe { core::arch::aarch64::vst2_p8(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2_p16(a: &mut [p16; 8], b: poly16x4x2_t) {
  unsafe { core::arch::aarch64::vst2_p16(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2q_p8(a: &mut [p8; 32], b: poly8x16x2_t) {
  unsafe { core::arch::aarch64::vst2q_p8(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2q_p16(a: &mut [p16; 16], b: poly16x8x2_t) {
  unsafe { core::arch::aarch64::vst2q_p16(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2_u64(a: &mut [u64; 2], b: uint64x1x2_t) {
  unsafe { core::arch::aarch64::vst2_u64(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vst2_p64(a: &mut [p64; 2], b: poly64x1x2_t) {
  unsafe { core::arch::aarch64::vst2_p64(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2_f32(a: &mut [f32; 4], b: float32x2x2_t) {
  unsafe { core::arch::aarch64::vst2_f32(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2q_f32(a: &mut [f32; 8], b: float32x4x2_t) {
  unsafe { core::arch::aarch64::vst2q_f32(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2_lane_s8<const LANE: i32>(a: &mut [i8; 2], b: int8x8x2_t) {
  unsafe { core::arch::aarch64::vst2_lane_s8::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2_lane_s16<const LANE: i32>(a: &mut [i16; 2], b: int16x4x2_t) {
  unsafe { core::arch::aarch64::vst2_lane_s16::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2_lane_s32<const LANE: i32>(a: &mut [i32; 2], b: int32x2x2_t) {
  unsafe { core::arch::aarch64::vst2_lane_s32::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2q_lane_s16<const LANE: i32>(a: &mut [i16; 2], b: int16x8x2_t) {
  unsafe { core::arch::aarch64::vst2q_lane_s16::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2q_lane_s32<const LANE: i32>(a: &mut [i32; 2], b: int32x4x2_t) {
  unsafe { core::arch::aarch64::vst2q_lane_s32::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2_lane_u8<const LANE: i32>(a: &mut [u8; 2], b: uint8x8x2_t) {
  unsafe { core::arch::aarch64::vst2_lane_u8::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2_lane_u16<const LANE: i32>(a: &mut [u16; 2], b: uint16x4x2_t) {
  unsafe { core::arch::aarch64::vst2_lane_u16::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2_lane_u32<const LANE: i32>(a: &mut [u32; 2], b: uint32x2x2_t) {
  unsafe { core::arch::aarch64::vst2_lane_u32::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2q_lane_u16<const LANE: i32>(a: &mut [u16; 2], b: uint16x8x2_t) {
  unsafe { core::arch::aarch64::vst2q_lane_u16::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2q_lane_u32<const LANE: i32>(a: &mut [u32; 2], b: uint32x4x2_t) {
  unsafe { core::arch::aarch64::vst2q_lane_u32::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2_lane_p8<const LANE: i32>(a: &mut [p8; 2], b: poly8x8x2_t) {
  unsafe { core::arch::aarch64::vst2_lane_p8::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2_lane_p16<const LANE: i32>(a: &mut [p16; 2], b: poly16x4x2_t) {
  unsafe { core::arch::aarch64::vst2_lane_p16::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2q_lane_p16<const LANE: i32>(a: &mut [p16; 2], b: poly16x8x2_t) {
  unsafe { core::arch::aarch64::vst2q_lane_p16::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2_lane_f32<const LANE: i32>(a: &mut [f32; 2], b: float32x2x2_t) {
  unsafe { core::arch::aarch64::vst2_lane_f32::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2q_lane_f32<const LANE: i32>(a: &mut [f32; 2], b: float32x4x2_t) {
  unsafe { core::arch::aarch64::vst2q_lane_f32::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3_s8(a: &mut [i8; 24], b: int8x8x3_t) {
  unsafe { core::arch::aarch64::vst3_s8(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3_s16(a: &mut [i16; 12], b: int16x4x3_t) {
  unsafe { core::arch::aarch64::vst3_s16(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3_s32(a: &mut [i32; 6], b: int32x2x3_t) {
  unsafe { core::arch::aarch64::vst3_s32(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3q_s8(a: &mut [i8; 48], b: int8x16x3_t) {
  unsafe { core::arch::aarch64::vst3q_s8(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3q_s16(a: &mut [i16; 24], b: int16x8x3_t) {
  unsafe { core::arch::aarch64::vst3q_s16(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3q_s32(a: &mut [i32; 12], b: int32x4x3_t) {
  unsafe { core::arch::aarch64::vst3q_s32(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3_s64(a: &mut [i64; 3], b: int64x1x3_t) {
  unsafe { core::arch::aarch64::vst3_s64(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3_u8(a: &mut [u8; 24], b: uint8x8x3_t) {
  unsafe { core::arch::aarch64::vst3_u8(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3_u16(a: &mut [u16; 12], b: uint16x4x3_t) {
  unsafe { core::arch::aarch64::vst3_u16(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3_u32(a: &mut [u32; 6], b: uint32x2x3_t) {
  unsafe { core::arch::aarch64::vst3_u32(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3q_u8(a: &mut [u8; 48], b: uint8x16x3_t) {
  unsafe { core::arch::aarch64::vst3q_u8(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3q_u16(a: &mut [u16; 24], b: uint16x8x3_t) {
  unsafe { core::arch::aarch64::vst3q_u16(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3q_u32(a: &mut [u32; 12], b: uint32x4x3_t) {
  unsafe { core::arch::aarch64::vst3q_u32(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3_p8(a: &mut [p8; 24], b: poly8x8x3_t) {
  unsafe { core::arch::aarch64::vst3_p8(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3_p16(a: &mut [p16; 12], b: poly16x4x3_t) {
  unsafe { core::arch::aarch64::vst3_p16(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3q_p8(a: &mut [p8; 48], b: poly8x16x3_t) {
  unsafe { core::arch::aarch64::vst3q_p8(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3q_p16(a: &mut [p16; 24], b: poly16x8x3_t) {
  unsafe { core::arch::aarch64::vst3q_p16(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3_u64(a: &mut [u64; 3], b: uint64x1x3_t) {
  unsafe { core::arch::aarch64::vst3_u64(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vst3_p64(a: &mut [p64; 3], b: poly64x1x3_t) {
  unsafe { core::arch::aarch64::vst3_p64(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3_f32(a: &mut [f32; 6], b: float32x2x3_t) {
  unsafe { core::arch::aarch64::vst3_f32(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3q_f32(a: &mut [f32; 12], b: float32x4x3_t) {
  unsafe { core::arch::aarch64::vst3q_f32(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3_lane_s8<const LANE: i32>(a: &mut [i8; 3], b: int8x8x3_t) {
  unsafe { core::arch::aarch64::vst3_lane_s8::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3_lane_s16<const LANE: i32>(a: &mut [i16; 3], b: int16x4x3_t) {
  unsafe { core::arch::aarch64::vst3_lane_s16::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3_lane_s32<const LANE: i32>(a: &mut [i32; 3], b: int32x2x3_t) {
  unsafe { core::arch::aarch64::vst3_lane_s32::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3q_lane_s16<const LANE: i32>(a: &mut [i16; 3], b: int16x8x3_t) {
  unsafe { core::arch::aarch64::vst3q_lane_s16::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3q_lane_s32<const LANE: i32>(a: &mut [i32; 3], b: int32x4x3_t) {
  unsafe { core::arch::aarch64::vst3q_lane_s32::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3_lane_u8<const LANE: i32>(a: &mut [u8; 3], b: uint8x8x3_t) {
  unsafe { core::arch::aarch64::vst3_lane_u8::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3_lane_u16<const LANE: i32>(a: &mut [u16; 3], b: uint16x4x3_t) {
  unsafe { core::arch::aarch64::vst3_lane_u16::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3_lane_u32<const LANE: i32>(a: &mut [u32; 3], b: uint32x2x3_t) {
  unsafe { core::arch::aarch64::vst3_lane_u32::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3q_lane_u16<const LANE: i32>(a: &mut [u16; 3], b: uint16x8x3_t) {
  unsafe { core::arch::aarch64::vst3q_lane_u16::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3q_lane_u32<const LANE: i32>(a: &mut [u32; 3], b: uint32x4x3_t) {
  unsafe { core::arch::aarch64::vst3q_lane_u32::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3_lane_p8<const LANE: i32>(a: &mut [p8; 3], b: poly8x8x3_t) {
  unsafe { core::arch::aarch64::vst3_lane_p8::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3_lane_p16<const LANE: i32>(a: &mut [p16; 3], b: poly16x4x3_t) {
  unsafe { core::arch::aarch64::vst3_lane_p16::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3q_lane_p16<const LANE: i32>(a: &mut [p16; 3], b: poly16x8x3_t) {
  unsafe { core::arch::aarch64::vst3q_lane_p16::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3_lane_f32<const LANE: i32>(a: &mut [f32; 3], b: float32x2x3_t) {
  unsafe { core::arch::aarch64::vst3_lane_f32::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3q_lane_f32<const LANE: i32>(a: &mut [f32; 3], b: float32x4x3_t) {
  unsafe { core::arch::aarch64::vst3q_lane_f32::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4_s8(a: &mut [i8; 32], b: int8x8x4_t) {
  unsafe { core::arch::aarch64::vst4_s8(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4_s16(a: &mut [i16; 16], b: int16x4x4_t) {
  unsafe { core::arch::aarch64::vst4_s16(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4_s32(a: &mut [i32; 8], b: int32x2x4_t) {
  unsafe { core::arch::aarch64::vst4_s32(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4q_s8(a: &mut [i8; 64], b: int8x16x4_t) {
  unsafe { core::arch::aarch64::vst4q_s8(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4q_s16(a: &mut [i16; 32], b: int16x8x4_t) {
  unsafe { core::arch::aarch64::vst4q_s16(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4q_s32(a: &mut [i32; 16], b: int32x4x4_t) {
  unsafe { core::arch::aarch64::vst4q_s32(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4_s64(a: &mut [i64; 4], b: int64x1x4_t) {
  unsafe { core::arch::aarch64::vst4_s64(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4_u8(a: &mut [u8; 32], b: uint8x8x4_t) {
  unsafe { core::arch::aarch64::vst4_u8(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4_u16(a: &mut [u16; 16], b: uint16x4x4_t) {
  unsafe { core::arch::aarch64::vst4_u16(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4_u32(a: &mut [u32; 8], b: uint32x2x4_t) {
  unsafe { core::arch::aarch64::vst4_u32(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4q_u8(a: &mut [u8; 64], b: uint8x16x4_t) {
  unsafe { core::arch::aarch64::vst4q_u8(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4q_u16(a: &mut [u16; 32], b: uint16x8x4_t) {
  unsafe { core::arch::aarch64::vst4q_u16(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4q_u32(a: &mut [u32; 16], b: uint32x4x4_t) {
  unsafe { core::arch::aarch64::vst4q_u32(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4_p8(a: &mut [p8; 32], b: poly8x8x4_t) {
  unsafe { core::arch::aarch64::vst4_p8(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4_p16(a: &mut [p16; 16], b: poly16x4x4_t) {
  unsafe { core::arch::aarch64::vst4_p16(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4q_p8(a: &mut [p8; 64], b: poly8x16x4_t) {
  unsafe { core::arch::aarch64::vst4q_p8(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4q_p16(a: &mut [p16; 32], b: poly16x8x4_t) {
  unsafe { core::arch::aarch64::vst4q_p16(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4_u64(a: &mut [u64; 4], b: uint64x1x4_t) {
  unsafe { core::arch::aarch64::vst4_u64(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vst4_p64(a: &mut [p64; 4], b: poly64x1x4_t) {
  unsafe { core::arch::aarch64::vst4_p64(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4_f32(a: &mut [f32; 8], b: float32x2x4_t) {
  unsafe { core::arch::aarch64::vst4_f32(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4q_f32(a: &mut [f32; 16], b: float32x4x4_t) {
  unsafe { core::arch::aarch64::vst4q_f32(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4_lane_s8<const LANE: i32>(a: &mut [i8; 4], b: int8x8x4_t) {
  unsafe { core::arch::aarch64::vst4_lane_s8::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4_lane_s16<const LANE: i32>(a: &mut [i16; 4], b: int16x4x4_t) {
  unsafe { core::arch::aarch64::vst4_lane_s16::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4_lane_s32<const LANE: i32>(a: &mut [i32; 4], b: int32x2x4_t) {
  unsafe { core::arch::aarch64::vst4_lane_s32::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4q_lane_s16<const LANE: i32>(a: &mut [i16; 4], b: int16x8x4_t) {
  unsafe { core::arch::aarch64::vst4q_lane_s16::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4q_lane_s32<const LANE: i32>(a: &mut [i32; 4], b: int32x4x4_t) {
  unsafe { core::arch::aarch64::vst4q_lane_s32::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4_lane_u8<const LANE: i32>(a: &mut [u8; 4], b: uint8x8x4_t) {
  unsafe { core::arch::aarch64::vst4_lane_u8::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4_lane_u16<const LANE: i32>(a: &mut [u16; 4], b: uint16x4x4_t) {
  unsafe { core::arch::aarch64::vst4_lane_u16::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4_lane_u32<const LANE: i32>(a: &mut [u32; 4], b: uint32x2x4_t) {
  unsafe { core::arch::aarch64::vst4_lane_u32::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4q_lane_u16<const LANE: i32>(a: &mut [u16; 4], b: uint16x8x4_t) {
  unsafe { core::arch::aarch64::vst4q_lane_u16::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4q_lane_u32<const LANE: i32>(a: &mut [u32; 4], b: uint32x4x4_t) {
  unsafe { core::arch::aarch64::vst4q_lane_u32::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4_lane_p8<const LANE: i32>(a: &mut [p8; 4], b: poly8x8x4_t) {
  unsafe { core::arch::aarch64::vst4_lane_p8::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4_lane_p16<const LANE: i32>(a: &mut [p16; 4], b: poly16x4x4_t) {
  unsafe { core::arch::aarch64::vst4_lane_p16::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4q_lane_p16<const LANE: i32>(a: &mut [p16; 4], b: poly16x8x4_t) {
  unsafe { core::arch::aarch64::vst4q_lane_p16::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4_lane_f32<const LANE: i32>(a: &mut [f32; 4], b: float32x2x4_t) {
  unsafe { core::arch::aarch64::vst4_lane_f32::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4q_lane_f32<const LANE: i32>(a: &mut [f32; 4], b: float32x4x4_t) {
  unsafe { core::arch::aarch64::vst4q_lane_f32::<LANE>(a.as_mut_ptr(), b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vmul_s8(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vmulq_s8(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vmul_s16(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vmulq_s16(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vmul_s32(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmulq_s32(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vmul_u8(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vmulq_u8(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vmul_u16(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vmulq_u16(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vmul_u32(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmulq_u32(a, b) }
}

/// Polynomial multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_p8(a: poly8x8_t, b: poly8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vmul_p8(a, b) }
}

/// Polynomial multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_p8(a: poly8x16_t, b: poly8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vmulq_p8(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vmul_f32(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vmulq_f32(a, b) }
}

/// Vector multiply by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_n_s16(a: int16x4_t, b: i16) -> int16x4_t {
  unsafe { core::arch::aarch64::vmul_n_s16(a, b) }
}

/// Vector multiply by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_n_s16(a: int16x8_t, b: i16) -> int16x8_t {
  unsafe { core::arch::aarch64::vmulq_n_s16(a, b) }
}

/// Vector multiply by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_n_s32(a: int32x2_t, b: i32) -> int32x2_t {
  unsafe { core::arch::aarch64::vmul_n_s32(a, b) }
}

/// Vector multiply by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_n_s32(a: int32x4_t, b: i32) -> int32x4_t {
  unsafe { core::arch::aarch64::vmulq_n_s32(a, b) }
}

/// Vector multiply by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_n_u16(a: uint16x4_t, b: u16) -> uint16x4_t {
  unsafe { core::arch::aarch64::vmul_n_u16(a, b) }
}

/// Vector multiply by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_n_u16(a: uint16x8_t, b: u16) -> uint16x8_t {
  unsafe { core::arch::aarch64::vmulq_n_u16(a, b) }
}

/// Vector multiply by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_n_u32(a: uint32x2_t, b: u32) -> uint32x2_t {
  unsafe { core::arch::aarch64::vmul_n_u32(a, b) }
}

/// Vector multiply by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_n_u32(a: uint32x4_t, b: u32) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmulq_n_u32(a, b) }
}

/// Vector multiply by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_n_f32(a: float32x2_t, b: f32) -> float32x2_t {
  unsafe { core::arch::aarch64::vmul_n_f32(a, b) }
}

/// Vector multiply by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_n_f32(a: float32x4_t, b: f32) -> float32x4_t {
  unsafe { core::arch::aarch64::vmulq_n_f32(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_lane_s16<const LANE: i32>(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vmul_lane_s16::<LANE>(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_laneq_s16<const LANE: i32>(a: int16x4_t, b: int16x8_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vmul_laneq_s16::<LANE>(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_lane_s16<const LANE: i32>(a: int16x8_t, b: int16x4_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vmulq_lane_s16::<LANE>(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_laneq_s16<const LANE: i32>(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vmulq_laneq_s16::<LANE>(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_lane_s32<const LANE: i32>(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vmul_lane_s32::<LANE>(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_laneq_s32<const LANE: i32>(a: int32x2_t, b: int32x4_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vmul_laneq_s32::<LANE>(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_lane_s32<const LANE: i32>(a: int32x4_t, b: int32x2_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmulq_lane_s32::<LANE>(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_laneq_s32<const LANE: i32>(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmulq_laneq_s32::<LANE>(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_lane_u16<const LANE: i32>(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vmul_lane_u16::<LANE>(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_laneq_u16<const LANE: i32>(a: uint16x4_t, b: uint16x8_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vmul_laneq_u16::<LANE>(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_lane_u16<const LANE: i32>(a: uint16x8_t, b: uint16x4_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vmulq_lane_u16::<LANE>(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_laneq_u16<const LANE: i32>(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vmulq_laneq_u16::<LANE>(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_lane_u32<const LANE: i32>(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vmul_lane_u32::<LANE>(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_laneq_u32<const LANE: i32>(a: uint32x2_t, b: uint32x4_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vmul_laneq_u32::<LANE>(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_lane_u32<const LANE: i32>(a: uint32x4_t, b: uint32x2_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmulq_lane_u32::<LANE>(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_laneq_u32<const LANE: i32>(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmulq_laneq_u32::<LANE>(a, b) }
}

/// Floating-point multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_lane_f32<const LANE: i32>(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vmul_lane_f32::<LANE>(a, b) }
}

/// Floating-point multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_laneq_f32<const LANE: i32>(a: float32x2_t, b: float32x4_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vmul_laneq_f32::<LANE>(a, b) }
}

/// Floating-point multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_lane_f32<const LANE: i32>(a: float32x4_t, b: float32x2_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vmulq_lane_f32::<LANE>(a, b) }
}

/// Floating-point multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_laneq_f32<const LANE: i32>(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vmulq_laneq_f32::<LANE>(a, b) }
}

/// Signed multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_s8(a: int8x8_t, b: int8x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vmull_s8(a, b) }
}

/// Signed multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_s16(a: int16x4_t, b: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmull_s16(a, b) }
}

/// Signed multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_s32(a: int32x2_t, b: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vmull_s32(a, b) }
}

/// Unsigned multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_u8(a: uint8x8_t, b: uint8x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vmull_u8(a, b) }
}

/// Unsigned multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_u16(a: uint16x4_t, b: uint16x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmull_u16(a, b) }
}

/// Unsigned multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_u32(a: uint32x2_t, b: uint32x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmull_u32(a, b) }
}

/// Polynomial multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_p8(a: poly8x8_t, b: poly8x8_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vmull_p8(a, b) }
}

/// Vector long multiply with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_n_s16(a: int16x4_t, b: i16) -> int32x4_t {
  unsafe { core::arch::aarch64::vmull_n_s16(a, b) }
}

/// Vector long multiply with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_n_s32(a: int32x2_t, b: i32) -> int64x2_t {
  unsafe { core::arch::aarch64::vmull_n_s32(a, b) }
}

/// Vector long multiply with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_n_u16(a: uint16x4_t, b: u16) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmull_n_u16(a, b) }
}

/// Vector long multiply with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_n_u32(a: uint32x2_t, b: u32) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmull_n_u32(a, b) }
}

/// Vector long multiply by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_lane_s16<const LANE: i32>(a: int16x4_t, b: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmull_lane_s16::<LANE>(a, b) }
}

/// Vector long multiply by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_laneq_s16<const LANE: i32>(a: int16x4_t, b: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmull_laneq_s16::<LANE>(a, b) }
}

/// Vector long multiply by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_lane_s32<const LANE: i32>(a: int32x2_t, b: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vmull_lane_s32::<LANE>(a, b) }
}

/// Vector long multiply by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_laneq_s32<const LANE: i32>(a: int32x2_t, b: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vmull_laneq_s32::<LANE>(a, b) }
}

/// Vector long multiply by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_lane_u16<const LANE: i32>(a: uint16x4_t, b: uint16x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmull_lane_u16::<LANE>(a, b) }
}

/// Vector long multiply by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_laneq_u16<const LANE: i32>(a: uint16x4_t, b: uint16x8_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmull_laneq_u16::<LANE>(a, b) }
}

/// Vector long multiply by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_lane_u32<const LANE: i32>(a: uint32x2_t, b: uint32x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmull_lane_u32::<LANE>(a, b) }
}

/// Vector long multiply by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_laneq_u32<const LANE: i32>(a: uint32x2_t, b: uint32x4_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmull_laneq_u32::<LANE>(a, b) }
}

/// Floating-point fused Multiply-Add to accumulator(vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfma_f32(a: float32x2_t, b: float32x2_t, c: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vfma_f32(a, b, c) }
}

/// Floating-point fused Multiply-Add to accumulator(vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfmaq_f32(a: float32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vfmaq_f32(a, b, c) }
}

/// Floating-point fused Multiply-Add to accumulator(vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfma_n_f32(a: float32x2_t, b: float32x2_t, c: f32) -> float32x2_t {
  unsafe { core::arch::aarch64::vfma_n_f32(a, b, c) }
}

/// Floating-point fused Multiply-Add to accumulator(vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfmaq_n_f32(a: float32x4_t, b: float32x4_t, c: f32) -> float32x4_t {
  unsafe { core::arch::aarch64::vfmaq_n_f32(a, b, c) }
}

/// Floating-point fused multiply-subtract from accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfms_f32(a: float32x2_t, b: float32x2_t, c: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vfms_f32(a, b, c) }
}

/// Floating-point fused multiply-subtract from accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfmsq_f32(a: float32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vfmsq_f32(a, b, c) }
}

/// Floating-point fused Multiply-subtract to accumulator(vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfms_n_f32(a: float32x2_t, b: float32x2_t, c: f32) -> float32x2_t {
  unsafe { core::arch::aarch64::vfms_n_f32(a, b, c) }
}

/// Floating-point fused Multiply-subtract to accumulator(vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfmsq_n_f32(a: float32x4_t, b: float32x4_t, c: f32) -> float32x4_t {
  unsafe { core::arch::aarch64::vfmsq_n_f32(a, b, c) }
}

/// Subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsub_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vsub_s8(a, b) }
}

/// Subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vsubq_s8(a, b) }
}

/// Subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsub_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vsub_s16(a, b) }
}

/// Subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vsubq_s16(a, b) }
}

/// Subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsub_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vsub_s32(a, b) }
}

/// Subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vsubq_s32(a, b) }
}

/// Subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsub_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vsub_u8(a, b) }
}

/// Subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vsubq_u8(a, b) }
}

/// Subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsub_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vsub_u16(a, b) }
}

/// Subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vsubq_u16(a, b) }
}

/// Subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsub_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vsub_u32(a, b) }
}

/// Subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vsubq_u32(a, b) }
}

/// Subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsub_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vsub_s64(a, b) }
}

/// Subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vsubq_s64(a, b) }
}

/// Subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsub_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vsub_u64(a, b) }
}

/// Subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vsubq_u64(a, b) }
}

/// Subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsub_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vsub_f32(a, b) }
}

/// Subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vsubq_f32(a, b) }
}

/// Bitwise exclusive OR
#[inline]
#[cfg(target_feature = "neon")]
pub fn vadd_p8(a: poly8x8_t, b: poly8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vadd_p8(a, b) }
}

/// Bitwise exclusive OR
#[inline]
#[cfg(target_feature = "neon")]
pub fn vadd_p16(a: poly16x4_t, b: poly16x4_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vadd_p16(a, b) }
}

/// Bitwise exclusive OR
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddq_p8(a: poly8x16_t, b: poly8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vaddq_p8(a, b) }
}

/// Bitwise exclusive OR
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddq_p16(a: poly16x8_t, b: poly16x8_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vaddq_p16(a, b) }
}

/// Bitwise exclusive OR
#[inline]
#[cfg(target_feature = "neon")]
pub fn vadd_p64(a: poly64x1_t, b: poly64x1_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vadd_p64(a, b) }
}

/// Bitwise exclusive OR
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddq_p64(a: poly64x2_t, b: poly64x2_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vaddq_p64(a, b) }
}

/// Bitwise exclusive OR
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddq_p128(a: p128, b: p128) -> p128 {
  unsafe { core::arch::aarch64::vaddq_p128(a, b) }
}

/// Subtract returning high narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubhn_s16(a: int16x8_t, b: int16x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vsubhn_s16(a, b) }
}

/// Subtract returning high narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubhn_s32(a: int32x4_t, b: int32x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vsubhn_s32(a, b) }
}

/// Subtract returning high narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubhn_s64(a: int64x2_t, b: int64x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vsubhn_s64(a, b) }
}

/// Subtract returning high narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubhn_u16(a: uint16x8_t, b: uint16x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vsubhn_u16(a, b) }
}

/// Subtract returning high narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubhn_u32(a: uint32x4_t, b: uint32x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vsubhn_u32(a, b) }
}

/// Subtract returning high narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubhn_u64(a: uint64x2_t, b: uint64x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vsubhn_u64(a, b) }
}

/// Subtract returning high narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubhn_high_s16(a: int8x8_t, b: int16x8_t, c: int16x8_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vsubhn_high_s16(a, b, c) }
}

/// Subtract returning high narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubhn_high_s32(a: int16x4_t, b: int32x4_t, c: int32x4_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vsubhn_high_s32(a, b, c) }
}

/// Subtract returning high narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubhn_high_s64(a: int32x2_t, b: int64x2_t, c: int64x2_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vsubhn_high_s64(a, b, c) }
}

/// Subtract returning high narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubhn_high_u16(a: uint8x8_t, b: uint16x8_t, c: uint16x8_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vsubhn_high_u16(a, b, c) }
}

/// Subtract returning high narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubhn_high_u32(a: uint16x4_t, b: uint32x4_t, c: uint32x4_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vsubhn_high_u32(a, b, c) }
}

/// Subtract returning high narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubhn_high_u64(a: uint32x2_t, b: uint64x2_t, c: uint64x2_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vsubhn_high_u64(a, b, c) }
}

/// Signed halving subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vhsub_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vhsub_u8(a, b) }
}

/// Signed halving subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vhsubq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vhsubq_u8(a, b) }
}

/// Signed halving subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vhsub_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vhsub_u16(a, b) }
}

/// Signed halving subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vhsubq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vhsubq_u16(a, b) }
}

/// Signed halving subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vhsub_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vhsub_u32(a, b) }
}

/// Signed halving subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vhsubq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vhsubq_u32(a, b) }
}

/// Signed halving subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vhsub_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vhsub_s8(a, b) }
}

/// Signed halving subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vhsubq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vhsubq_s8(a, b) }
}

/// Signed halving subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vhsub_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vhsub_s16(a, b) }
}

/// Signed halving subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vhsubq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vhsubq_s16(a, b) }
}

/// Signed halving subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vhsub_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vhsub_s32(a, b) }
}

/// Signed halving subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vhsubq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vhsubq_s32(a, b) }
}

/// Signed Subtract Wide
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubw_s8(a: int16x8_t, b: int8x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vsubw_s8(a, b) }
}

/// Signed Subtract Wide
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubw_s16(a: int32x4_t, b: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vsubw_s16(a, b) }
}

/// Signed Subtract Wide
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubw_s32(a: int64x2_t, b: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vsubw_s32(a, b) }
}

/// Unsigned Subtract Wide
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubw_u8(a: uint16x8_t, b: uint8x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vsubw_u8(a, b) }
}

/// Unsigned Subtract Wide
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubw_u16(a: uint32x4_t, b: uint16x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vsubw_u16(a, b) }
}

/// Unsigned Subtract Wide
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubw_u32(a: uint64x2_t, b: uint32x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vsubw_u32(a, b) }
}

/// Signed Subtract Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubl_s8(a: int8x8_t, b: int8x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vsubl_s8(a, b) }
}

/// Signed Subtract Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubl_s16(a: int16x4_t, b: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vsubl_s16(a, b) }
}

/// Signed Subtract Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubl_s32(a: int32x2_t, b: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vsubl_s32(a, b) }
}

/// Unsigned Subtract Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubl_u8(a: uint8x8_t, b: uint8x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vsubl_u8(a, b) }
}

/// Unsigned Subtract Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubl_u16(a: uint16x4_t, b: uint16x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vsubl_u16(a, b) }
}

/// Unsigned Subtract Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubl_u32(a: uint32x2_t, b: uint32x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vsubl_u32(a, b) }
}

/// Maximum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmax_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vmax_s8(a, b) }
}

/// Maximum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vmaxq_s8(a, b) }
}

/// Maximum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmax_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vmax_s16(a, b) }
}

/// Maximum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vmaxq_s16(a, b) }
}

/// Maximum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmax_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vmax_s32(a, b) }
}

/// Maximum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmaxq_s32(a, b) }
}

/// Maximum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmax_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vmax_u8(a, b) }
}

/// Maximum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vmaxq_u8(a, b) }
}

/// Maximum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmax_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vmax_u16(a, b) }
}

/// Maximum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vmaxq_u16(a, b) }
}

/// Maximum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmax_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vmax_u32(a, b) }
}

/// Maximum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmaxq_u32(a, b) }
}

/// Maximum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmax_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vmax_f32(a, b) }
}

/// Maximum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vmaxq_f32(a, b) }
}

/// Floating-point Maximum Number (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxnm_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vmaxnm_f32(a, b) }
}

/// Floating-point Maximum Number (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxnmq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vmaxnmq_f32(a, b) }
}

/// Minimum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmin_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vmin_s8(a, b) }
}

/// Minimum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vminq_s8(a, b) }
}

/// Minimum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmin_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vmin_s16(a, b) }
}

/// Minimum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vminq_s16(a, b) }
}

/// Minimum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmin_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vmin_s32(a, b) }
}

/// Minimum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vminq_s32(a, b) }
}

/// Minimum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmin_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vmin_u8(a, b) }
}

/// Minimum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vminq_u8(a, b) }
}

/// Minimum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmin_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vmin_u16(a, b) }
}

/// Minimum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vminq_u16(a, b) }
}

/// Minimum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmin_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vmin_u32(a, b) }
}

/// Minimum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vminq_u32(a, b) }
}

/// Minimum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmin_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vmin_f32(a, b) }
}

/// Minimum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vminq_f32(a, b) }
}

/// Floating-point Minimum Number (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminnm_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vminnm_f32(a, b) }
}

/// Floating-point Minimum Number (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminnmq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vminnmq_f32(a, b) }
}

/// Floating-point add pairwise
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpadd_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vpadd_f32(a, b) }
}

/// Signed saturating doubling multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmull_s16(a: int16x4_t, b: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmull_s16(a, b) }
}

/// Signed saturating doubling multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmull_s32(a: int32x2_t, b: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqdmull_s32(a, b) }
}

/// Vector saturating doubling long multiply with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmull_n_s16(a: int16x4_t, b: i16) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmull_n_s16(a, b) }
}

/// Vector saturating doubling long multiply with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmull_n_s32(a: int32x2_t, b: i32) -> int64x2_t {
  unsafe { core::arch::aarch64::vqdmull_n_s32(a, b) }
}

/// Vector saturating doubling long multiply by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmull_lane_s16<const N: i32>(a: int16x4_t, b: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmull_lane_s16::<N>(a, b) }
}

/// Vector saturating doubling long multiply by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmull_lane_s32<const N: i32>(a: int32x2_t, b: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqdmull_lane_s32::<N>(a, b) }
}

/// Signed saturating doubling multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlal_s16(a: int32x4_t, b: int16x4_t, c: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmlal_s16(a, b, c) }
}

/// Signed saturating doubling multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlal_s32(a: int64x2_t, b: int32x2_t, c: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqdmlal_s32(a, b, c) }
}

/// Vector widening saturating doubling multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlal_n_s16(a: int32x4_t, b: int16x4_t, c: i16) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmlal_n_s16(a, b, c) }
}

/// Vector widening saturating doubling multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlal_n_s32(a: int64x2_t, b: int32x2_t, c: i32) -> int64x2_t {
  unsafe { core::arch::aarch64::vqdmlal_n_s32(a, b, c) }
}

/// Vector widening saturating doubling multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlal_lane_s16<const N: i32>(a: int32x4_t, b: int16x4_t, c: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmlal_lane_s16::<N>(a, b, c) }
}

/// Vector widening saturating doubling multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlal_lane_s32<const N: i32>(a: int64x2_t, b: int32x2_t, c: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqdmlal_lane_s32::<N>(a, b, c) }
}

/// Signed saturating doubling multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlsl_s16(a: int32x4_t, b: int16x4_t, c: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmlsl_s16(a, b, c) }
}

/// Signed saturating doubling multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlsl_s32(a: int64x2_t, b: int32x2_t, c: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqdmlsl_s32(a, b, c) }
}

/// Vector widening saturating doubling multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlsl_n_s16(a: int32x4_t, b: int16x4_t, c: i16) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmlsl_n_s16(a, b, c) }
}

/// Vector widening saturating doubling multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlsl_n_s32(a: int64x2_t, b: int32x2_t, c: i32) -> int64x2_t {
  unsafe { core::arch::aarch64::vqdmlsl_n_s32(a, b, c) }
}

/// Vector widening saturating doubling multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlsl_lane_s16<const N: i32>(a: int32x4_t, b: int16x4_t, c: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmlsl_lane_s16::<N>(a, b, c) }
}

/// Vector widening saturating doubling multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlsl_lane_s32<const N: i32>(a: int64x2_t, b: int32x2_t, c: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqdmlsl_lane_s32::<N>(a, b, c) }
}

/// Signed saturating doubling multiply returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulh_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vqdmulh_s16(a, b) }
}

/// Signed saturating doubling multiply returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulhq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vqdmulhq_s16(a, b) }
}

/// Signed saturating doubling multiply returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulh_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vqdmulh_s32(a, b) }
}

/// Signed saturating doubling multiply returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulhq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmulhq_s32(a, b) }
}

/// Vector saturating doubling multiply high with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulh_n_s16(a: int16x4_t, b: i16) -> int16x4_t {
  unsafe { core::arch::aarch64::vqdmulh_n_s16(a, b) }
}

/// Vector saturating doubling multiply high with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulh_n_s32(a: int32x2_t, b: i32) -> int32x2_t {
  unsafe { core::arch::aarch64::vqdmulh_n_s32(a, b) }
}

/// Vector saturating doubling multiply high with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulhq_n_s16(a: int16x8_t, b: i16) -> int16x8_t {
  unsafe { core::arch::aarch64::vqdmulhq_n_s16(a, b) }
}

/// Vector saturating doubling multiply high with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulhq_n_s32(a: int32x4_t, b: i32) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmulhq_n_s32(a, b) }
}

/// Vector saturating doubling multiply high by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulhq_laneq_s16<const LANE: i32>(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vqdmulhq_laneq_s16::<LANE>(a, b) }
}

/// Vector saturating doubling multiply high by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulh_laneq_s16<const LANE: i32>(a: int16x4_t, b: int16x8_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vqdmulh_laneq_s16::<LANE>(a, b) }
}

/// Vector saturating doubling multiply high by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulhq_laneq_s32<const LANE: i32>(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmulhq_laneq_s32::<LANE>(a, b) }
}

/// Vector saturating doubling multiply high by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulh_laneq_s32<const LANE: i32>(a: int32x2_t, b: int32x4_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vqdmulh_laneq_s32::<LANE>(a, b) }
}

/// Signed saturating extract narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovn_s16(a: int16x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vqmovn_s16(a) }
}

/// Signed saturating extract narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovn_s32(a: int32x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vqmovn_s32(a) }
}

/// Signed saturating extract narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovn_s64(a: int64x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vqmovn_s64(a) }
}

/// Unsigned saturating extract narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovn_u16(a: uint16x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vqmovn_u16(a) }
}

/// Unsigned saturating extract narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovn_u32(a: uint32x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vqmovn_u32(a) }
}

/// Unsigned saturating extract narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovn_u64(a: uint64x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vqmovn_u64(a) }
}

/// Signed saturating extract unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovun_s16(a: int16x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vqmovun_s16(a) }
}

/// Signed saturating extract unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovun_s32(a: int32x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vqmovun_s32(a) }
}

/// Signed saturating extract unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovun_s64(a: int64x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vqmovun_s64(a) }
}

/// Signed saturating rounding doubling multiply returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmulh_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vqrdmulh_s16(a, b) }
}

/// Signed saturating rounding doubling multiply returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmulhq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vqrdmulhq_s16(a, b) }
}

/// Signed saturating rounding doubling multiply returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmulh_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vqrdmulh_s32(a, b) }
}

/// Signed saturating rounding doubling multiply returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmulhq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqrdmulhq_s32(a, b) }
}

/// Vector saturating rounding doubling multiply high with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmulh_n_s16(a: int16x4_t, b: i16) -> int16x4_t {
  unsafe { core::arch::aarch64::vqrdmulh_n_s16(a, b) }
}

/// Vector saturating rounding doubling multiply high with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmulhq_n_s16(a: int16x8_t, b: i16) -> int16x8_t {
  unsafe { core::arch::aarch64::vqrdmulhq_n_s16(a, b) }
}

/// Vector saturating rounding doubling multiply high with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmulh_n_s32(a: int32x2_t, b: i32) -> int32x2_t {
  unsafe { core::arch::aarch64::vqrdmulh_n_s32(a, b) }
}

/// Vector saturating rounding doubling multiply high with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmulhq_n_s32(a: int32x4_t, b: i32) -> int32x4_t {
  unsafe { core::arch::aarch64::vqrdmulhq_n_s32(a, b) }
}

/// Vector rounding saturating doubling multiply high by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmulh_lane_s16<const LANE: i32>(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vqrdmulh_lane_s16::<LANE>(a, b) }
}

/// Vector rounding saturating doubling multiply high by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmulh_laneq_s16<const LANE: i32>(a: int16x4_t, b: int16x8_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vqrdmulh_laneq_s16::<LANE>(a, b) }
}

/// Vector rounding saturating doubling multiply high by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmulhq_lane_s16<const LANE: i32>(a: int16x8_t, b: int16x4_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vqrdmulhq_lane_s16::<LANE>(a, b) }
}

/// Vector rounding saturating doubling multiply high by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmulhq_laneq_s16<const LANE: i32>(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vqrdmulhq_laneq_s16::<LANE>(a, b) }
}

/// Vector rounding saturating doubling multiply high by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmulh_lane_s32<const LANE: i32>(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vqrdmulh_lane_s32::<LANE>(a, b) }
}

/// Vector rounding saturating doubling multiply high by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmulh_laneq_s32<const LANE: i32>(a: int32x2_t, b: int32x4_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vqrdmulh_laneq_s32::<LANE>(a, b) }
}

/// Vector rounding saturating doubling multiply high by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmulhq_lane_s32<const LANE: i32>(a: int32x4_t, b: int32x2_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqrdmulhq_lane_s32::<LANE>(a, b) }
}

/// Vector rounding saturating doubling multiply high by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmulhq_laneq_s32<const LANE: i32>(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqrdmulhq_laneq_s32::<LANE>(a, b) }
}

/// Signed saturating rounding doubling multiply subtract returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmlsh_s16(a: int16x4_t, b: int16x4_t, c: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vqrdmlsh_s16(a, b, c) }
}

/// Signed saturating rounding doubling multiply subtract returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmlshq_s16(a: int16x8_t, b: int16x8_t, c: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vqrdmlshq_s16(a, b, c) }
}

/// Signed saturating rounding doubling multiply subtract returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmlsh_s32(a: int32x2_t, b: int32x2_t, c: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vqrdmlsh_s32(a, b, c) }
}

/// Signed saturating rounding doubling multiply subtract returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmlshq_s32(a: int32x4_t, b: int32x4_t, c: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqrdmlshq_s32(a, b, c) }
}

/// Signed saturating rounding doubling multiply subtract returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmlsh_lane_s16<const LANE: i32>(a: int16x4_t, b: int16x4_t, c: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vqrdmlsh_lane_s16::<LANE>(a, b, c) }
}

/// Signed saturating rounding doubling multiply subtract returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmlsh_laneq_s16<const LANE: i32>(a: int16x4_t, b: int16x4_t, c: int16x8_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vqrdmlsh_laneq_s16::<LANE>(a, b, c) }
}

/// Signed saturating rounding doubling multiply subtract returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmlshq_lane_s16<const LANE: i32>(a: int16x8_t, b: int16x8_t, c: int16x4_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vqrdmlshq_lane_s16::<LANE>(a, b, c) }
}

/// Signed saturating rounding doubling multiply subtract returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmlshq_laneq_s16<const LANE: i32>(a: int16x8_t, b: int16x8_t, c: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vqrdmlshq_laneq_s16::<LANE>(a, b, c) }
}

/// Signed saturating rounding doubling multiply subtract returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmlsh_lane_s32<const LANE: i32>(a: int32x2_t, b: int32x2_t, c: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vqrdmlsh_lane_s32::<LANE>(a, b, c) }
}

/// Signed saturating rounding doubling multiply subtract returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmlsh_laneq_s32<const LANE: i32>(a: int32x2_t, b: int32x2_t, c: int32x4_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vqrdmlsh_laneq_s32::<LANE>(a, b, c) }
}

/// Signed saturating rounding doubling multiply subtract returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmlshq_lane_s32<const LANE: i32>(a: int32x4_t, b: int32x4_t, c: int32x2_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqrdmlshq_lane_s32::<LANE>(a, b, c) }
}

/// Signed saturating rounding doubling multiply subtract returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmlshq_laneq_s32<const LANE: i32>(a: int32x4_t, b: int32x4_t, c: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqrdmlshq_laneq_s32::<LANE>(a, b, c) }
}

/// Signed saturating rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshl_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vqrshl_s8(a, b) }
}

/// Signed saturating rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshlq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vqrshlq_s8(a, b) }
}

/// Signed saturating rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshl_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vqrshl_s16(a, b) }
}

/// Signed saturating rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshlq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vqrshlq_s16(a, b) }
}

/// Signed saturating rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshl_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vqrshl_s32(a, b) }
}

/// Signed saturating rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshlq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqrshlq_s32(a, b) }
}

/// Signed saturating rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshl_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vqrshl_s64(a, b) }
}

/// Signed saturating rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshlq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqrshlq_s64(a, b) }
}

/// Unsigned signed saturating rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshl_u8(a: uint8x8_t, b: int8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vqrshl_u8(a, b) }
}

/// Unsigned signed saturating rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshlq_u8(a: uint8x16_t, b: int8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vqrshlq_u8(a, b) }
}

/// Unsigned signed saturating rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshl_u16(a: uint16x4_t, b: int16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vqrshl_u16(a, b) }
}

/// Unsigned signed saturating rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshlq_u16(a: uint16x8_t, b: int16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vqrshlq_u16(a, b) }
}

/// Unsigned signed saturating rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshl_u32(a: uint32x2_t, b: int32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vqrshl_u32(a, b) }
}

/// Unsigned signed saturating rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshlq_u32(a: uint32x4_t, b: int32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vqrshlq_u32(a, b) }
}

/// Unsigned signed saturating rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshl_u64(a: uint64x1_t, b: int64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vqrshl_u64(a, b) }
}

/// Unsigned signed saturating rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshlq_u64(a: uint64x2_t, b: int64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vqrshlq_u64(a, b) }
}

/// Signed saturating rounded shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrn_n_s16<const N: i32>(a: int16x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vqrshrn_n_s16::<N>(a) }
}

/// Signed saturating rounded shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrn_n_s32<const N: i32>(a: int32x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vqrshrn_n_s32::<N>(a) }
}

/// Signed saturating rounded shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrn_n_s64<const N: i32>(a: int64x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vqrshrn_n_s64::<N>(a) }
}

/// Unsigned signed saturating rounded shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrn_n_u16<const N: i32>(a: uint16x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vqrshrn_n_u16::<N>(a) }
}

/// Unsigned signed saturating rounded shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrn_n_u32<const N: i32>(a: uint32x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vqrshrn_n_u32::<N>(a) }
}

/// Unsigned signed saturating rounded shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrn_n_u64<const N: i32>(a: uint64x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vqrshrn_n_u64::<N>(a) }
}

/// Signed saturating rounded shift right unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrun_n_s16<const N: i32>(a: int16x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vqrshrun_n_s16::<N>(a) }
}

/// Signed saturating rounded shift right unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrun_n_s32<const N: i32>(a: int32x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vqrshrun_n_s32::<N>(a) }
}

/// Signed saturating rounded shift right unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrun_n_s64<const N: i32>(a: int64x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vqrshrun_n_s64::<N>(a) }
}

/// Signed saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshl_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vqshl_s8(a, b) }
}

/// Signed saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vqshlq_s8(a, b) }
}

/// Signed saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshl_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vqshl_s16(a, b) }
}

/// Signed saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vqshlq_s16(a, b) }
}

/// Signed saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshl_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vqshl_s32(a, b) }
}

/// Signed saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqshlq_s32(a, b) }
}

/// Signed saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshl_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vqshl_s64(a, b) }
}

/// Signed saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqshlq_s64(a, b) }
}

/// Unsigned saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshl_u8(a: uint8x8_t, b: int8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vqshl_u8(a, b) }
}

/// Unsigned saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlq_u8(a: uint8x16_t, b: int8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vqshlq_u8(a, b) }
}

/// Unsigned saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshl_u16(a: uint16x4_t, b: int16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vqshl_u16(a, b) }
}

/// Unsigned saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlq_u16(a: uint16x8_t, b: int16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vqshlq_u16(a, b) }
}

/// Unsigned saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshl_u32(a: uint32x2_t, b: int32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vqshl_u32(a, b) }
}

/// Unsigned saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlq_u32(a: uint32x4_t, b: int32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vqshlq_u32(a, b) }
}

/// Unsigned saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshl_u64(a: uint64x1_t, b: int64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vqshl_u64(a, b) }
}

/// Unsigned saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlq_u64(a: uint64x2_t, b: int64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vqshlq_u64(a, b) }
}

/// Signed saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshl_n_s8<const N: i32>(a: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vqshl_n_s8::<N>(a) }
}

/// Signed saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlq_n_s8<const N: i32>(a: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vqshlq_n_s8::<N>(a) }
}

/// Signed saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshl_n_s16<const N: i32>(a: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vqshl_n_s16::<N>(a) }
}

/// Signed saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlq_n_s16<const N: i32>(a: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vqshlq_n_s16::<N>(a) }
}

/// Signed saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshl_n_s32<const N: i32>(a: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vqshl_n_s32::<N>(a) }
}

/// Signed saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlq_n_s32<const N: i32>(a: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqshlq_n_s32::<N>(a) }
}

/// Signed saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshl_n_s64<const N: i32>(a: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vqshl_n_s64::<N>(a) }
}

/// Signed saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlq_n_s64<const N: i32>(a: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqshlq_n_s64::<N>(a) }
}

/// Unsigned saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshl_n_u8<const N: i32>(a: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vqshl_n_u8::<N>(a) }
}

/// Unsigned saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlq_n_u8<const N: i32>(a: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vqshlq_n_u8::<N>(a) }
}

/// Unsigned saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshl_n_u16<const N: i32>(a: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vqshl_n_u16::<N>(a) }
}

/// Unsigned saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlq_n_u16<const N: i32>(a: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vqshlq_n_u16::<N>(a) }
}

/// Unsigned saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshl_n_u32<const N: i32>(a: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vqshl_n_u32::<N>(a) }
}

/// Unsigned saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlq_n_u32<const N: i32>(a: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vqshlq_n_u32::<N>(a) }
}

/// Unsigned saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshl_n_u64<const N: i32>(a: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vqshl_n_u64::<N>(a) }
}

/// Unsigned saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlq_n_u64<const N: i32>(a: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vqshlq_n_u64::<N>(a) }
}

/// Signed saturating shift left unsigned
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlu_n_s8<const N: i32>(a: int8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vqshlu_n_s8::<N>(a) }
}

/// Signed saturating shift left unsigned
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlu_n_s16<const N: i32>(a: int16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vqshlu_n_s16::<N>(a) }
}

/// Signed saturating shift left unsigned
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlu_n_s32<const N: i32>(a: int32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vqshlu_n_s32::<N>(a) }
}

/// Signed saturating shift left unsigned
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlu_n_s64<const N: i32>(a: int64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vqshlu_n_s64::<N>(a) }
}

/// Signed saturating shift left unsigned
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshluq_n_s8<const N: i32>(a: int8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vqshluq_n_s8::<N>(a) }
}

/// Signed saturating shift left unsigned
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshluq_n_s16<const N: i32>(a: int16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vqshluq_n_s16::<N>(a) }
}

/// Signed saturating shift left unsigned
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshluq_n_s32<const N: i32>(a: int32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vqshluq_n_s32::<N>(a) }
}

/// Signed saturating shift left unsigned
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshluq_n_s64<const N: i32>(a: int64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vqshluq_n_s64::<N>(a) }
}

/// Signed saturating shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrn_n_s16<const N: i32>(a: int16x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vqshrn_n_s16::<N>(a) }
}

/// Signed saturating shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrn_n_s32<const N: i32>(a: int32x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vqshrn_n_s32::<N>(a) }
}

/// Signed saturating shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrn_n_s64<const N: i32>(a: int64x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vqshrn_n_s64::<N>(a) }
}

/// Unsigned saturating shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrn_n_u16<const N: i32>(a: uint16x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vqshrn_n_u16::<N>(a) }
}

/// Unsigned saturating shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrn_n_u32<const N: i32>(a: uint32x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vqshrn_n_u32::<N>(a) }
}

/// Unsigned saturating shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrn_n_u64<const N: i32>(a: uint64x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vqshrn_n_u64::<N>(a) }
}

/// Signed saturating shift right unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrun_n_s16<const N: i32>(a: int16x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vqshrun_n_s16::<N>(a) }
}

/// Signed saturating shift right unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrun_n_s32<const N: i32>(a: int32x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vqshrun_n_s32::<N>(a) }
}

/// Signed saturating shift right unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrun_n_s64<const N: i32>(a: int64x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vqshrun_n_s64::<N>(a) }
}

/// Reciprocal square-root estimate.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsqrte_f32(a: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vrsqrte_f32(a) }
}

/// Reciprocal square-root estimate.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsqrteq_f32(a: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vrsqrteq_f32(a) }
}

/// Unsigned reciprocal square root estimate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsqrte_u32(a: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vrsqrte_u32(a) }
}

/// Unsigned reciprocal square root estimate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsqrteq_u32(a: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vrsqrteq_u32(a) }
}

/// Floating-point reciprocal square root step
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsqrts_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vrsqrts_f32(a, b) }
}

/// Floating-point reciprocal square root step
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsqrtsq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vrsqrtsq_f32(a, b) }
}

/// Reciprocal estimate.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrecpe_f32(a: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vrecpe_f32(a) }
}

/// Reciprocal estimate.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrecpeq_f32(a: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vrecpeq_f32(a) }
}

/// Unsigned reciprocal estimate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrecpe_u32(a: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vrecpe_u32(a) }
}

/// Unsigned reciprocal estimate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrecpeq_u32(a: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vrecpeq_u32(a) }
}

/// Floating-point reciprocal step
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrecps_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vrecps_f32(a, b) }
}

/// Floating-point reciprocal step
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrecpsq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vrecpsq_f32(a, b) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s8_u8(a: uint8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_s8_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s8_p8(a: poly8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_s8_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s16_p16(a: poly16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_s16_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s16_u16(a: uint16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_s16_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s32_u32(a: uint32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_s32_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s64_u64(a: uint64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_s64_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s8_u8(a: uint8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_s8_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s8_p8(a: poly8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_s8_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s16_p16(a: poly16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_s16_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s16_u16(a: uint16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_s16_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s32_u32(a: uint32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_s32_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s64_u64(a: uint64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_s64_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u8_p8(a: poly8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_u8_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u8_s8(a: int8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_u8_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u16_p16(a: poly16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_u16_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u16_s16(a: int16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_u16_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u32_s32(a: int32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_u32_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u64_s64(a: int64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_u64_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u8_p8(a: poly8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_u8_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u8_s8(a: int8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_u8_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u16_p16(a: poly16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_u16_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u16_s16(a: int16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_u16_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u32_s32(a: int32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_u32_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u64_s64(a: int64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_u64_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p8_s8(a: int8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_p8_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p8_u8(a: uint8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_p8_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p16_s16(a: int16x4_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_p16_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p16_u16(a: uint16x4_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_p16_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p8_s8(a: int8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_p8_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p8_u8(a: uint8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_p8_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p16_s16(a: int16x8_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_p16_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p16_u16(a: uint16x8_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_p16_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s8_s16(a: int16x4_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_s8_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s8_u16(a: uint16x4_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_s8_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s8_p16(a: poly16x4_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_s8_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s16_s32(a: int32x2_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_s16_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s16_u32(a: uint32x2_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_s16_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s32_s64(a: int64x1_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_s32_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s32_u64(a: uint64x1_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_s32_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s8_s16(a: int16x8_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_s8_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s8_u16(a: uint16x8_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_s8_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s8_p16(a: poly16x8_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_s8_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s16_s32(a: int32x4_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_s16_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s16_u32(a: uint32x4_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_s16_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s32_s64(a: int64x2_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_s32_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s32_u64(a: uint64x2_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_s32_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u8_p16(a: poly16x4_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_u8_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u8_s16(a: int16x4_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_u8_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u8_u16(a: uint16x4_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_u8_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u16_s32(a: int32x2_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_u16_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u16_u32(a: uint32x2_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_u16_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u32_s64(a: int64x1_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_u32_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u32_u64(a: uint64x1_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_u32_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u8_p16(a: poly16x8_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_u8_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u8_s16(a: int16x8_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_u8_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u8_u16(a: uint16x8_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_u8_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u16_s32(a: int32x4_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_u16_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u16_u32(a: uint32x4_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_u16_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u32_s64(a: int64x2_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_u32_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u32_u64(a: uint64x2_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_u32_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p8_p16(a: poly16x4_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_p8_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p8_s16(a: int16x4_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_p8_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p8_u16(a: uint16x4_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_p8_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p16_s32(a: int32x2_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_p16_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p16_u32(a: uint32x2_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_p16_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p8_p16(a: poly16x8_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_p8_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p8_s16(a: int16x8_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_p8_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p8_u16(a: uint16x8_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_p8_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p16_s32(a: int32x4_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_p16_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p16_u32(a: uint32x4_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_p16_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpret_s32_p64(a: poly64x1_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_s32_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpret_u32_p64(a: poly64x1_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_u32_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_s32_p64(a: poly64x2_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_s32_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_u32_p64(a: poly64x2_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_u32_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_s64_p128(a: p128) -> int64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_s64_p128(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_u64_p128(a: p128) -> uint64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_u64_p128(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_p64_p128(a: p128) -> poly64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_p64_p128(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s16_p8(a: poly8x8_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_s16_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s16_s8(a: int8x8_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_s16_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s16_u8(a: uint8x8_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_s16_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s32_p16(a: poly16x4_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_s32_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s32_s16(a: int16x4_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_s32_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s32_u16(a: uint16x4_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_s32_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s64_s32(a: int32x2_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_s64_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s64_u32(a: uint32x2_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_s64_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s16_p8(a: poly8x16_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_s16_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s16_s8(a: int8x16_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_s16_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s16_u8(a: uint8x16_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_s16_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s32_p16(a: poly16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_s32_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s32_s16(a: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_s32_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s32_u16(a: uint16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_s32_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s64_s32(a: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_s64_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s64_u32(a: uint32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_s64_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u16_p8(a: poly8x8_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_u16_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u16_s8(a: int8x8_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_u16_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u16_u8(a: uint8x8_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_u16_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u32_p16(a: poly16x4_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_u32_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u32_s16(a: int16x4_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_u32_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u32_u16(a: uint16x4_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_u32_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u64_s32(a: int32x2_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_u64_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u64_u32(a: uint32x2_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_u64_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u16_p8(a: poly8x16_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_u16_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u16_s8(a: int8x16_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_u16_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u16_u8(a: uint8x16_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_u16_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u32_p16(a: poly16x8_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_u32_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u32_s16(a: int16x8_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_u32_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u32_u16(a: uint16x8_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_u32_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u64_s32(a: int32x4_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_u64_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u64_u32(a: uint32x4_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_u64_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p16_p8(a: poly8x8_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_p16_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p16_s8(a: int8x8_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_p16_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p16_u8(a: uint8x8_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_p16_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p16_p8(a: poly8x16_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_p16_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p16_s8(a: int8x16_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_p16_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p16_u8(a: uint8x16_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_p16_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpret_p64_s32(a: int32x2_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_p64_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpret_p64_u32(a: uint32x2_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_p64_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_p64_s32(a: int32x4_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_p64_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_p64_u32(a: uint32x4_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_p64_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_p128_s64(a: int64x2_t) -> p128 {
  unsafe { core::arch::aarch64::vreinterpretq_p128_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_p128_u64(a: uint64x2_t) -> p128 {
  unsafe { core::arch::aarch64::vreinterpretq_p128_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_p128_p64(a: poly64x2_t) -> p128 {
  unsafe { core::arch::aarch64::vreinterpretq_p128_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s8_s32(a: int32x2_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_s8_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s8_u32(a: uint32x2_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_s8_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s16_s64(a: int64x1_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_s16_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s16_u64(a: uint64x1_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_s16_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s8_s32(a: int32x4_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_s8_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s8_u32(a: uint32x4_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_s8_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s16_s64(a: int64x2_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_s16_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s16_u64(a: uint64x2_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_s16_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u8_s32(a: int32x2_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_u8_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u8_u32(a: uint32x2_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_u8_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u16_s64(a: int64x1_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_u16_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u16_u64(a: uint64x1_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_u16_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u8_s32(a: int32x4_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_u8_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u8_u32(a: uint32x4_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_u8_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u16_s64(a: int64x2_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_u16_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u16_u64(a: uint64x2_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_u16_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p8_s32(a: int32x2_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_p8_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p8_u32(a: uint32x2_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_p8_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p16_s64(a: int64x1_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_p16_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p16_u64(a: uint64x1_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_p16_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p8_s32(a: int32x4_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_p8_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p8_u32(a: uint32x4_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_p8_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p16_s64(a: int64x2_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_p16_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p16_u64(a: uint64x2_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_p16_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpret_s16_p64(a: poly64x1_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_s16_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpret_u16_p64(a: poly64x1_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_u16_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpret_p16_p64(a: poly64x1_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_p16_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_s16_p64(a: poly64x2_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_s16_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_u16_p64(a: poly64x2_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_u16_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_p16_p64(a: poly64x2_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_p16_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_s32_p128(a: p128) -> int32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_s32_p128(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_u32_p128(a: p128) -> uint32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_u32_p128(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s32_p8(a: poly8x8_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_s32_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s32_s8(a: int8x8_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_s32_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s32_u8(a: uint8x8_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_s32_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s64_p16(a: poly16x4_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_s64_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s64_s16(a: int16x4_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_s64_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s64_u16(a: uint16x4_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_s64_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s32_p8(a: poly8x16_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_s32_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s32_s8(a: int8x16_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_s32_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s32_u8(a: uint8x16_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_s32_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s64_p16(a: poly16x8_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_s64_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s64_s16(a: int16x8_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_s64_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s64_u16(a: uint16x8_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_s64_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u32_p8(a: poly8x8_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_u32_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u32_s8(a: int8x8_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_u32_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u32_u8(a: uint8x8_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_u32_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u64_p16(a: poly16x4_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_u64_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u64_s16(a: int16x4_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_u64_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u64_u16(a: uint16x4_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_u64_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u32_p8(a: poly8x16_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_u32_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u32_s8(a: int8x16_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_u32_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u32_u8(a: uint8x16_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_u32_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u64_p16(a: poly16x8_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_u64_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u64_s16(a: int16x8_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_u64_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u64_u16(a: uint16x8_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_u64_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpret_p64_p16(a: poly16x4_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_p64_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpret_p64_s16(a: int16x4_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_p64_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpret_p64_u16(a: uint16x4_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_p64_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_p64_p16(a: poly16x8_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_p64_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_p64_s16(a: int16x8_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_p64_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_p64_u16(a: uint16x8_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_p64_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_p128_s32(a: int32x4_t) -> p128 {
  unsafe { core::arch::aarch64::vreinterpretq_p128_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_p128_u32(a: uint32x4_t) -> p128 {
  unsafe { core::arch::aarch64::vreinterpretq_p128_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s8_s64(a: int64x1_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_s8_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s8_u64(a: uint64x1_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_s8_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u8_s64(a: int64x1_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_u8_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u8_u64(a: uint64x1_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_u8_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p8_s64(a: int64x1_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_p8_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p8_u64(a: uint64x1_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_p8_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s8_s64(a: int64x2_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_s8_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s8_u64(a: uint64x2_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_s8_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u8_s64(a: int64x2_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_u8_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u8_u64(a: uint64x2_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_u8_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p8_s64(a: int64x2_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_p8_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p8_u64(a: uint64x2_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_p8_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpret_s8_p64(a: poly64x1_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_s8_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpret_u8_p64(a: poly64x1_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_u8_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpret_p8_p64(a: poly64x1_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_p8_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_s8_p64(a: poly64x2_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_s8_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_u8_p64(a: poly64x2_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_u8_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_p8_p64(a: poly64x2_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_p8_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_s16_p128(a: p128) -> int16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_s16_p128(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_u16_p128(a: p128) -> uint16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_u16_p128(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_p16_p128(a: p128) -> poly16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_p16_p128(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s64_p8(a: poly8x8_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_s64_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s64_s8(a: int8x8_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_s64_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s64_u8(a: uint8x8_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_s64_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u64_p8(a: poly8x8_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_u64_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u64_s8(a: int8x8_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_u64_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u64_u8(a: uint8x8_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_u64_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s64_p8(a: poly8x16_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_s64_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s64_s8(a: int8x16_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_s64_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s64_u8(a: uint8x16_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_s64_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u64_p8(a: poly8x16_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_u64_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u64_s8(a: int8x16_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_u64_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u64_u8(a: uint8x16_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_u64_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpret_p64_p8(a: poly8x8_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_p64_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpret_p64_s8(a: int8x8_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_p64_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpret_p64_u8(a: uint8x8_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_p64_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_p64_p8(a: poly8x16_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_p64_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_p64_s8(a: int8x16_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_p64_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_p64_u8(a: uint8x16_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_p64_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_p128_s16(a: int16x8_t) -> p128 {
  unsafe { core::arch::aarch64::vreinterpretq_p128_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_p128_u16(a: uint16x8_t) -> p128 {
  unsafe { core::arch::aarch64::vreinterpretq_p128_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_p128_p16(a: poly16x8_t) -> p128 {
  unsafe { core::arch::aarch64::vreinterpretq_p128_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_p128_s8(a: int8x16_t) -> p128 {
  unsafe { core::arch::aarch64::vreinterpretq_p128_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_p128_u8(a: uint8x16_t) -> p128 {
  unsafe { core::arch::aarch64::vreinterpretq_p128_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_p128_p8(a: poly8x16_t) -> p128 {
  unsafe { core::arch::aarch64::vreinterpretq_p128_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_s8_p128(a: p128) -> int8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_s8_p128(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_u8_p128(a: p128) -> uint8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_u8_p128(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vreinterpretq_p8_p128(a: p128) -> poly8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_p8_p128(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s8_f32(a: float32x2_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_s8_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s16_f32(a: float32x2_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_s16_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s32_f32(a: float32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_s32_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s64_f32(a: float32x2_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_s64_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s8_f32(a: float32x4_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_s8_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s16_f32(a: float32x4_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_s16_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s32_f32(a: float32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_s32_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s64_f32(a: float32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_s64_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u8_f32(a: float32x2_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_u8_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u16_f32(a: float32x2_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_u16_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u32_f32(a: float32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_u32_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u64_f32(a: float32x2_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_u64_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u8_f32(a: float32x4_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_u8_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u16_f32(a: float32x4_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_u16_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u32_f32(a: float32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_u32_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u64_f32(a: float32x4_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_u64_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p8_f32(a: float32x2_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_p8_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p16_f32(a: float32x2_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_p16_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p8_f32(a: float32x4_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_p8_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p16_f32(a: float32x4_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_p16_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p128_f32(a: float32x4_t) -> p128 {
  unsafe { core::arch::aarch64::vreinterpretq_p128_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_f32_s8(a: int8x8_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_f32_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_f32_s16(a: int16x4_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_f32_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_f32_s32(a: int32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_f32_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_f32_s64(a: int64x1_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_f32_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f32_s8(a: int8x16_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_f32_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f32_s16(a: int16x8_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_f32_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f32_s32(a: int32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_f32_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f32_s64(a: int64x2_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_f32_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_f32_u8(a: uint8x8_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_f32_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_f32_u16(a: uint16x4_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_f32_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_f32_u32(a: uint32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_f32_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_f32_u64(a: uint64x1_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_f32_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f32_u8(a: uint8x16_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_f32_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f32_u16(a: uint16x8_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_f32_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f32_u32(a: uint32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_f32_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f32_u64(a: uint64x2_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_f32_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_f32_p8(a: poly8x8_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_f32_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_f32_p16(a: poly16x4_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_f32_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f32_p8(a: poly8x16_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_f32_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f32_p16(a: poly16x8_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_f32_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f32_p128(a: p128) -> float32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_f32_p128(a) }
}

/// Signed rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshl_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vrshl_s8(a, b) }
}

/// Signed rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshlq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vrshlq_s8(a, b) }
}

/// Signed rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshl_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vrshl_s16(a, b) }
}

/// Signed rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshlq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vrshlq_s16(a, b) }
}

/// Signed rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshl_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vrshl_s32(a, b) }
}

/// Signed rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshlq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vrshlq_s32(a, b) }
}

/// Signed rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshl_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vrshl_s64(a, b) }
}

/// Signed rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshlq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vrshlq_s64(a, b) }
}

/// Unsigned rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshl_u8(a: uint8x8_t, b: int8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vrshl_u8(a, b) }
}

/// Unsigned rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshlq_u8(a: uint8x16_t, b: int8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vrshlq_u8(a, b) }
}

/// Unsigned rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshl_u16(a: uint16x4_t, b: int16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vrshl_u16(a, b) }
}

/// Unsigned rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshlq_u16(a: uint16x8_t, b: int16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vrshlq_u16(a, b) }
}

/// Unsigned rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshl_u32(a: uint32x2_t, b: int32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vrshl_u32(a, b) }
}

/// Unsigned rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshlq_u32(a: uint32x4_t, b: int32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vrshlq_u32(a, b) }
}

/// Unsigned rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshl_u64(a: uint64x1_t, b: int64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vrshl_u64(a, b) }
}

/// Unsigned rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshlq_u64(a: uint64x2_t, b: int64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vrshlq_u64(a, b) }
}

/// Signed rounding shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshr_n_s8<const N: i32>(a: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vrshr_n_s8::<N>(a) }
}

/// Signed rounding shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshrq_n_s8<const N: i32>(a: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vrshrq_n_s8::<N>(a) }
}

/// Signed rounding shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshr_n_s16<const N: i32>(a: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vrshr_n_s16::<N>(a) }
}

/// Signed rounding shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshrq_n_s16<const N: i32>(a: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vrshrq_n_s16::<N>(a) }
}

/// Signed rounding shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshr_n_s32<const N: i32>(a: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vrshr_n_s32::<N>(a) }
}

/// Signed rounding shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshrq_n_s32<const N: i32>(a: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vrshrq_n_s32::<N>(a) }
}

/// Signed rounding shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshr_n_s64<const N: i32>(a: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vrshr_n_s64::<N>(a) }
}

/// Signed rounding shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshrq_n_s64<const N: i32>(a: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vrshrq_n_s64::<N>(a) }
}

/// Unsigned rounding shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshr_n_u8<const N: i32>(a: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vrshr_n_u8::<N>(a) }
}

/// Unsigned rounding shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshrq_n_u8<const N: i32>(a: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vrshrq_n_u8::<N>(a) }
}

/// Unsigned rounding shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshr_n_u16<const N: i32>(a: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vrshr_n_u16::<N>(a) }
}

/// Unsigned rounding shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshrq_n_u16<const N: i32>(a: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vrshrq_n_u16::<N>(a) }
}

/// Unsigned rounding shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshr_n_u32<const N: i32>(a: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vrshr_n_u32::<N>(a) }
}

/// Unsigned rounding shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshrq_n_u32<const N: i32>(a: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vrshrq_n_u32::<N>(a) }
}

/// Unsigned rounding shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshr_n_u64<const N: i32>(a: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vrshr_n_u64::<N>(a) }
}

/// Unsigned rounding shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshrq_n_u64<const N: i32>(a: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vrshrq_n_u64::<N>(a) }
}

/// Rounding shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshrn_n_s16<const N: i32>(a: int16x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vrshrn_n_s16::<N>(a) }
}

/// Rounding shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshrn_n_s32<const N: i32>(a: int32x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vrshrn_n_s32::<N>(a) }
}

/// Rounding shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshrn_n_s64<const N: i32>(a: int64x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vrshrn_n_s64::<N>(a) }
}

/// Rounding shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshrn_n_u16<const N: i32>(a: uint16x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vrshrn_n_u16::<N>(a) }
}

/// Rounding shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshrn_n_u32<const N: i32>(a: uint32x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vrshrn_n_u32::<N>(a) }
}

/// Rounding shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshrn_n_u64<const N: i32>(a: uint64x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vrshrn_n_u64::<N>(a) }
}

/// Signed rounding shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsra_n_s8<const N: i32>(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vrsra_n_s8::<N>(a, b) }
}

/// Signed rounding shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsraq_n_s8<const N: i32>(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vrsraq_n_s8::<N>(a, b) }
}

/// Signed rounding shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsra_n_s16<const N: i32>(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vrsra_n_s16::<N>(a, b) }
}

/// Signed rounding shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsraq_n_s16<const N: i32>(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vrsraq_n_s16::<N>(a, b) }
}

/// Signed rounding shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsra_n_s32<const N: i32>(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vrsra_n_s32::<N>(a, b) }
}

/// Signed rounding shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsraq_n_s32<const N: i32>(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vrsraq_n_s32::<N>(a, b) }
}

/// Signed rounding shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsra_n_s64<const N: i32>(a: int64x1_t, b: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vrsra_n_s64::<N>(a, b) }
}

/// Signed rounding shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsraq_n_s64<const N: i32>(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vrsraq_n_s64::<N>(a, b) }
}

/// Unsigned rounding shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsra_n_u8<const N: i32>(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vrsra_n_u8::<N>(a, b) }
}

/// Unsigned rounding shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsraq_n_u8<const N: i32>(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vrsraq_n_u8::<N>(a, b) }
}

/// Unsigned rounding shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsra_n_u16<const N: i32>(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vrsra_n_u16::<N>(a, b) }
}

/// Unsigned rounding shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsraq_n_u16<const N: i32>(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vrsraq_n_u16::<N>(a, b) }
}

/// Unsigned rounding shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsra_n_u32<const N: i32>(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vrsra_n_u32::<N>(a, b) }
}

/// Unsigned rounding shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsraq_n_u32<const N: i32>(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vrsraq_n_u32::<N>(a, b) }
}

/// Unsigned rounding shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsra_n_u64<const N: i32>(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vrsra_n_u64::<N>(a, b) }
}

/// Unsigned rounding shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsraq_n_u64<const N: i32>(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vrsraq_n_u64::<N>(a, b) }
}

/// Rounding subtract returning high narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsubhn_s16(a: int16x8_t, b: int16x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vrsubhn_s16(a, b) }
}

/// Rounding subtract returning high narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsubhn_s32(a: int32x4_t, b: int32x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vrsubhn_s32(a, b) }
}

/// Rounding subtract returning high narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsubhn_s64(a: int64x2_t, b: int64x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vrsubhn_s64(a, b) }
}

/// Rounding subtract returning high narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsubhn_u16(a: uint16x8_t, b: uint16x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vrsubhn_u16(a, b) }
}

/// Rounding subtract returning high narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsubhn_u32(a: uint32x4_t, b: uint32x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vrsubhn_u32(a, b) }
}

/// Rounding subtract returning high narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsubhn_u64(a: uint64x2_t, b: uint64x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vrsubhn_u64(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vset_lane_s8<const LANE: i32>(a: i8, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vset_lane_s8::<LANE>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vset_lane_s16<const LANE: i32>(a: i16, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vset_lane_s16::<LANE>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vset_lane_s32<const LANE: i32>(a: i32, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vset_lane_s32::<LANE>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vset_lane_s64<const LANE: i32>(a: i64, b: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vset_lane_s64::<LANE>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vset_lane_u8<const LANE: i32>(a: u8, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vset_lane_u8::<LANE>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vset_lane_u16<const LANE: i32>(a: u16, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vset_lane_u16::<LANE>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vset_lane_u32<const LANE: i32>(a: u32, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vset_lane_u32::<LANE>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vset_lane_u64<const LANE: i32>(a: u64, b: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vset_lane_u64::<LANE>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vset_lane_p8<const LANE: i32>(a: p8, b: poly8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vset_lane_p8::<LANE>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vset_lane_p16<const LANE: i32>(a: p16, b: poly16x4_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vset_lane_p16::<LANE>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vset_lane_p64<const LANE: i32>(a: p64, b: poly64x1_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vset_lane_p64::<LANE>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsetq_lane_s8<const LANE: i32>(a: i8, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vsetq_lane_s8::<LANE>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsetq_lane_s16<const LANE: i32>(a: i16, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vsetq_lane_s16::<LANE>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsetq_lane_s32<const LANE: i32>(a: i32, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vsetq_lane_s32::<LANE>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsetq_lane_s64<const LANE: i32>(a: i64, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vsetq_lane_s64::<LANE>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsetq_lane_u8<const LANE: i32>(a: u8, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vsetq_lane_u8::<LANE>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsetq_lane_u16<const LANE: i32>(a: u16, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vsetq_lane_u16::<LANE>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsetq_lane_u32<const LANE: i32>(a: u32, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vsetq_lane_u32::<LANE>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsetq_lane_u64<const LANE: i32>(a: u64, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vsetq_lane_u64::<LANE>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsetq_lane_p8<const LANE: i32>(a: p8, b: poly8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vsetq_lane_p8::<LANE>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsetq_lane_p16<const LANE: i32>(a: p16, b: poly16x8_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vsetq_lane_p16::<LANE>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vsetq_lane_p64<const LANE: i32>(a: p64, b: poly64x2_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vsetq_lane_p64::<LANE>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vset_lane_f32<const LANE: i32>(a: f32, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vset_lane_f32::<LANE>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsetq_lane_f32<const LANE: i32>(a: f32, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vsetq_lane_f32::<LANE>(a, b) }
}

/// Signed Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshl_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vshl_s8(a, b) }
}

/// Signed Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshlq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vshlq_s8(a, b) }
}

/// Signed Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshl_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vshl_s16(a, b) }
}

/// Signed Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshlq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vshlq_s16(a, b) }
}

/// Signed Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshl_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vshl_s32(a, b) }
}

/// Signed Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshlq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vshlq_s32(a, b) }
}

/// Signed Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshl_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vshl_s64(a, b) }
}

/// Signed Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshlq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vshlq_s64(a, b) }
}

/// Unsigned Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshl_u8(a: uint8x8_t, b: int8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vshl_u8(a, b) }
}

/// Unsigned Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshlq_u8(a: uint8x16_t, b: int8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vshlq_u8(a, b) }
}

/// Unsigned Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshl_u16(a: uint16x4_t, b: int16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vshl_u16(a, b) }
}

/// Unsigned Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshlq_u16(a: uint16x8_t, b: int16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vshlq_u16(a, b) }
}

/// Unsigned Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshl_u32(a: uint32x2_t, b: int32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vshl_u32(a, b) }
}

/// Unsigned Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshlq_u32(a: uint32x4_t, b: int32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vshlq_u32(a, b) }
}

/// Unsigned Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshl_u64(a: uint64x1_t, b: int64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vshl_u64(a, b) }
}

/// Unsigned Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshlq_u64(a: uint64x2_t, b: int64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vshlq_u64(a, b) }
}

/// Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshl_n_s8<const N: i32>(a: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vshl_n_s8::<N>(a) }
}

/// Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshlq_n_s8<const N: i32>(a: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vshlq_n_s8::<N>(a) }
}

/// Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshl_n_s16<const N: i32>(a: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vshl_n_s16::<N>(a) }
}

/// Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshlq_n_s16<const N: i32>(a: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vshlq_n_s16::<N>(a) }
}

/// Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshl_n_s32<const N: i32>(a: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vshl_n_s32::<N>(a) }
}

/// Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshlq_n_s32<const N: i32>(a: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vshlq_n_s32::<N>(a) }
}

/// Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshl_n_u8<const N: i32>(a: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vshl_n_u8::<N>(a) }
}

/// Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshlq_n_u8<const N: i32>(a: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vshlq_n_u8::<N>(a) }
}

/// Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshl_n_u16<const N: i32>(a: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vshl_n_u16::<N>(a) }
}

/// Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshlq_n_u16<const N: i32>(a: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vshlq_n_u16::<N>(a) }
}

/// Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshl_n_u32<const N: i32>(a: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vshl_n_u32::<N>(a) }
}

/// Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshlq_n_u32<const N: i32>(a: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vshlq_n_u32::<N>(a) }
}

/// Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshl_n_s64<const N: i32>(a: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vshl_n_s64::<N>(a) }
}

/// Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshlq_n_s64<const N: i32>(a: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vshlq_n_s64::<N>(a) }
}

/// Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshl_n_u64<const N: i32>(a: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vshl_n_u64::<N>(a) }
}

/// Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshlq_n_u64<const N: i32>(a: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vshlq_n_u64::<N>(a) }
}

/// Signed shift left long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshll_n_s8<const N: i32>(a: int8x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vshll_n_s8::<N>(a) }
}

/// Signed shift left long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshll_n_s16<const N: i32>(a: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vshll_n_s16::<N>(a) }
}

/// Signed shift left long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshll_n_s32<const N: i32>(a: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vshll_n_s32::<N>(a) }
}

/// Signed shift left long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshll_n_u8<const N: i32>(a: uint8x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vshll_n_u8::<N>(a) }
}

/// Signed shift left long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshll_n_u16<const N: i32>(a: uint16x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vshll_n_u16::<N>(a) }
}

/// Signed shift left long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshll_n_u32<const N: i32>(a: uint32x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vshll_n_u32::<N>(a) }
}

/// Shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshr_n_s8<const N: i32>(a: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vshr_n_s8::<N>(a) }
}

/// Shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshrq_n_s8<const N: i32>(a: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vshrq_n_s8::<N>(a) }
}

/// Shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshr_n_s16<const N: i32>(a: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vshr_n_s16::<N>(a) }
}

/// Shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshrq_n_s16<const N: i32>(a: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vshrq_n_s16::<N>(a) }
}

/// Shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshr_n_s32<const N: i32>(a: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vshr_n_s32::<N>(a) }
}

/// Shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshrq_n_s32<const N: i32>(a: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vshrq_n_s32::<N>(a) }
}

/// Shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshr_n_s64<const N: i32>(a: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vshr_n_s64::<N>(a) }
}

/// Shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshrq_n_s64<const N: i32>(a: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vshrq_n_s64::<N>(a) }
}

/// Shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshr_n_u8<const N: i32>(a: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vshr_n_u8::<N>(a) }
}

/// Shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshrq_n_u8<const N: i32>(a: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vshrq_n_u8::<N>(a) }
}

/// Shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshr_n_u16<const N: i32>(a: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vshr_n_u16::<N>(a) }
}

/// Shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshrq_n_u16<const N: i32>(a: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vshrq_n_u16::<N>(a) }
}

/// Shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshr_n_u32<const N: i32>(a: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vshr_n_u32::<N>(a) }
}

/// Shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshrq_n_u32<const N: i32>(a: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vshrq_n_u32::<N>(a) }
}

/// Shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshr_n_u64<const N: i32>(a: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vshr_n_u64::<N>(a) }
}

/// Shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshrq_n_u64<const N: i32>(a: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vshrq_n_u64::<N>(a) }
}

/// Shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshrn_n_s16<const N: i32>(a: int16x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vshrn_n_s16::<N>(a) }
}

/// Shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshrn_n_s32<const N: i32>(a: int32x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vshrn_n_s32::<N>(a) }
}

/// Shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshrn_n_s64<const N: i32>(a: int64x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vshrn_n_s64::<N>(a) }
}

/// Shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshrn_n_u16<const N: i32>(a: uint16x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vshrn_n_u16::<N>(a) }
}

/// Shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshrn_n_u32<const N: i32>(a: uint32x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vshrn_n_u32::<N>(a) }
}

/// Shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshrn_n_u64<const N: i32>(a: uint64x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vshrn_n_u64::<N>(a) }
}

/// Signed shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsra_n_s8<const N: i32>(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vsra_n_s8::<N>(a, b) }
}

/// Signed shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsraq_n_s8<const N: i32>(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vsraq_n_s8::<N>(a, b) }
}

/// Signed shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsra_n_s16<const N: i32>(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vsra_n_s16::<N>(a, b) }
}

/// Signed shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsraq_n_s16<const N: i32>(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vsraq_n_s16::<N>(a, b) }
}

/// Signed shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsra_n_s32<const N: i32>(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vsra_n_s32::<N>(a, b) }
}

/// Signed shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsraq_n_s32<const N: i32>(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vsraq_n_s32::<N>(a, b) }
}

/// Signed shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsra_n_s64<const N: i32>(a: int64x1_t, b: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vsra_n_s64::<N>(a, b) }
}

/// Signed shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsraq_n_s64<const N: i32>(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vsraq_n_s64::<N>(a, b) }
}

/// Unsigned shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsra_n_u8<const N: i32>(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vsra_n_u8::<N>(a, b) }
}

/// Unsigned shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsraq_n_u8<const N: i32>(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vsraq_n_u8::<N>(a, b) }
}

/// Unsigned shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsra_n_u16<const N: i32>(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vsra_n_u16::<N>(a, b) }
}

/// Unsigned shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsraq_n_u16<const N: i32>(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vsraq_n_u16::<N>(a, b) }
}

/// Unsigned shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsra_n_u32<const N: i32>(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vsra_n_u32::<N>(a, b) }
}

/// Unsigned shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsraq_n_u32<const N: i32>(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vsraq_n_u32::<N>(a, b) }
}

/// Unsigned shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsra_n_u64<const N: i32>(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vsra_n_u64::<N>(a, b) }
}

/// Unsigned shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsraq_n_u64<const N: i32>(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vsraq_n_u64::<N>(a, b) }
}

/// Transpose elements
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn_s8(a: int8x8_t, b: int8x8_t) -> int8x8x2_t {
  unsafe { core::arch::aarch64::vtrn_s8(a, b) }
}

/// Transpose elements
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn_s16(a: int16x4_t, b: int16x4_t) -> int16x4x2_t {
  unsafe { core::arch::aarch64::vtrn_s16(a, b) }
}

/// Transpose elements
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrnq_s8(a: int8x16_t, b: int8x16_t) -> int8x16x2_t {
  unsafe { core::arch::aarch64::vtrnq_s8(a, b) }
}

/// Transpose elements
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrnq_s16(a: int16x8_t, b: int16x8_t) -> int16x8x2_t {
  unsafe { core::arch::aarch64::vtrnq_s16(a, b) }
}

/// Transpose elements
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrnq_s32(a: int32x4_t, b: int32x4_t) -> int32x4x2_t {
  unsafe { core::arch::aarch64::vtrnq_s32(a, b) }
}

/// Transpose elements
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8x2_t {
  unsafe { core::arch::aarch64::vtrn_u8(a, b) }
}

/// Transpose elements
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4x2_t {
  unsafe { core::arch::aarch64::vtrn_u16(a, b) }
}

/// Transpose elements
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrnq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16x2_t {
  unsafe { core::arch::aarch64::vtrnq_u8(a, b) }
}

/// Transpose elements
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrnq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8x2_t {
  unsafe { core::arch::aarch64::vtrnq_u16(a, b) }
}

/// Transpose elements
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrnq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4x2_t {
  unsafe { core::arch::aarch64::vtrnq_u32(a, b) }
}

/// Transpose elements
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn_p8(a: poly8x8_t, b: poly8x8_t) -> poly8x8x2_t {
  unsafe { core::arch::aarch64::vtrn_p8(a, b) }
}

/// Transpose elements
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn_p16(a: poly16x4_t, b: poly16x4_t) -> poly16x4x2_t {
  unsafe { core::arch::aarch64::vtrn_p16(a, b) }
}

/// Transpose elements
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrnq_p8(a: poly8x16_t, b: poly8x16_t) -> poly8x16x2_t {
  unsafe { core::arch::aarch64::vtrnq_p8(a, b) }
}

/// Transpose elements
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrnq_p16(a: poly16x8_t, b: poly16x8_t) -> poly16x8x2_t {
  unsafe { core::arch::aarch64::vtrnq_p16(a, b) }
}

/// Transpose elements
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn_s32(a: int32x2_t, b: int32x2_t) -> int32x2x2_t {
  unsafe { core::arch::aarch64::vtrn_s32(a, b) }
}

/// Transpose elements
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2x2_t {
  unsafe { core::arch::aarch64::vtrn_u32(a, b) }
}

/// Transpose elements
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn_f32(a: float32x2_t, b: float32x2_t) -> float32x2x2_t {
  unsafe { core::arch::aarch64::vtrn_f32(a, b) }
}

/// Transpose elements
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrnq_f32(a: float32x4_t, b: float32x4_t) -> float32x4x2_t {
  unsafe { core::arch::aarch64::vtrnq_f32(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip_s8(a: int8x8_t, b: int8x8_t) -> int8x8x2_t {
  unsafe { core::arch::aarch64::vzip_s8(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip_s16(a: int16x4_t, b: int16x4_t) -> int16x4x2_t {
  unsafe { core::arch::aarch64::vzip_s16(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8x2_t {
  unsafe { core::arch::aarch64::vzip_u8(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4x2_t {
  unsafe { core::arch::aarch64::vzip_u16(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip_p8(a: poly8x8_t, b: poly8x8_t) -> poly8x8x2_t {
  unsafe { core::arch::aarch64::vzip_p8(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip_p16(a: poly16x4_t, b: poly16x4_t) -> poly16x4x2_t {
  unsafe { core::arch::aarch64::vzip_p16(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip_s32(a: int32x2_t, b: int32x2_t) -> int32x2x2_t {
  unsafe { core::arch::aarch64::vzip_s32(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2x2_t {
  unsafe { core::arch::aarch64::vzip_u32(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzipq_s8(a: int8x16_t, b: int8x16_t) -> int8x16x2_t {
  unsafe { core::arch::aarch64::vzipq_s8(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzipq_s16(a: int16x8_t, b: int16x8_t) -> int16x8x2_t {
  unsafe { core::arch::aarch64::vzipq_s16(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzipq_s32(a: int32x4_t, b: int32x4_t) -> int32x4x2_t {
  unsafe { core::arch::aarch64::vzipq_s32(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzipq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16x2_t {
  unsafe { core::arch::aarch64::vzipq_u8(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzipq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8x2_t {
  unsafe { core::arch::aarch64::vzipq_u16(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzipq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4x2_t {
  unsafe { core::arch::aarch64::vzipq_u32(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzipq_p8(a: poly8x16_t, b: poly8x16_t) -> poly8x16x2_t {
  unsafe { core::arch::aarch64::vzipq_p8(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzipq_p16(a: poly16x8_t, b: poly16x8_t) -> poly16x8x2_t {
  unsafe { core::arch::aarch64::vzipq_p16(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip_f32(a: float32x2_t, b: float32x2_t) -> float32x2x2_t {
  unsafe { core::arch::aarch64::vzip_f32(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzipq_f32(a: float32x4_t, b: float32x4_t) -> float32x4x2_t {
  unsafe { core::arch::aarch64::vzipq_f32(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp_s8(a: int8x8_t, b: int8x8_t) -> int8x8x2_t {
  unsafe { core::arch::aarch64::vuzp_s8(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp_s16(a: int16x4_t, b: int16x4_t) -> int16x4x2_t {
  unsafe { core::arch::aarch64::vuzp_s16(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzpq_s8(a: int8x16_t, b: int8x16_t) -> int8x16x2_t {
  unsafe { core::arch::aarch64::vuzpq_s8(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzpq_s16(a: int16x8_t, b: int16x8_t) -> int16x8x2_t {
  unsafe { core::arch::aarch64::vuzpq_s16(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzpq_s32(a: int32x4_t, b: int32x4_t) -> int32x4x2_t {
  unsafe { core::arch::aarch64::vuzpq_s32(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8x2_t {
  unsafe { core::arch::aarch64::vuzp_u8(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4x2_t {
  unsafe { core::arch::aarch64::vuzp_u16(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzpq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16x2_t {
  unsafe { core::arch::aarch64::vuzpq_u8(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzpq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8x2_t {
  unsafe { core::arch::aarch64::vuzpq_u16(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzpq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4x2_t {
  unsafe { core::arch::aarch64::vuzpq_u32(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp_p8(a: poly8x8_t, b: poly8x8_t) -> poly8x8x2_t {
  unsafe { core::arch::aarch64::vuzp_p8(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp_p16(a: poly16x4_t, b: poly16x4_t) -> poly16x4x2_t {
  unsafe { core::arch::aarch64::vuzp_p16(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzpq_p8(a: poly8x16_t, b: poly8x16_t) -> poly8x16x2_t {
  unsafe { core::arch::aarch64::vuzpq_p8(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzpq_p16(a: poly16x8_t, b: poly16x8_t) -> poly16x8x2_t {
  unsafe { core::arch::aarch64::vuzpq_p16(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp_s32(a: int32x2_t, b: int32x2_t) -> int32x2x2_t {
  unsafe { core::arch::aarch64::vuzp_s32(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2x2_t {
  unsafe { core::arch::aarch64::vuzp_u32(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp_f32(a: float32x2_t, b: float32x2_t) -> float32x2x2_t {
  unsafe { core::arch::aarch64::vuzp_f32(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzpq_f32(a: float32x4_t, b: float32x4_t) -> float32x4x2_t {
  unsafe { core::arch::aarch64::vuzpq_f32(a, b) }
}

/// Unsigned Absolute difference and Accumulate Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabal_u8(a: uint16x8_t, b: uint8x8_t, c: uint8x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vabal_u8(a, b, c) }
}

/// Unsigned Absolute difference and Accumulate Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabal_u16(a: uint32x4_t, b: uint16x4_t, c: uint16x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vabal_u16(a, b, c) }
}

/// Unsigned Absolute difference and Accumulate Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabal_u32(a: uint64x2_t, b: uint32x2_t, c: uint32x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vabal_u32(a, b, c) }
}

/// Signed Absolute difference and Accumulate Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabal_s8(a: int16x8_t, b: int8x8_t, c: int8x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vabal_s8(a, b, c) }
}

/// Signed Absolute difference and Accumulate Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabal_s16(a: int32x4_t, b: int16x4_t, c: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vabal_s16(a, b, c) }
}

/// Signed Absolute difference and Accumulate Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabal_s32(a: int64x2_t, b: int32x2_t, c: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vabal_s32(a, b, c) }
}

/// Singned saturating Absolute value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqabs_s8(a: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vqabs_s8(a) }
}

/// Singned saturating Absolute value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqabsq_s8(a: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vqabsq_s8(a) }
}

/// Singned saturating Absolute value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqabs_s16(a: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vqabs_s16(a) }
}

/// Singned saturating Absolute value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqabsq_s16(a: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vqabsq_s16(a) }
}

/// Singned saturating Absolute value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqabs_s32(a: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vqabs_s32(a) }
}

/// Singned saturating Absolute value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqabsq_s32(a: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqabsq_s32(a) }
}

/// Three-way exclusive OR
#[inline]
#[cfg(target_feature = "neon,sha3")]
pub fn veor3q_s8(a: int8x16_t, b: int8x16_t, c: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::veor3q_s8(a, b, c) }
}

/// Three-way exclusive OR
#[inline]
#[cfg(target_feature = "neon,sha3")]
pub fn veor3q_s16(a: int16x8_t, b: int16x8_t, c: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::veor3q_s16(a, b, c) }
}

/// Three-way exclusive OR
#[inline]
#[cfg(target_feature = "neon,sha3")]
pub fn veor3q_s32(a: int32x4_t, b: int32x4_t, c: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::veor3q_s32(a, b, c) }
}

/// Three-way exclusive OR
#[inline]
#[cfg(target_feature = "neon,sha3")]
pub fn veor3q_s64(a: int64x2_t, b: int64x2_t, c: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::veor3q_s64(a, b, c) }
}

/// Three-way exclusive OR
#[inline]
#[cfg(target_feature = "neon,sha3")]
pub fn veor3q_u8(a: uint8x16_t, b: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::veor3q_u8(a, b, c) }
}

/// Three-way exclusive OR
#[inline]
#[cfg(target_feature = "neon,sha3")]
pub fn veor3q_u16(a: uint16x8_t, b: uint16x8_t, c: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::veor3q_u16(a, b, c) }
}

/// Three-way exclusive OR
#[inline]
#[cfg(target_feature = "neon,sha3")]
pub fn veor3q_u32(a: uint32x4_t, b: uint32x4_t, c: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::veor3q_u32(a, b, c) }
}

/// Three-way exclusive OR
#[inline]
#[cfg(target_feature = "neon,sha3")]
pub fn veor3q_u64(a: uint64x2_t, b: uint64x2_t, c: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::veor3q_u64(a, b, c) }
}

/// Absolute difference between the arguments of Floating
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabd_f64(a: float64x1_t, b: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vabd_f64(a, b) }
}

/// Absolute difference between the arguments of Floating
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabdq_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vabdq_f64(a, b) }
}

/// Floating-point absolute difference
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabds_f32(a: f32, b: f32) -> f32 {
  unsafe { core::arch::aarch64::vabds_f32(a, b) }
}

/// Floating-point absolute difference
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabdd_f64(a: f64, b: f64) -> f64 {
  unsafe { core::arch::aarch64::vabdd_f64(a, b) }
}

/// Unsigned Absolute difference Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabdl_high_u8(a: uint8x16_t, b: uint8x16_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vabdl_high_u8(a, b) }
}

/// Unsigned Absolute difference Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabdl_high_u16(a: uint16x8_t, b: uint16x8_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vabdl_high_u16(a, b) }
}

/// Unsigned Absolute difference Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabdl_high_u32(a: uint32x4_t, b: uint32x4_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vabdl_high_u32(a, b) }
}

/// Signed Absolute difference Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabdl_high_s8(a: int8x16_t, b: int8x16_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vabdl_high_s8(a, b) }
}

/// Signed Absolute difference Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabdl_high_s16(a: int16x8_t, b: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vabdl_high_s16(a, b) }
}

/// Signed Absolute difference Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabdl_high_s32(a: int32x4_t, b: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vabdl_high_s32(a, b) }
}

/// Compare bitwise Equal (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceq_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vceq_u64(a, b) }
}

/// Compare bitwise Equal (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vceqq_u64(a, b) }
}

/// Compare bitwise Equal (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceq_s64(a: int64x1_t, b: int64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vceq_s64(a, b) }
}

/// Compare bitwise Equal (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqq_s64(a: int64x2_t, b: int64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vceqq_s64(a, b) }
}

/// Compare bitwise Equal (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceq_p64(a: poly64x1_t, b: poly64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vceq_p64(a, b) }
}

/// Compare bitwise Equal (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqq_p64(a: poly64x2_t, b: poly64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vceqq_p64(a, b) }
}

/// Floating-point compare equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceq_f64(a: float64x1_t, b: float64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vceq_f64(a, b) }
}

/// Floating-point compare equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqq_f64(a: float64x2_t, b: float64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vceqq_f64(a, b) }
}

/// Compare bitwise equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqd_s64(a: i64, b: i64) -> u64 {
  unsafe { core::arch::aarch64::vceqd_s64(a, b) }
}

/// Compare bitwise equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqd_u64(a: u64, b: u64) -> u64 {
  unsafe { core::arch::aarch64::vceqd_u64(a, b) }
}

/// Floating-point compare equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqs_f32(a: f32, b: f32) -> u32 {
  unsafe { core::arch::aarch64::vceqs_f32(a, b) }
}

/// Floating-point compare equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqd_f64(a: f64, b: f64) -> u64 {
  unsafe { core::arch::aarch64::vceqd_f64(a, b) }
}

/// Signed compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqz_s8(a: int8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vceqz_s8(a) }
}

/// Signed compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqzq_s8(a: int8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vceqzq_s8(a) }
}

/// Signed compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqz_s16(a: int16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vceqz_s16(a) }
}

/// Signed compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqzq_s16(a: int16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vceqzq_s16(a) }
}

/// Signed compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqz_s32(a: int32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vceqz_s32(a) }
}

/// Signed compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqzq_s32(a: int32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vceqzq_s32(a) }
}

/// Signed compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqz_s64(a: int64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vceqz_s64(a) }
}

/// Signed compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqzq_s64(a: int64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vceqzq_s64(a) }
}

/// Signed compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqz_p8(a: poly8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vceqz_p8(a) }
}

/// Signed compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqzq_p8(a: poly8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vceqzq_p8(a) }
}

/// Signed compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqz_p64(a: poly64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vceqz_p64(a) }
}

/// Signed compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqzq_p64(a: poly64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vceqzq_p64(a) }
}

/// Unsigned compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqz_u8(a: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vceqz_u8(a) }
}

/// Unsigned compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqzq_u8(a: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vceqzq_u8(a) }
}

/// Unsigned compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqz_u16(a: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vceqz_u16(a) }
}

/// Unsigned compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqzq_u16(a: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vceqzq_u16(a) }
}

/// Unsigned compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqz_u32(a: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vceqz_u32(a) }
}

/// Unsigned compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqzq_u32(a: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vceqzq_u32(a) }
}

/// Unsigned compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqz_u64(a: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vceqz_u64(a) }
}

/// Unsigned compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqzq_u64(a: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vceqzq_u64(a) }
}

/// Floating-point compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqz_f32(a: float32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vceqz_f32(a) }
}

/// Floating-point compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqzq_f32(a: float32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vceqzq_f32(a) }
}

/// Floating-point compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqz_f64(a: float64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vceqz_f64(a) }
}

/// Floating-point compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqzq_f64(a: float64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vceqzq_f64(a) }
}

/// Compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqzd_s64(a: i64) -> u64 {
  unsafe { core::arch::aarch64::vceqzd_s64(a) }
}

/// Compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqzd_u64(a: u64) -> u64 {
  unsafe { core::arch::aarch64::vceqzd_u64(a) }
}

/// Floating-point compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqzs_f32(a: f32) -> u32 {
  unsafe { core::arch::aarch64::vceqzs_f32(a) }
}

/// Floating-point compare bitwise equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vceqzd_f64(a: f64) -> u64 {
  unsafe { core::arch::aarch64::vceqzd_f64(a) }
}

/// Signed compare bitwise Test bits nonzero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtst_s64(a: int64x1_t, b: int64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vtst_s64(a, b) }
}

/// Signed compare bitwise Test bits nonzero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtstq_s64(a: int64x2_t, b: int64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vtstq_s64(a, b) }
}

/// Signed compare bitwise Test bits nonzero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtst_p64(a: poly64x1_t, b: poly64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vtst_p64(a, b) }
}

/// Signed compare bitwise Test bits nonzero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtstq_p64(a: poly64x2_t, b: poly64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vtstq_p64(a, b) }
}

/// Unsigned compare bitwise Test bits nonzero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtst_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vtst_u64(a, b) }
}

/// Unsigned compare bitwise Test bits nonzero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtstq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vtstq_u64(a, b) }
}

/// Compare bitwise test bits nonzero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtstd_s64(a: i64, b: i64) -> u64 {
  unsafe { core::arch::aarch64::vtstd_s64(a, b) }
}

/// Compare bitwise test bits nonzero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtstd_u64(a: u64, b: u64) -> u64 {
  unsafe { core::arch::aarch64::vtstd_u64(a, b) }
}

/// Signed saturating accumulate of unsigned value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuqadds_s32(a: i32, b: u32) -> i32 {
  unsafe { core::arch::aarch64::vuqadds_s32(a, b) }
}

/// Signed saturating accumulate of unsigned value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuqaddd_s64(a: i64, b: u64) -> i64 {
  unsafe { core::arch::aarch64::vuqaddd_s64(a, b) }
}

/// Signed saturating accumulate of unsigned value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuqaddb_s8(a: i8, b: u8) -> i8 {
  unsafe { core::arch::aarch64::vuqaddb_s8(a, b) }
}

/// Signed saturating accumulate of unsigned value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuqaddh_s16(a: i16, b: u16) -> i16 {
  unsafe { core::arch::aarch64::vuqaddh_s16(a, b) }
}

/// Floating-point absolute value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabs_f64(a: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vabs_f64(a) }
}

/// Floating-point absolute value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabsq_f64(a: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vabsq_f64(a) }
}

/// Compare signed greater than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgt_s64(a: int64x1_t, b: int64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcgt_s64(a, b) }
}

/// Compare signed greater than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtq_s64(a: int64x2_t, b: int64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcgtq_s64(a, b) }
}

/// Compare unsigned highe
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgt_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcgt_u64(a, b) }
}

/// Compare unsigned highe
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcgtq_u64(a, b) }
}

/// Floating-point compare greater than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgt_f64(a: float64x1_t, b: float64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcgt_f64(a, b) }
}

/// Floating-point compare greater than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtq_f64(a: float64x2_t, b: float64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcgtq_f64(a, b) }
}

/// Compare greater than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtd_s64(a: i64, b: i64) -> u64 {
  unsafe { core::arch::aarch64::vcgtd_s64(a, b) }
}

/// Compare greater than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtd_u64(a: u64, b: u64) -> u64 {
  unsafe { core::arch::aarch64::vcgtd_u64(a, b) }
}

/// Floating-point compare greater than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgts_f32(a: f32, b: f32) -> u32 {
  unsafe { core::arch::aarch64::vcgts_f32(a, b) }
}

/// Floating-point compare greater than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtd_f64(a: f64, b: f64) -> u64 {
  unsafe { core::arch::aarch64::vcgtd_f64(a, b) }
}

/// Compare signed less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclt_s64(a: int64x1_t, b: int64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vclt_s64(a, b) }
}

/// Compare signed less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltq_s64(a: int64x2_t, b: int64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcltq_s64(a, b) }
}

/// Compare unsigned less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclt_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vclt_u64(a, b) }
}

/// Compare unsigned less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcltq_u64(a, b) }
}

/// Floating-point compare less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclt_f64(a: float64x1_t, b: float64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vclt_f64(a, b) }
}

/// Floating-point compare less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltq_f64(a: float64x2_t, b: float64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcltq_f64(a, b) }
}

/// Compare less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltd_s64(a: i64, b: i64) -> u64 {
  unsafe { core::arch::aarch64::vcltd_s64(a, b) }
}

/// Compare less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltd_u64(a: u64, b: u64) -> u64 {
  unsafe { core::arch::aarch64::vcltd_u64(a, b) }
}

/// Floating-point compare less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclts_f32(a: f32, b: f32) -> u32 {
  unsafe { core::arch::aarch64::vclts_f32(a, b) }
}

/// Floating-point compare less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltd_f64(a: f64, b: f64) -> u64 {
  unsafe { core::arch::aarch64::vcltd_f64(a, b) }
}

/// Compare signed less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcle_s64(a: int64x1_t, b: int64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcle_s64(a, b) }
}

/// Compare signed less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcleq_s64(a: int64x2_t, b: int64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcleq_s64(a, b) }
}

/// Compare greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcged_s64(a: i64, b: i64) -> u64 {
  unsafe { core::arch::aarch64::vcged_s64(a, b) }
}

/// Compare greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcged_u64(a: u64, b: u64) -> u64 {
  unsafe { core::arch::aarch64::vcged_u64(a, b) }
}

/// Floating-point compare greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcges_f32(a: f32, b: f32) -> u32 {
  unsafe { core::arch::aarch64::vcges_f32(a, b) }
}

/// Floating-point compare greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcged_f64(a: f64, b: f64) -> u64 {
  unsafe { core::arch::aarch64::vcged_f64(a, b) }
}

/// Compare unsigned less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcle_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcle_u64(a, b) }
}

/// Compare unsigned less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcleq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcleq_u64(a, b) }
}

/// Floating-point compare less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcle_f64(a: float64x1_t, b: float64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcle_f64(a, b) }
}

/// Floating-point compare less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcleq_f64(a: float64x2_t, b: float64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcleq_f64(a, b) }
}

/// Compare less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcled_s64(a: i64, b: i64) -> u64 {
  unsafe { core::arch::aarch64::vcled_s64(a, b) }
}

/// Compare less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcled_u64(a: u64, b: u64) -> u64 {
  unsafe { core::arch::aarch64::vcled_u64(a, b) }
}

/// Floating-point compare less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcles_f32(a: f32, b: f32) -> u32 {
  unsafe { core::arch::aarch64::vcles_f32(a, b) }
}

/// Floating-point compare less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcled_f64(a: f64, b: f64) -> u64 {
  unsafe { core::arch::aarch64::vcled_f64(a, b) }
}

/// Compare signed greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcge_s64(a: int64x1_t, b: int64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcge_s64(a, b) }
}

/// Compare signed greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgeq_s64(a: int64x2_t, b: int64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcgeq_s64(a, b) }
}

/// Compare unsigned greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcge_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcge_u64(a, b) }
}

/// Compare unsigned greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgeq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcgeq_u64(a, b) }
}

/// Floating-point compare greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcge_f64(a: float64x1_t, b: float64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcge_f64(a, b) }
}

/// Floating-point compare greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgeq_f64(a: float64x2_t, b: float64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcgeq_f64(a, b) }
}

/// Compare signed greater than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgez_s8(a: int8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vcgez_s8(a) }
}

/// Compare signed greater than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgezq_s8(a: int8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vcgezq_s8(a) }
}

/// Compare signed greater than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgez_s16(a: int16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vcgez_s16(a) }
}

/// Compare signed greater than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgezq_s16(a: int16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vcgezq_s16(a) }
}

/// Compare signed greater than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgez_s32(a: int32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcgez_s32(a) }
}

/// Compare signed greater than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgezq_s32(a: int32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcgezq_s32(a) }
}

/// Compare signed greater than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgez_s64(a: int64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcgez_s64(a) }
}

/// Compare signed greater than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgezq_s64(a: int64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcgezq_s64(a) }
}

/// Floating-point compare greater than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgez_f32(a: float32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcgez_f32(a) }
}

/// Floating-point compare greater than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgezq_f32(a: float32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcgezq_f32(a) }
}

/// Floating-point compare greater than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgez_f64(a: float64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcgez_f64(a) }
}

/// Floating-point compare greater than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgezq_f64(a: float64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcgezq_f64(a) }
}

/// Compare signed greater than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgezd_s64(a: i64) -> u64 {
  unsafe { core::arch::aarch64::vcgezd_s64(a) }
}

/// Floating-point compare greater than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgezs_f32(a: f32) -> u32 {
  unsafe { core::arch::aarch64::vcgezs_f32(a) }
}

/// Floating-point compare greater than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgezd_f64(a: f64) -> u64 {
  unsafe { core::arch::aarch64::vcgezd_f64(a) }
}

/// Compare signed greater than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtz_s8(a: int8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vcgtz_s8(a) }
}

/// Compare signed greater than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtzq_s8(a: int8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vcgtzq_s8(a) }
}

/// Compare signed greater than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtz_s16(a: int16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vcgtz_s16(a) }
}

/// Compare signed greater than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtzq_s16(a: int16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vcgtzq_s16(a) }
}

/// Compare signed greater than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtz_s32(a: int32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcgtz_s32(a) }
}

/// Compare signed greater than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtzq_s32(a: int32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcgtzq_s32(a) }
}

/// Compare signed greater than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtz_s64(a: int64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcgtz_s64(a) }
}

/// Compare signed greater than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtzq_s64(a: int64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcgtzq_s64(a) }
}

/// Floating-point compare greater than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtz_f32(a: float32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcgtz_f32(a) }
}

/// Floating-point compare greater than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtzq_f32(a: float32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcgtzq_f32(a) }
}

/// Floating-point compare greater than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtz_f64(a: float64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcgtz_f64(a) }
}

/// Floating-point compare greater than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtzq_f64(a: float64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcgtzq_f64(a) }
}

/// Compare signed greater than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtzd_s64(a: i64) -> u64 {
  unsafe { core::arch::aarch64::vcgtzd_s64(a) }
}

/// Floating-point compare greater than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtzs_f32(a: f32) -> u32 {
  unsafe { core::arch::aarch64::vcgtzs_f32(a) }
}

/// Floating-point compare greater than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcgtzd_f64(a: f64) -> u64 {
  unsafe { core::arch::aarch64::vcgtzd_f64(a) }
}

/// Compare signed less than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclez_s8(a: int8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vclez_s8(a) }
}

/// Compare signed less than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclezq_s8(a: int8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vclezq_s8(a) }
}

/// Compare signed less than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclez_s16(a: int16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vclez_s16(a) }
}

/// Compare signed less than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclezq_s16(a: int16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vclezq_s16(a) }
}

/// Compare signed less than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclez_s32(a: int32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vclez_s32(a) }
}

/// Compare signed less than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclezq_s32(a: int32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vclezq_s32(a) }
}

/// Compare signed less than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclez_s64(a: int64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vclez_s64(a) }
}

/// Compare signed less than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclezq_s64(a: int64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vclezq_s64(a) }
}

/// Floating-point compare less than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclez_f32(a: float32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vclez_f32(a) }
}

/// Floating-point compare less than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclezq_f32(a: float32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vclezq_f32(a) }
}

/// Floating-point compare less than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclez_f64(a: float64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vclez_f64(a) }
}

/// Floating-point compare less than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclezq_f64(a: float64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vclezq_f64(a) }
}

/// Compare less than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclezd_s64(a: i64) -> u64 {
  unsafe { core::arch::aarch64::vclezd_s64(a) }
}

/// Floating-point compare less than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclezs_f32(a: f32) -> u32 {
  unsafe { core::arch::aarch64::vclezs_f32(a) }
}

/// Floating-point compare less than or equal to zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vclezd_f64(a: f64) -> u64 {
  unsafe { core::arch::aarch64::vclezd_f64(a) }
}

/// Compare signed less than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltz_s8(a: int8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vcltz_s8(a) }
}

/// Compare signed less than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltzq_s8(a: int8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vcltzq_s8(a) }
}

/// Compare signed less than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltz_s16(a: int16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vcltz_s16(a) }
}

/// Compare signed less than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltzq_s16(a: int16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vcltzq_s16(a) }
}

/// Compare signed less than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltz_s32(a: int32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcltz_s32(a) }
}

/// Compare signed less than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltzq_s32(a: int32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcltzq_s32(a) }
}

/// Compare signed less than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltz_s64(a: int64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcltz_s64(a) }
}

/// Compare signed less than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltzq_s64(a: int64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcltzq_s64(a) }
}

/// Floating-point compare less than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltz_f32(a: float32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcltz_f32(a) }
}

/// Floating-point compare less than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltzq_f32(a: float32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcltzq_f32(a) }
}

/// Floating-point compare less than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltz_f64(a: float64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcltz_f64(a) }
}

/// Floating-point compare less than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltzq_f64(a: float64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcltzq_f64(a) }
}

/// Compare less than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltzd_s64(a: i64) -> u64 {
  unsafe { core::arch::aarch64::vcltzd_s64(a) }
}

/// Floating-point compare less than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltzs_f32(a: f32) -> u32 {
  unsafe { core::arch::aarch64::vcltzs_f32(a) }
}

/// Floating-point compare less than zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcltzd_f64(a: f64) -> u64 {
  unsafe { core::arch::aarch64::vcltzd_f64(a) }
}

/// Floating-point absolute compare greater than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcagt_f64(a: float64x1_t, b: float64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcagt_f64(a, b) }
}

/// Floating-point absolute compare greater than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcagtq_f64(a: float64x2_t, b: float64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcagtq_f64(a, b) }
}

/// Floating-point absolute compare greater than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcagts_f32(a: f32, b: f32) -> u32 {
  unsafe { core::arch::aarch64::vcagts_f32(a, b) }
}

/// Floating-point absolute compare greater than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcagtd_f64(a: f64, b: f64) -> u64 {
  unsafe { core::arch::aarch64::vcagtd_f64(a, b) }
}

/// Floating-point absolute compare greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcage_f64(a: float64x1_t, b: float64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcage_f64(a, b) }
}

/// Floating-point absolute compare greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcageq_f64(a: float64x2_t, b: float64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcageq_f64(a, b) }
}

/// Floating-point absolute compare greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcages_f32(a: f32, b: f32) -> u32 {
  unsafe { core::arch::aarch64::vcages_f32(a, b) }
}

/// Floating-point absolute compare greater than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcaged_f64(a: f64, b: f64) -> u64 {
  unsafe { core::arch::aarch64::vcaged_f64(a, b) }
}

/// Floating-point absolute compare less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcalt_f64(a: float64x1_t, b: float64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcalt_f64(a, b) }
}

/// Floating-point absolute compare less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcaltq_f64(a: float64x2_t, b: float64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcaltq_f64(a, b) }
}

/// Floating-point absolute compare less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcalts_f32(a: f32, b: f32) -> u32 {
  unsafe { core::arch::aarch64::vcalts_f32(a, b) }
}

/// Floating-point absolute compare less than
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcaltd_f64(a: f64, b: f64) -> u64 {
  unsafe { core::arch::aarch64::vcaltd_f64(a, b) }
}

/// Floating-point absolute compare less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcale_f64(a: float64x1_t, b: float64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcale_f64(a, b) }
}

/// Floating-point absolute compare less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcaleq_f64(a: float64x2_t, b: float64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcaleq_f64(a, b) }
}

/// Floating-point absolute compare less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcales_f32(a: f32, b: f32) -> u32 {
  unsafe { core::arch::aarch64::vcales_f32(a, b) }
}

/// Floating-point absolute compare less than or equal
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcaled_f64(a: f64, b: f64) -> u64 {
  unsafe { core::arch::aarch64::vcaled_f64(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_lane_s8<const LANE1: i32, const LANE2: i32>(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vcopy_lane_s8::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_laneq_s8<const LANE1: i32, const LANE2: i32>(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vcopyq_laneq_s8::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_lane_s16<const LANE1: i32, const LANE2: i32>(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vcopy_lane_s16::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_laneq_s16<const LANE1: i32, const LANE2: i32>(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vcopyq_laneq_s16::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_lane_s32<const LANE1: i32, const LANE2: i32>(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vcopy_lane_s32::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_laneq_s32<const LANE1: i32, const LANE2: i32>(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vcopyq_laneq_s32::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_laneq_s64<const LANE1: i32, const LANE2: i32>(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vcopyq_laneq_s64::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_lane_u8<const LANE1: i32, const LANE2: i32>(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vcopy_lane_u8::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_laneq_u8<const LANE1: i32, const LANE2: i32>(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vcopyq_laneq_u8::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_lane_u16<const LANE1: i32, const LANE2: i32>(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vcopy_lane_u16::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_laneq_u16<const LANE1: i32, const LANE2: i32>(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vcopyq_laneq_u16::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_lane_u32<const LANE1: i32, const LANE2: i32>(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcopy_lane_u32::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_laneq_u32<const LANE1: i32, const LANE2: i32>(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcopyq_laneq_u32::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_laneq_u64<const LANE1: i32, const LANE2: i32>(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcopyq_laneq_u64::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_lane_p8<const LANE1: i32, const LANE2: i32>(a: poly8x8_t, b: poly8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vcopy_lane_p8::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_laneq_p8<const LANE1: i32, const LANE2: i32>(a: poly8x16_t, b: poly8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vcopyq_laneq_p8::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_lane_p16<const LANE1: i32, const LANE2: i32>(a: poly16x4_t, b: poly16x4_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vcopy_lane_p16::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_laneq_p16<const LANE1: i32, const LANE2: i32>(a: poly16x8_t, b: poly16x8_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vcopyq_laneq_p16::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_laneq_p64<const LANE1: i32, const LANE2: i32>(a: poly64x2_t, b: poly64x2_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vcopyq_laneq_p64::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_lane_f32<const LANE1: i32, const LANE2: i32>(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vcopy_lane_f32::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_laneq_f32<const LANE1: i32, const LANE2: i32>(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vcopyq_laneq_f32::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_laneq_f64<const LANE1: i32, const LANE2: i32>(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vcopyq_laneq_f64::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_laneq_s8<const LANE1: i32, const LANE2: i32>(a: int8x8_t, b: int8x16_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vcopy_laneq_s8::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_laneq_s16<const LANE1: i32, const LANE2: i32>(a: int16x4_t, b: int16x8_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vcopy_laneq_s16::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_laneq_s32<const LANE1: i32, const LANE2: i32>(a: int32x2_t, b: int32x4_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vcopy_laneq_s32::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_laneq_u8<const LANE1: i32, const LANE2: i32>(a: uint8x8_t, b: uint8x16_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vcopy_laneq_u8::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_laneq_u16<const LANE1: i32, const LANE2: i32>(a: uint16x4_t, b: uint16x8_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vcopy_laneq_u16::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_laneq_u32<const LANE1: i32, const LANE2: i32>(a: uint32x2_t, b: uint32x4_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcopy_laneq_u32::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_laneq_p8<const LANE1: i32, const LANE2: i32>(a: poly8x8_t, b: poly8x16_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vcopy_laneq_p8::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_laneq_p16<const LANE1: i32, const LANE2: i32>(a: poly16x4_t, b: poly16x8_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vcopy_laneq_p16::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_laneq_f32<const LANE1: i32, const LANE2: i32>(a: float32x2_t, b: float32x4_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vcopy_laneq_f32::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_lane_s8<const LANE1: i32, const LANE2: i32>(a: int8x16_t, b: int8x8_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vcopyq_lane_s8::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_lane_s16<const LANE1: i32, const LANE2: i32>(a: int16x8_t, b: int16x4_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vcopyq_lane_s16::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_lane_s32<const LANE1: i32, const LANE2: i32>(a: int32x4_t, b: int32x2_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vcopyq_lane_s32::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_lane_u8<const LANE1: i32, const LANE2: i32>(a: uint8x16_t, b: uint8x8_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vcopyq_lane_u8::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_lane_u16<const LANE1: i32, const LANE2: i32>(a: uint16x8_t, b: uint16x4_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vcopyq_lane_u16::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_lane_u32<const LANE1: i32, const LANE2: i32>(a: uint32x4_t, b: uint32x2_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcopyq_lane_u32::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_lane_p8<const LANE1: i32, const LANE2: i32>(a: poly8x16_t, b: poly8x8_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vcopyq_lane_p8::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_lane_p16<const LANE1: i32, const LANE2: i32>(a: poly16x8_t, b: poly16x4_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vcopyq_lane_p16::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_lane_s64<const LANE1: i32, const LANE2: i32>(a: int64x2_t, b: int64x1_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vcopyq_lane_s64::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_lane_u64<const LANE1: i32, const LANE2: i32>(a: uint64x2_t, b: uint64x1_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcopyq_lane_u64::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_lane_p64<const LANE1: i32, const LANE2: i32>(a: poly64x2_t, b: poly64x1_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vcopyq_lane_p64::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_lane_f32<const LANE1: i32, const LANE2: i32>(a: float32x4_t, b: float32x2_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vcopyq_lane_f32::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopyq_lane_f64<const LANE1: i32, const LANE2: i32>(a: float64x2_t, b: float64x1_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vcopyq_lane_f64::<LANE1, LANE2>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcreate_f64(a: u64) -> float64x1_t {
  unsafe { core::arch::aarch64::vcreate_f64(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvt_f64_s64(a: int64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vcvt_f64_s64(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtq_f64_s64(a: int64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vcvtq_f64_s64(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvt_f64_u64(a: uint64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vcvt_f64_u64(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtq_f64_u64(a: uint64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vcvtq_f64_u64(a) }
}

/// Floating-point convert to higher precision long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvt_f64_f32(a: float32x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vcvt_f64_f32(a) }
}

/// Floating-point convert to higher precision long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvt_high_f64_f32(a: float32x4_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vcvt_high_f64_f32(a) }
}

/// Floating-point convert to lower precision narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvt_f32_f64(a: float64x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vcvt_f32_f64(a) }
}

/// Floating-point convert to lower precision narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvt_high_f32_f64(a: float32x2_t, b: float64x2_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vcvt_high_f32_f64(a, b) }
}

/// Floating-point convert to lower precision narrow, rounding to odd
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtx_f32_f64(a: float64x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vcvtx_f32_f64(a) }
}

/// Floating-point convert to lower precision narrow, rounding to odd
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtxd_f32_f64(a: f64) -> f32 {
  unsafe { core::arch::aarch64::vcvtxd_f32_f64(a) }
}

/// Floating-point convert to lower precision narrow, rounding to odd
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtx_high_f32_f64(a: float32x2_t, b: float64x2_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vcvtx_high_f32_f64(a, b) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvt_n_f64_s64<const N: i32>(a: int64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vcvt_n_f64_s64::<N>(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtq_n_f64_s64<const N: i32>(a: int64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vcvtq_n_f64_s64::<N>(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvts_n_f32_s32<const N: i32>(a: i32) -> f32 {
  unsafe { core::arch::aarch64::vcvts_n_f32_s32::<N>(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtd_n_f64_s64<const N: i32>(a: i64) -> f64 {
  unsafe { core::arch::aarch64::vcvtd_n_f64_s64::<N>(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvt_n_f64_u64<const N: i32>(a: uint64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vcvt_n_f64_u64::<N>(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtq_n_f64_u64<const N: i32>(a: uint64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vcvtq_n_f64_u64::<N>(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvts_n_f32_u32<const N: i32>(a: u32) -> f32 {
  unsafe { core::arch::aarch64::vcvts_n_f32_u32::<N>(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtd_n_f64_u64<const N: i32>(a: u64) -> f64 {
  unsafe { core::arch::aarch64::vcvtd_n_f64_u64::<N>(a) }
}

/// Floating-point convert to fixed-point, rounding toward zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvt_n_s64_f64<const N: i32>(a: float64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vcvt_n_s64_f64::<N>(a) }
}

/// Floating-point convert to fixed-point, rounding toward zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtq_n_s64_f64<const N: i32>(a: float64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vcvtq_n_s64_f64::<N>(a) }
}

/// Floating-point convert to fixed-point, rounding toward zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvts_n_s32_f32<const N: i32>(a: f32) -> i32 {
  unsafe { core::arch::aarch64::vcvts_n_s32_f32::<N>(a) }
}

/// Floating-point convert to fixed-point, rounding toward zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtd_n_s64_f64<const N: i32>(a: f64) -> i64 {
  unsafe { core::arch::aarch64::vcvtd_n_s64_f64::<N>(a) }
}

/// Floating-point convert to fixed-point, rounding toward zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvt_n_u64_f64<const N: i32>(a: float64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcvt_n_u64_f64::<N>(a) }
}

/// Floating-point convert to fixed-point, rounding toward zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtq_n_u64_f64<const N: i32>(a: float64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcvtq_n_u64_f64::<N>(a) }
}

/// Floating-point convert to fixed-point, rounding toward zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvts_n_u32_f32<const N: i32>(a: f32) -> u32 {
  unsafe { core::arch::aarch64::vcvts_n_u32_f32::<N>(a) }
}

/// Floating-point convert to fixed-point, rounding toward zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtd_n_u64_f64<const N: i32>(a: f64) -> u64 {
  unsafe { core::arch::aarch64::vcvtd_n_u64_f64::<N>(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvts_f32_s32(a: i32) -> f32 {
  unsafe { core::arch::aarch64::vcvts_f32_s32(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtd_f64_s64(a: i64) -> f64 {
  unsafe { core::arch::aarch64::vcvtd_f64_s64(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvts_f32_u32(a: u32) -> f32 {
  unsafe { core::arch::aarch64::vcvts_f32_u32(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtd_f64_u64(a: u64) -> f64 {
  unsafe { core::arch::aarch64::vcvtd_f64_u64(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvts_s32_f32(a: f32) -> i32 {
  unsafe { core::arch::aarch64::vcvts_s32_f32(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtd_s64_f64(a: f64) -> i64 {
  unsafe { core::arch::aarch64::vcvtd_s64_f64(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvts_u32_f32(a: f32) -> u32 {
  unsafe { core::arch::aarch64::vcvts_u32_f32(a) }
}

/// Fixed-point convert to floating-point
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtd_u64_f64(a: f64) -> u64 {
  unsafe { core::arch::aarch64::vcvtd_u64_f64(a) }
}

/// Floating-point convert to signed fixed-point, rounding toward zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvt_s64_f64(a: float64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vcvt_s64_f64(a) }
}

/// Floating-point convert to signed fixed-point, rounding toward zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtq_s64_f64(a: float64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vcvtq_s64_f64(a) }
}

/// Floating-point convert to unsigned fixed-point, rounding toward zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvt_u64_f64(a: float64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcvt_u64_f64(a) }
}

/// Floating-point convert to unsigned fixed-point, rounding toward zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtq_u64_f64(a: float64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcvtq_u64_f64(a) }
}

/// Floating-point convert to signed integer, rounding to nearest with ties to
/// away
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvta_s32_f32(a: float32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vcvta_s32_f32(a) }
}

/// Floating-point convert to signed integer, rounding to nearest with ties to
/// away
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtaq_s32_f32(a: float32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vcvtaq_s32_f32(a) }
}

/// Floating-point convert to signed integer, rounding to nearest with ties to
/// away
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvta_s64_f64(a: float64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vcvta_s64_f64(a) }
}

/// Floating-point convert to signed integer, rounding to nearest with ties to
/// away
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtaq_s64_f64(a: float64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vcvtaq_s64_f64(a) }
}

/// Floating-point convert to integer, rounding to nearest with ties to away
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtas_s32_f32(a: f32) -> i32 {
  unsafe { core::arch::aarch64::vcvtas_s32_f32(a) }
}

/// Floating-point convert to integer, rounding to nearest with ties to away
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtad_s64_f64(a: f64) -> i64 {
  unsafe { core::arch::aarch64::vcvtad_s64_f64(a) }
}

/// Floating-point convert to integer, rounding to nearest with ties to away
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtas_u32_f32(a: f32) -> u32 {
  unsafe { core::arch::aarch64::vcvtas_u32_f32(a) }
}

/// Floating-point convert to integer, rounding to nearest with ties to away
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtad_u64_f64(a: f64) -> u64 {
  unsafe { core::arch::aarch64::vcvtad_u64_f64(a) }
}

/// Floating-point convert to signed integer, rounding to nearest with ties to
/// even
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtn_s32_f32(a: float32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vcvtn_s32_f32(a) }
}

/// Floating-point convert to signed integer, rounding to nearest with ties to
/// even
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtnq_s32_f32(a: float32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vcvtnq_s32_f32(a) }
}

/// Floating-point convert to signed integer, rounding to nearest with ties to
/// even
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtn_s64_f64(a: float64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vcvtn_s64_f64(a) }
}

/// Floating-point convert to signed integer, rounding to nearest with ties to
/// even
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtnq_s64_f64(a: float64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vcvtnq_s64_f64(a) }
}

/// Floating-point convert to signed integer, rounding to nearest with ties to
/// even
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtns_s32_f32(a: f32) -> i32 {
  unsafe { core::arch::aarch64::vcvtns_s32_f32(a) }
}

/// Floating-point convert to signed integer, rounding to nearest with ties to
/// even
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtnd_s64_f64(a: f64) -> i64 {
  unsafe { core::arch::aarch64::vcvtnd_s64_f64(a) }
}

/// Floating-point convert to signed integer, rounding toward minus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtm_s32_f32(a: float32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vcvtm_s32_f32(a) }
}

/// Floating-point convert to signed integer, rounding toward minus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtmq_s32_f32(a: float32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vcvtmq_s32_f32(a) }
}

/// Floating-point convert to signed integer, rounding toward minus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtm_s64_f64(a: float64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vcvtm_s64_f64(a) }
}

/// Floating-point convert to signed integer, rounding toward minus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtmq_s64_f64(a: float64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vcvtmq_s64_f64(a) }
}

/// Floating-point convert to signed integer, rounding toward minus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtms_s32_f32(a: f32) -> i32 {
  unsafe { core::arch::aarch64::vcvtms_s32_f32(a) }
}

/// Floating-point convert to signed integer, rounding toward minus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtmd_s64_f64(a: f64) -> i64 {
  unsafe { core::arch::aarch64::vcvtmd_s64_f64(a) }
}

/// Floating-point convert to signed integer, rounding toward plus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtp_s32_f32(a: float32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vcvtp_s32_f32(a) }
}

/// Floating-point convert to signed integer, rounding toward plus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtpq_s32_f32(a: float32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vcvtpq_s32_f32(a) }
}

/// Floating-point convert to signed integer, rounding toward plus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtp_s64_f64(a: float64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vcvtp_s64_f64(a) }
}

/// Floating-point convert to signed integer, rounding toward plus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtpq_s64_f64(a: float64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vcvtpq_s64_f64(a) }
}

/// Floating-point convert to signed integer, rounding toward plus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtps_s32_f32(a: f32) -> i32 {
  unsafe { core::arch::aarch64::vcvtps_s32_f32(a) }
}

/// Floating-point convert to signed integer, rounding toward plus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtpd_s64_f64(a: f64) -> i64 {
  unsafe { core::arch::aarch64::vcvtpd_s64_f64(a) }
}

/// Floating-point convert to unsigned integer, rounding to nearest with ties to
/// away
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvta_u32_f32(a: float32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcvta_u32_f32(a) }
}

/// Floating-point convert to unsigned integer, rounding to nearest with ties to
/// away
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtaq_u32_f32(a: float32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcvtaq_u32_f32(a) }
}

/// Floating-point convert to unsigned integer, rounding to nearest with ties to
/// away
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvta_u64_f64(a: float64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcvta_u64_f64(a) }
}

/// Floating-point convert to unsigned integer, rounding to nearest with ties to
/// away
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtaq_u64_f64(a: float64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcvtaq_u64_f64(a) }
}

/// Floating-point convert to unsigned integer, rounding to nearest with ties to
/// even
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtn_u32_f32(a: float32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcvtn_u32_f32(a) }
}

/// Floating-point convert to unsigned integer, rounding to nearest with ties to
/// even
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtnq_u32_f32(a: float32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcvtnq_u32_f32(a) }
}

/// Floating-point convert to unsigned integer, rounding to nearest with ties to
/// even
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtn_u64_f64(a: float64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcvtn_u64_f64(a) }
}

/// Floating-point convert to unsigned integer, rounding to nearest with ties to
/// even
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtnq_u64_f64(a: float64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcvtnq_u64_f64(a) }
}

/// Floating-point convert to unsigned integer, rounding to nearest with ties to
/// even
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtns_u32_f32(a: f32) -> u32 {
  unsafe { core::arch::aarch64::vcvtns_u32_f32(a) }
}

/// Floating-point convert to unsigned integer, rounding to nearest with ties to
/// even
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtnd_u64_f64(a: f64) -> u64 {
  unsafe { core::arch::aarch64::vcvtnd_u64_f64(a) }
}

/// Floating-point convert to unsigned integer, rounding toward minus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtm_u32_f32(a: float32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcvtm_u32_f32(a) }
}

/// Floating-point convert to unsigned integer, rounding toward minus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtmq_u32_f32(a: float32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcvtmq_u32_f32(a) }
}

/// Floating-point convert to unsigned integer, rounding toward minus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtm_u64_f64(a: float64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcvtm_u64_f64(a) }
}

/// Floating-point convert to unsigned integer, rounding toward minus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtmq_u64_f64(a: float64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcvtmq_u64_f64(a) }
}

/// Floating-point convert to unsigned integer, rounding toward minus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtms_u32_f32(a: f32) -> u32 {
  unsafe { core::arch::aarch64::vcvtms_u32_f32(a) }
}

/// Floating-point convert to unsigned integer, rounding toward minus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtmd_u64_f64(a: f64) -> u64 {
  unsafe { core::arch::aarch64::vcvtmd_u64_f64(a) }
}

/// Floating-point convert to unsigned integer, rounding toward plus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtp_u32_f32(a: float32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vcvtp_u32_f32(a) }
}

/// Floating-point convert to unsigned integer, rounding toward plus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtpq_u32_f32(a: float32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcvtpq_u32_f32(a) }
}

/// Floating-point convert to unsigned integer, rounding toward plus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtp_u64_f64(a: float64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcvtp_u64_f64(a) }
}

/// Floating-point convert to unsigned integer, rounding toward plus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtpq_u64_f64(a: float64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcvtpq_u64_f64(a) }
}

/// Floating-point convert to unsigned integer, rounding toward plus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtps_u32_f32(a: f32) -> u32 {
  unsafe { core::arch::aarch64::vcvtps_u32_f32(a) }
}

/// Floating-point convert to unsigned integer, rounding toward plus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcvtpd_u64_f64(a: f64) -> u64 {
  unsafe { core::arch::aarch64::vcvtpd_u64_f64(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_laneq_p64<const N: i32>(a: poly64x2_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vdupq_laneq_p64::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_lane_p64<const N: i32>(a: poly64x1_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vdupq_lane_p64::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_laneq_f64<const N: i32>(a: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vdupq_laneq_f64::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_lane_f64<const N: i32>(a: float64x1_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vdupq_lane_f64::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_lane_p64<const N: i32>(a: poly64x1_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vdup_lane_p64::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_lane_f64<const N: i32>(a: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vdup_lane_f64::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_laneq_p64<const N: i32>(a: poly64x2_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vdup_laneq_p64::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_laneq_f64<const N: i32>(a: float64x2_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vdup_laneq_f64::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupb_lane_s8<const N: i32>(a: int8x8_t) -> i8 {
  unsafe { core::arch::aarch64::vdupb_lane_s8::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupb_laneq_s8<const N: i32>(a: int8x16_t) -> i8 {
  unsafe { core::arch::aarch64::vdupb_laneq_s8::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vduph_lane_s16<const N: i32>(a: int16x4_t) -> i16 {
  unsafe { core::arch::aarch64::vduph_lane_s16::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vduph_laneq_s16<const N: i32>(a: int16x8_t) -> i16 {
  unsafe { core::arch::aarch64::vduph_laneq_s16::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdups_lane_s32<const N: i32>(a: int32x2_t) -> i32 {
  unsafe { core::arch::aarch64::vdups_lane_s32::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdups_laneq_s32<const N: i32>(a: int32x4_t) -> i32 {
  unsafe { core::arch::aarch64::vdups_laneq_s32::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupd_lane_s64<const N: i32>(a: int64x1_t) -> i64 {
  unsafe { core::arch::aarch64::vdupd_lane_s64::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupd_laneq_s64<const N: i32>(a: int64x2_t) -> i64 {
  unsafe { core::arch::aarch64::vdupd_laneq_s64::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupb_lane_u8<const N: i32>(a: uint8x8_t) -> u8 {
  unsafe { core::arch::aarch64::vdupb_lane_u8::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupb_laneq_u8<const N: i32>(a: uint8x16_t) -> u8 {
  unsafe { core::arch::aarch64::vdupb_laneq_u8::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vduph_lane_u16<const N: i32>(a: uint16x4_t) -> u16 {
  unsafe { core::arch::aarch64::vduph_lane_u16::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vduph_laneq_u16<const N: i32>(a: uint16x8_t) -> u16 {
  unsafe { core::arch::aarch64::vduph_laneq_u16::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdups_lane_u32<const N: i32>(a: uint32x2_t) -> u32 {
  unsafe { core::arch::aarch64::vdups_lane_u32::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdups_laneq_u32<const N: i32>(a: uint32x4_t) -> u32 {
  unsafe { core::arch::aarch64::vdups_laneq_u32::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupd_lane_u64<const N: i32>(a: uint64x1_t) -> u64 {
  unsafe { core::arch::aarch64::vdupd_lane_u64::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupd_laneq_u64<const N: i32>(a: uint64x2_t) -> u64 {
  unsafe { core::arch::aarch64::vdupd_laneq_u64::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupb_lane_p8<const N: i32>(a: poly8x8_t) -> p8 {
  unsafe { core::arch::aarch64::vdupb_lane_p8::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupb_laneq_p8<const N: i32>(a: poly8x16_t) -> p8 {
  unsafe { core::arch::aarch64::vdupb_laneq_p8::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vduph_lane_p16<const N: i32>(a: poly16x4_t) -> p16 {
  unsafe { core::arch::aarch64::vduph_lane_p16::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vduph_laneq_p16<const N: i32>(a: poly16x8_t) -> p16 {
  unsafe { core::arch::aarch64::vduph_laneq_p16::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdups_lane_f32<const N: i32>(a: float32x2_t) -> f32 {
  unsafe { core::arch::aarch64::vdups_lane_f32::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdups_laneq_f32<const N: i32>(a: float32x4_t) -> f32 {
  unsafe { core::arch::aarch64::vdups_laneq_f32::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupd_lane_f64<const N: i32>(a: float64x1_t) -> f64 {
  unsafe { core::arch::aarch64::vdupd_lane_f64::<N>(a) }
}

/// Set all vector lanes to the same value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupd_laneq_f64<const N: i32>(a: float64x2_t) -> f64 {
  unsafe { core::arch::aarch64::vdupd_laneq_f64::<N>(a) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vextq_p64<const N: i32>(a: poly64x2_t, b: poly64x2_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vextq_p64::<N>(a, b) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vextq_f64<const N: i32>(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vextq_f64::<N>(a, b) }
}

/// Floating-point multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmla_f64(a: float64x1_t, b: float64x1_t, c: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vmla_f64(a, b, c) }
}

/// Floating-point multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlaq_f64(a: float64x2_t, b: float64x2_t, c: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vmlaq_f64(a, b, c) }
}

/// Signed multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_high_s8(a: int16x8_t, b: int8x16_t, c: int8x16_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vmlal_high_s8(a, b, c) }
}

/// Signed multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_high_s16(a: int32x4_t, b: int16x8_t, c: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmlal_high_s16(a, b, c) }
}

/// Signed multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_high_s32(a: int64x2_t, b: int32x4_t, c: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vmlal_high_s32(a, b, c) }
}

/// Unsigned multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_high_u8(a: uint16x8_t, b: uint8x16_t, c: uint8x16_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vmlal_high_u8(a, b, c) }
}

/// Unsigned multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_high_u16(a: uint32x4_t, b: uint16x8_t, c: uint16x8_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmlal_high_u16(a, b, c) }
}

/// Unsigned multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_high_u32(a: uint64x2_t, b: uint32x4_t, c: uint32x4_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmlal_high_u32(a, b, c) }
}

/// Multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_high_n_s16(a: int32x4_t, b: int16x8_t, c: i16) -> int32x4_t {
  unsafe { core::arch::aarch64::vmlal_high_n_s16(a, b, c) }
}

/// Multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_high_n_s32(a: int64x2_t, b: int32x4_t, c: i32) -> int64x2_t {
  unsafe { core::arch::aarch64::vmlal_high_n_s32(a, b, c) }
}

/// Multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_high_n_u16(a: uint32x4_t, b: uint16x8_t, c: u16) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmlal_high_n_u16(a, b, c) }
}

/// Multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_high_n_u32(a: uint64x2_t, b: uint32x4_t, c: u32) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmlal_high_n_u32(a, b, c) }
}

/// Multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_high_lane_s16<const LANE: i32>(a: int32x4_t, b: int16x8_t, c: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmlal_high_lane_s16::<LANE>(a, b, c) }
}

/// Multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_high_laneq_s16<const LANE: i32>(a: int32x4_t, b: int16x8_t, c: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmlal_high_laneq_s16::<LANE>(a, b, c) }
}

/// Multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_high_lane_s32<const LANE: i32>(a: int64x2_t, b: int32x4_t, c: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vmlal_high_lane_s32::<LANE>(a, b, c) }
}

/// Multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_high_laneq_s32<const LANE: i32>(a: int64x2_t, b: int32x4_t, c: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vmlal_high_laneq_s32::<LANE>(a, b, c) }
}

/// Multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_high_lane_u16<const LANE: i32>(a: uint32x4_t, b: uint16x8_t, c: uint16x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmlal_high_lane_u16::<LANE>(a, b, c) }
}

/// Multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_high_laneq_u16<const LANE: i32>(a: uint32x4_t, b: uint16x8_t, c: uint16x8_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmlal_high_laneq_u16::<LANE>(a, b, c) }
}

/// Multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_high_lane_u32<const LANE: i32>(a: uint64x2_t, b: uint32x4_t, c: uint32x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmlal_high_lane_u32::<LANE>(a, b, c) }
}

/// Multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlal_high_laneq_u32<const LANE: i32>(a: uint64x2_t, b: uint32x4_t, c: uint32x4_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmlal_high_laneq_u32::<LANE>(a, b, c) }
}

/// Floating-point multiply-subtract from accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmls_f64(a: float64x1_t, b: float64x1_t, c: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vmls_f64(a, b, c) }
}

/// Floating-point multiply-subtract from accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsq_f64(a: float64x2_t, b: float64x2_t, c: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vmlsq_f64(a, b, c) }
}

/// Signed multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_high_s8(a: int16x8_t, b: int8x16_t, c: int8x16_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vmlsl_high_s8(a, b, c) }
}

/// Signed multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_high_s16(a: int32x4_t, b: int16x8_t, c: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmlsl_high_s16(a, b, c) }
}

/// Signed multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_high_s32(a: int64x2_t, b: int32x4_t, c: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vmlsl_high_s32(a, b, c) }
}

/// Unsigned multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_high_u8(a: uint16x8_t, b: uint8x16_t, c: uint8x16_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vmlsl_high_u8(a, b, c) }
}

/// Unsigned multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_high_u16(a: uint32x4_t, b: uint16x8_t, c: uint16x8_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmlsl_high_u16(a, b, c) }
}

/// Unsigned multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_high_u32(a: uint64x2_t, b: uint32x4_t, c: uint32x4_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmlsl_high_u32(a, b, c) }
}

/// Multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_high_n_s16(a: int32x4_t, b: int16x8_t, c: i16) -> int32x4_t {
  unsafe { core::arch::aarch64::vmlsl_high_n_s16(a, b, c) }
}

/// Multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_high_n_s32(a: int64x2_t, b: int32x4_t, c: i32) -> int64x2_t {
  unsafe { core::arch::aarch64::vmlsl_high_n_s32(a, b, c) }
}

/// Multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_high_n_u16(a: uint32x4_t, b: uint16x8_t, c: u16) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmlsl_high_n_u16(a, b, c) }
}

/// Multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_high_n_u32(a: uint64x2_t, b: uint32x4_t, c: u32) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmlsl_high_n_u32(a, b, c) }
}

/// Multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_high_lane_s16<const LANE: i32>(a: int32x4_t, b: int16x8_t, c: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmlsl_high_lane_s16::<LANE>(a, b, c) }
}

/// Multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_high_laneq_s16<const LANE: i32>(a: int32x4_t, b: int16x8_t, c: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmlsl_high_laneq_s16::<LANE>(a, b, c) }
}

/// Multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_high_lane_s32<const LANE: i32>(a: int64x2_t, b: int32x4_t, c: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vmlsl_high_lane_s32::<LANE>(a, b, c) }
}

/// Multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_high_laneq_s32<const LANE: i32>(a: int64x2_t, b: int32x4_t, c: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vmlsl_high_laneq_s32::<LANE>(a, b, c) }
}

/// Multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_high_lane_u16<const LANE: i32>(a: uint32x4_t, b: uint16x8_t, c: uint16x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmlsl_high_lane_u16::<LANE>(a, b, c) }
}

/// Multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_high_laneq_u16<const LANE: i32>(a: uint32x4_t, b: uint16x8_t, c: uint16x8_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmlsl_high_laneq_u16::<LANE>(a, b, c) }
}

/// Multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_high_lane_u32<const LANE: i32>(a: uint64x2_t, b: uint32x4_t, c: uint32x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmlsl_high_lane_u32::<LANE>(a, b, c) }
}

/// Multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmlsl_high_laneq_u32<const LANE: i32>(a: uint64x2_t, b: uint32x4_t, c: uint32x4_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmlsl_high_laneq_u32::<LANE>(a, b, c) }
}

/// Extract narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmovn_high_s16(a: int8x8_t, b: int16x8_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vmovn_high_s16(a, b) }
}

/// Extract narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmovn_high_s32(a: int16x4_t, b: int32x4_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vmovn_high_s32(a, b) }
}

/// Extract narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmovn_high_s64(a: int32x2_t, b: int64x2_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmovn_high_s64(a, b) }
}

/// Extract narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmovn_high_u16(a: uint8x8_t, b: uint16x8_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vmovn_high_u16(a, b) }
}

/// Extract narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmovn_high_u32(a: uint16x4_t, b: uint32x4_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vmovn_high_u32(a, b) }
}

/// Extract narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmovn_high_u64(a: uint32x2_t, b: uint64x2_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmovn_high_u64(a, b) }
}

/// Negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vneg_s64(a: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vneg_s64(a) }
}

/// Negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vnegq_s64(a: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vnegq_s64(a) }
}

/// Negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vnegd_s64(a: i64) -> i64 {
  unsafe { core::arch::aarch64::vnegd_s64(a) }
}

/// Negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vneg_f64(a: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vneg_f64(a) }
}

/// Negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vnegq_f64(a: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vnegq_f64(a) }
}

/// Signed saturating negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqneg_s64(a: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vqneg_s64(a) }
}

/// Signed saturating negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqnegq_s64(a: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqnegq_s64(a) }
}

/// Signed saturating negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqnegb_s8(a: i8) -> i8 {
  unsafe { core::arch::aarch64::vqnegb_s8(a) }
}

/// Signed saturating negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqnegh_s16(a: i16) -> i16 {
  unsafe { core::arch::aarch64::vqnegh_s16(a) }
}

/// Signed saturating negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqnegs_s32(a: i32) -> i32 {
  unsafe { core::arch::aarch64::vqnegs_s32(a) }
}

/// Signed saturating negate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqnegd_s64(a: i64) -> i64 {
  unsafe { core::arch::aarch64::vqnegd_s64(a) }
}

/// Saturating subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqsubb_s8(a: i8, b: i8) -> i8 {
  unsafe { core::arch::aarch64::vqsubb_s8(a, b) }
}

/// Saturating subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqsubh_s16(a: i16, b: i16) -> i16 {
  unsafe { core::arch::aarch64::vqsubh_s16(a, b) }
}

/// Saturating subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqsubb_u8(a: u8, b: u8) -> u8 {
  unsafe { core::arch::aarch64::vqsubb_u8(a, b) }
}

/// Saturating subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqsubh_u16(a: u16, b: u16) -> u16 {
  unsafe { core::arch::aarch64::vqsubh_u16(a, b) }
}

/// Saturating subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqsubs_u32(a: u32, b: u32) -> u32 {
  unsafe { core::arch::aarch64::vqsubs_u32(a, b) }
}

/// Saturating subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqsubd_u64(a: u64, b: u64) -> u64 {
  unsafe { core::arch::aarch64::vqsubd_u64(a, b) }
}

/// Saturating subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqsubs_s32(a: i32, b: i32) -> i32 {
  unsafe { core::arch::aarch64::vqsubs_s32(a, b) }
}

/// Saturating subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqsubd_s64(a: i64, b: i64) -> i64 {
  unsafe { core::arch::aarch64::vqsubd_s64(a, b) }
}

/// Reverse bit order
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrbit_s8(a: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vrbit_s8(a) }
}

/// Reverse bit order
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrbitq_s8(a: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vrbitq_s8(a) }
}

/// Reverse bit order
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrbit_u8(a: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vrbit_u8(a) }
}

/// Reverse bit order
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrbitq_u8(a: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vrbitq_u8(a) }
}

/// Reverse bit order
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrbit_p8(a: poly8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vrbit_p8(a) }
}

/// Reverse bit order
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrbitq_p8(a: poly8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vrbitq_p8(a) }
}

/// Floating-point round to integral exact, using current rounding mode
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndx_f32(a: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vrndx_f32(a) }
}

/// Floating-point round to integral exact, using current rounding mode
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndxq_f32(a: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vrndxq_f32(a) }
}

/// Floating-point round to integral exact, using current rounding mode
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndx_f64(a: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vrndx_f64(a) }
}

/// Floating-point round to integral exact, using current rounding mode
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndxq_f64(a: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vrndxq_f64(a) }
}

/// Floating-point round to integral, to nearest with ties to away
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrnda_f32(a: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vrnda_f32(a) }
}

/// Floating-point round to integral, to nearest with ties to away
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndaq_f32(a: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vrndaq_f32(a) }
}

/// Floating-point round to integral, to nearest with ties to away
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrnda_f64(a: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vrnda_f64(a) }
}

/// Floating-point round to integral, to nearest with ties to away
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndaq_f64(a: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vrndaq_f64(a) }
}

/// Floating-point round to integral, to nearest with ties to even
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndn_f64(a: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vrndn_f64(a) }
}

/// Floating-point round to integral, to nearest with ties to even
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndnq_f64(a: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vrndnq_f64(a) }
}

/// Floating-point round to integral, to nearest with ties to even
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndns_f32(a: f32) -> f32 {
  unsafe { core::arch::aarch64::vrndns_f32(a) }
}

/// Floating-point round to integral, toward minus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndm_f32(a: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vrndm_f32(a) }
}

/// Floating-point round to integral, toward minus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndmq_f32(a: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vrndmq_f32(a) }
}

/// Floating-point round to integral, toward minus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndm_f64(a: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vrndm_f64(a) }
}

/// Floating-point round to integral, toward minus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndmq_f64(a: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vrndmq_f64(a) }
}

/// Floating-point round to integral, toward plus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndp_f32(a: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vrndp_f32(a) }
}

/// Floating-point round to integral, toward plus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndpq_f32(a: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vrndpq_f32(a) }
}

/// Floating-point round to integral, toward plus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndp_f64(a: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vrndp_f64(a) }
}

/// Floating-point round to integral, toward plus infinity
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndpq_f64(a: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vrndpq_f64(a) }
}

/// Floating-point round to integral, toward zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrnd_f32(a: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vrnd_f32(a) }
}

/// Floating-point round to integral, toward zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndq_f32(a: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vrndq_f32(a) }
}

/// Floating-point round to integral, toward zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrnd_f64(a: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vrnd_f64(a) }
}

/// Floating-point round to integral, toward zero
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndq_f64(a: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vrndq_f64(a) }
}

/// Floating-point round to integral, using current rounding mode
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndi_f32(a: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vrndi_f32(a) }
}

/// Floating-point round to integral, using current rounding mode
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndiq_f32(a: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vrndiq_f32(a) }
}

/// Floating-point round to integral, using current rounding mode
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndi_f64(a: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vrndi_f64(a) }
}

/// Floating-point round to integral, using current rounding mode
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrndiq_f64(a: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vrndiq_f64(a) }
}

/// Saturating add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqaddb_s8(a: i8, b: i8) -> i8 {
  unsafe { core::arch::aarch64::vqaddb_s8(a, b) }
}

/// Saturating add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqaddh_s16(a: i16, b: i16) -> i16 {
  unsafe { core::arch::aarch64::vqaddh_s16(a, b) }
}

/// Saturating add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqaddb_u8(a: u8, b: u8) -> u8 {
  unsafe { core::arch::aarch64::vqaddb_u8(a, b) }
}

/// Saturating add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqaddh_u16(a: u16, b: u16) -> u16 {
  unsafe { core::arch::aarch64::vqaddh_u16(a, b) }
}

/// Saturating add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqadds_u32(a: u32, b: u32) -> u32 {
  unsafe { core::arch::aarch64::vqadds_u32(a, b) }
}

/// Saturating add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqaddd_u64(a: u64, b: u64) -> u64 {
  unsafe { core::arch::aarch64::vqaddd_u64(a, b) }
}

/// Saturating add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqadds_s32(a: i32, b: i32) -> i32 {
  unsafe { core::arch::aarch64::vqadds_s32(a, b) }
}

/// Saturating add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqaddd_s64(a: i64, b: i64) -> i64 {
  unsafe { core::arch::aarch64::vqaddd_s64(a, b) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_f64_x2(a: &[f64; 2]) -> float64x1x2_t {
  unsafe { core::arch::aarch64::vld1_f64_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_f64_x2(a: &[f64; 4]) -> float64x2x2_t {
  unsafe { core::arch::aarch64::vld1q_f64_x2(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_f64_x3(a: &[f64; 3]) -> float64x1x3_t {
  unsafe { core::arch::aarch64::vld1_f64_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_f64_x3(a: &[f64; 6]) -> float64x2x3_t {
  unsafe { core::arch::aarch64::vld1q_f64_x3(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_f64_x4(a: &[f64; 4]) -> float64x1x4_t {
  unsafe { core::arch::aarch64::vld1_f64_x4(a.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_f64_x4(a: &[f64; 8]) -> float64x2x4_t {
  unsafe { core::arch::aarch64::vld1q_f64_x4(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_s64(a: &[i64; 4]) -> int64x2x2_t {
  unsafe { core::arch::aarch64::vld2q_s64(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_u64(a: &[u64; 4]) -> uint64x2x2_t {
  unsafe { core::arch::aarch64::vld2q_u64(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld2q_p64(a: &[p64; 4]) -> poly64x2x2_t {
  unsafe { core::arch::aarch64::vld2q_p64(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_f64(a: &[f64; 2]) -> float64x1x2_t {
  unsafe { core::arch::aarch64::vld2_f64(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_f64(a: &[f64; 4]) -> float64x2x2_t {
  unsafe { core::arch::aarch64::vld2q_f64(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_dup_s64(a: &[i64; 2]) -> int64x2x2_t {
  unsafe { core::arch::aarch64::vld2q_dup_s64(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_dup_u64(a: &[u64; 2]) -> uint64x2x2_t {
  unsafe { core::arch::aarch64::vld2q_dup_u64(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld2q_dup_p64(a: &[p64; 2]) -> poly64x2x2_t {
  unsafe { core::arch::aarch64::vld2q_dup_p64(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_dup_f64(a: &[f64; 2]) -> float64x1x2_t {
  unsafe { core::arch::aarch64::vld2_dup_f64(a.as_ptr()) }
}

/// Load single 2-element structure and replicate to all lanes of two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_dup_f64(a: &[f64; 2]) -> float64x2x2_t {
  unsafe { core::arch::aarch64::vld2q_dup_f64(a.as_ptr()) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_lane_s8<const LANE: i32>(a: &[i8; 2], b: int8x16x2_t) -> int8x16x2_t {
  unsafe { core::arch::aarch64::vld2q_lane_s8::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_lane_s64<const LANE: i32>(a: &[i64; 2], b: int64x1x2_t) -> int64x1x2_t {
  unsafe { core::arch::aarch64::vld2_lane_s64::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_lane_s64<const LANE: i32>(a: &[i64; 2], b: int64x2x2_t) -> int64x2x2_t {
  unsafe { core::arch::aarch64::vld2q_lane_s64::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld2_lane_p64<const LANE: i32>(a: &[p64; 2], b: poly64x1x2_t) -> poly64x1x2_t {
  unsafe { core::arch::aarch64::vld2_lane_p64::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld2q_lane_p64<const LANE: i32>(a: &[p64; 2], b: poly64x2x2_t) -> poly64x2x2_t {
  unsafe { core::arch::aarch64::vld2q_lane_p64::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_lane_u8<const LANE: i32>(a: &[u8; 2], b: uint8x16x2_t) -> uint8x16x2_t {
  unsafe { core::arch::aarch64::vld2q_lane_u8::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_lane_u64<const LANE: i32>(a: &[u64; 2], b: uint64x1x2_t) -> uint64x1x2_t {
  unsafe { core::arch::aarch64::vld2_lane_u64::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_lane_u64<const LANE: i32>(a: &[u64; 2], b: uint64x2x2_t) -> uint64x2x2_t {
  unsafe { core::arch::aarch64::vld2q_lane_u64::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_lane_p8<const LANE: i32>(a: &[p8; 2], b: poly8x16x2_t) -> poly8x16x2_t {
  unsafe { core::arch::aarch64::vld2q_lane_p8::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2_lane_f64<const LANE: i32>(a: &[f64; 2], b: float64x1x2_t) -> float64x1x2_t {
  unsafe { core::arch::aarch64::vld2_lane_f64::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 2-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld2q_lane_f64<const LANE: i32>(a: &[f64; 2], b: float64x2x2_t) -> float64x2x2_t {
  unsafe { core::arch::aarch64::vld2q_lane_f64::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_s64(a: &[i64; 6]) -> int64x2x3_t {
  unsafe { core::arch::aarch64::vld3q_s64(a.as_ptr()) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_u64(a: &[u64; 6]) -> uint64x2x3_t {
  unsafe { core::arch::aarch64::vld3q_u64(a.as_ptr()) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld3q_p64(a: &[p64; 6]) -> poly64x2x3_t {
  unsafe { core::arch::aarch64::vld3q_p64(a.as_ptr()) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_f64(a: &[f64; 3]) -> float64x1x3_t {
  unsafe { core::arch::aarch64::vld3_f64(a.as_ptr()) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_f64(a: &[f64; 6]) -> float64x2x3_t {
  unsafe { core::arch::aarch64::vld3q_f64(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_dup_s64(a: &[i64; 3]) -> int64x2x3_t {
  unsafe { core::arch::aarch64::vld3q_dup_s64(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_dup_u64(a: &[u64; 3]) -> uint64x2x3_t {
  unsafe { core::arch::aarch64::vld3q_dup_u64(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld3q_dup_p64(a: &[p64; 3]) -> poly64x2x3_t {
  unsafe { core::arch::aarch64::vld3q_dup_p64(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_dup_f64(a: &[f64; 3]) -> float64x1x3_t {
  unsafe { core::arch::aarch64::vld3_dup_f64(a.as_ptr()) }
}

/// Load single 3-element structure and replicate to all lanes of three
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_dup_f64(a: &[f64; 3]) -> float64x2x3_t {
  unsafe { core::arch::aarch64::vld3q_dup_f64(a.as_ptr()) }
}

/// Load multiple 3-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_lane_s8<const LANE: i32>(a: &[i8; 3], b: int8x16x3_t) -> int8x16x3_t {
  unsafe { core::arch::aarch64::vld3q_lane_s8::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_lane_s64<const LANE: i32>(a: &[i64; 3], b: int64x1x3_t) -> int64x1x3_t {
  unsafe { core::arch::aarch64::vld3_lane_s64::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_lane_s64<const LANE: i32>(a: &[i64; 3], b: int64x2x3_t) -> int64x2x3_t {
  unsafe { core::arch::aarch64::vld3q_lane_s64::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld3_lane_p64<const LANE: i32>(a: &[p64; 3], b: poly64x1x3_t) -> poly64x1x3_t {
  unsafe { core::arch::aarch64::vld3_lane_p64::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld3q_lane_p64<const LANE: i32>(a: &[p64; 3], b: poly64x2x3_t) -> poly64x2x3_t {
  unsafe { core::arch::aarch64::vld3q_lane_p64::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_lane_p8<const LANE: i32>(a: &[p8; 3], b: poly8x16x3_t) -> poly8x16x3_t {
  unsafe { core::arch::aarch64::vld3q_lane_p8::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_lane_u8<const LANE: i32>(a: &[u8; 3], b: uint8x16x3_t) -> uint8x16x3_t {
  unsafe { core::arch::aarch64::vld3q_lane_u8::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_lane_u64<const LANE: i32>(a: &[u64; 3], b: uint64x1x3_t) -> uint64x1x3_t {
  unsafe { core::arch::aarch64::vld3_lane_u64::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_lane_u64<const LANE: i32>(a: &[u64; 3], b: uint64x2x3_t) -> uint64x2x3_t {
  unsafe { core::arch::aarch64::vld3q_lane_u64::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3_lane_f64<const LANE: i32>(a: &[f64; 3], b: float64x1x3_t) -> float64x1x3_t {
  unsafe { core::arch::aarch64::vld3_lane_f64::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 3-element structures to three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld3q_lane_f64<const LANE: i32>(a: &[f64; 3], b: float64x2x3_t) -> float64x2x3_t {
  unsafe { core::arch::aarch64::vld3q_lane_f64::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_s64(a: &[i64; 8]) -> int64x2x4_t {
  unsafe { core::arch::aarch64::vld4q_s64(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_u64(a: &[u64; 8]) -> uint64x2x4_t {
  unsafe { core::arch::aarch64::vld4q_u64(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld4q_p64(a: &[p64; 8]) -> poly64x2x4_t {
  unsafe { core::arch::aarch64::vld4q_p64(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_f64(a: &[f64; 4]) -> float64x1x4_t {
  unsafe { core::arch::aarch64::vld4_f64(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_f64(a: &[f64; 8]) -> float64x2x4_t {
  unsafe { core::arch::aarch64::vld4q_f64(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_dup_s64(a: &[i64; 4]) -> int64x2x4_t {
  unsafe { core::arch::aarch64::vld4q_dup_s64(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_dup_u64(a: &[u64; 4]) -> uint64x2x4_t {
  unsafe { core::arch::aarch64::vld4q_dup_u64(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld4q_dup_p64(a: &[p64; 4]) -> poly64x2x4_t {
  unsafe { core::arch::aarch64::vld4q_dup_p64(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_dup_f64(a: &[f64; 4]) -> float64x1x4_t {
  unsafe { core::arch::aarch64::vld4_dup_f64(a.as_ptr()) }
}

/// Load single 4-element structure and replicate to all lanes of four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_dup_f64(a: &[f64; 4]) -> float64x2x4_t {
  unsafe { core::arch::aarch64::vld4q_dup_f64(a.as_ptr()) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_lane_s8<const LANE: i32>(a: &[i8; 4], b: int8x16x4_t) -> int8x16x4_t {
  unsafe { core::arch::aarch64::vld4q_lane_s8::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_lane_s64<const LANE: i32>(a: &[i64; 4], b: int64x1x4_t) -> int64x1x4_t {
  unsafe { core::arch::aarch64::vld4_lane_s64::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_lane_s64<const LANE: i32>(a: &[i64; 4], b: int64x2x4_t) -> int64x2x4_t {
  unsafe { core::arch::aarch64::vld4q_lane_s64::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld4_lane_p64<const LANE: i32>(a: &[p64; 4], b: poly64x1x4_t) -> poly64x1x4_t {
  unsafe { core::arch::aarch64::vld4_lane_p64::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld4q_lane_p64<const LANE: i32>(a: &[p64; 4], b: poly64x2x4_t) -> poly64x2x4_t {
  unsafe { core::arch::aarch64::vld4q_lane_p64::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_lane_p8<const LANE: i32>(a: &[p8; 4], b: poly8x16x4_t) -> poly8x16x4_t {
  unsafe { core::arch::aarch64::vld4q_lane_p8::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_lane_u8<const LANE: i32>(a: &[u8; 4], b: uint8x16x4_t) -> uint8x16x4_t {
  unsafe { core::arch::aarch64::vld4q_lane_u8::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_lane_u64<const LANE: i32>(a: &[u64; 4], b: uint64x1x4_t) -> uint64x1x4_t {
  unsafe { core::arch::aarch64::vld4_lane_u64::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_lane_u64<const LANE: i32>(a: &[u64; 4], b: uint64x2x4_t) -> uint64x2x4_t {
  unsafe { core::arch::aarch64::vld4q_lane_u64::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4_lane_f64<const LANE: i32>(a: &[f64; 4], b: float64x1x4_t) -> float64x1x4_t {
  unsafe { core::arch::aarch64::vld4_lane_f64::<LANE>(a.as_ptr(), b) }
}

/// Load multiple 4-element structures to four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld4q_lane_f64<const LANE: i32>(a: &[f64; 4], b: float64x2x4_t) -> float64x2x4_t {
  unsafe { core::arch::aarch64::vld4q_lane_f64::<LANE>(a.as_ptr(), b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_lane_f64<const LANE: i32>(a: &mut f64, b: float64x1_t) {
  unsafe { core::arch::aarch64::vst1_lane_f64::<LANE>(a, b) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_lane_f64<const LANE: i32>(a: &mut f64, b: float64x2_t) {
  unsafe { core::arch::aarch64::vst1q_lane_f64::<LANE>(a, b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_f64_x2(a: &mut [f64; 2], b: float64x1x2_t) {
  unsafe { core::arch::aarch64::vst1_f64_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_f64_x2(a: &mut [f64; 4], b: float64x2x2_t) {
  unsafe { core::arch::aarch64::vst1q_f64_x2(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_f64_x3(a: &mut [f64; 3], b: float64x1x3_t) {
  unsafe { core::arch::aarch64::vst1_f64_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_f64_x3(a: &mut [f64; 6], b: float64x2x3_t) {
  unsafe { core::arch::aarch64::vst1q_f64_x3(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1_f64_x4(a: &mut [f64; 4], b: float64x1x4_t) {
  unsafe { core::arch::aarch64::vst1_f64_x4(a.as_mut_ptr(), b) }
}

/// Store multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst1q_f64_x4(a: &mut [f64; 8], b: float64x2x4_t) {
  unsafe { core::arch::aarch64::vst1q_f64_x4(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2q_s64(a: &mut [i64; 4], b: int64x2x2_t) {
  unsafe { core::arch::aarch64::vst2q_s64(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2q_u64(a: &mut [u64; 4], b: uint64x2x2_t) {
  unsafe { core::arch::aarch64::vst2q_u64(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vst2q_p64(a: &mut [p64; 4], b: poly64x2x2_t) {
  unsafe { core::arch::aarch64::vst2q_p64(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2_f64(a: &mut [f64; 2], b: float64x1x2_t) {
  unsafe { core::arch::aarch64::vst2_f64(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2q_f64(a: &mut [f64; 4], b: float64x2x2_t) {
  unsafe { core::arch::aarch64::vst2q_f64(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2q_lane_s8<const LANE: i32>(a: &mut [i8; 2], b: int8x16x2_t) {
  unsafe { core::arch::aarch64::vst2q_lane_s8::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2_lane_s64<const LANE: i32>(a: &mut [i64; 2], b: int64x1x2_t) {
  unsafe { core::arch::aarch64::vst2_lane_s64::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2q_lane_s64<const LANE: i32>(a: &mut [i64; 2], b: int64x2x2_t) {
  unsafe { core::arch::aarch64::vst2q_lane_s64::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2q_lane_u8<const LANE: i32>(a: &mut [u8; 2], b: uint8x16x2_t) {
  unsafe { core::arch::aarch64::vst2q_lane_u8::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2_lane_u64<const LANE: i32>(a: &mut [u64; 2], b: uint64x1x2_t) {
  unsafe { core::arch::aarch64::vst2_lane_u64::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2q_lane_u64<const LANE: i32>(a: &mut [u64; 2], b: uint64x2x2_t) {
  unsafe { core::arch::aarch64::vst2q_lane_u64::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2q_lane_p8<const LANE: i32>(a: &mut [p8; 2], b: poly8x16x2_t) {
  unsafe { core::arch::aarch64::vst2q_lane_p8::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vst2_lane_p64<const LANE: i32>(a: &mut [p64; 2], b: poly64x1x2_t) {
  unsafe { core::arch::aarch64::vst2_lane_p64::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vst2q_lane_p64<const LANE: i32>(a: &mut [p64; 2], b: poly64x2x2_t) {
  unsafe { core::arch::aarch64::vst2q_lane_p64::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2_lane_f64<const LANE: i32>(a: &mut [f64; 2], b: float64x1x2_t) {
  unsafe { core::arch::aarch64::vst2_lane_f64::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 2-element structures from two registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst2q_lane_f64<const LANE: i32>(a: &mut [f64; 2], b: float64x2x2_t) {
  unsafe { core::arch::aarch64::vst2q_lane_f64::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3q_s64(a: &mut [i64; 6], b: int64x2x3_t) {
  unsafe { core::arch::aarch64::vst3q_s64(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3q_u64(a: &mut [u64; 6], b: uint64x2x3_t) {
  unsafe { core::arch::aarch64::vst3q_u64(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vst3q_p64(a: &mut [p64; 6], b: poly64x2x3_t) {
  unsafe { core::arch::aarch64::vst3q_p64(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3_f64(a: &mut [f64; 3], b: float64x1x3_t) {
  unsafe { core::arch::aarch64::vst3_f64(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3q_f64(a: &mut [f64; 6], b: float64x2x3_t) {
  unsafe { core::arch::aarch64::vst3q_f64(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3q_lane_s8<const LANE: i32>(a: &mut [i8; 3], b: int8x16x3_t) {
  unsafe { core::arch::aarch64::vst3q_lane_s8::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3_lane_s64<const LANE: i32>(a: &mut [i64; 3], b: int64x1x3_t) {
  unsafe { core::arch::aarch64::vst3_lane_s64::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3q_lane_s64<const LANE: i32>(a: &mut [i64; 3], b: int64x2x3_t) {
  unsafe { core::arch::aarch64::vst3q_lane_s64::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3q_lane_u8<const LANE: i32>(a: &mut [u8; 3], b: uint8x16x3_t) {
  unsafe { core::arch::aarch64::vst3q_lane_u8::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3_lane_u64<const LANE: i32>(a: &mut [u64; 3], b: uint64x1x3_t) {
  unsafe { core::arch::aarch64::vst3_lane_u64::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3q_lane_u64<const LANE: i32>(a: &mut [u64; 3], b: uint64x2x3_t) {
  unsafe { core::arch::aarch64::vst3q_lane_u64::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3q_lane_p8<const LANE: i32>(a: &mut [p8; 3], b: poly8x16x3_t) {
  unsafe { core::arch::aarch64::vst3q_lane_p8::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vst3_lane_p64<const LANE: i32>(a: &mut [p64; 3], b: poly64x1x3_t) {
  unsafe { core::arch::aarch64::vst3_lane_p64::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vst3q_lane_p64<const LANE: i32>(a: &mut [p64; 3], b: poly64x2x3_t) {
  unsafe { core::arch::aarch64::vst3q_lane_p64::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3_lane_f64<const LANE: i32>(a: &mut [f64; 3], b: float64x1x3_t) {
  unsafe { core::arch::aarch64::vst3_lane_f64::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 3-element structures from three registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst3q_lane_f64<const LANE: i32>(a: &mut [f64; 3], b: float64x2x3_t) {
  unsafe { core::arch::aarch64::vst3q_lane_f64::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4q_s64(a: &mut [i64; 8], b: int64x2x4_t) {
  unsafe { core::arch::aarch64::vst4q_s64(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4q_u64(a: &mut [u64; 8], b: uint64x2x4_t) {
  unsafe { core::arch::aarch64::vst4q_u64(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vst4q_p64(a: &mut [p64; 8], b: poly64x2x4_t) {
  unsafe { core::arch::aarch64::vst4q_p64(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4_f64(a: &mut [f64; 4], b: float64x1x4_t) {
  unsafe { core::arch::aarch64::vst4_f64(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4q_f64(a: &mut [f64; 8], b: float64x2x4_t) {
  unsafe { core::arch::aarch64::vst4q_f64(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4q_lane_s8<const LANE: i32>(a: &mut [i8; 4], b: int8x16x4_t) {
  unsafe { core::arch::aarch64::vst4q_lane_s8::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4_lane_s64<const LANE: i32>(a: &mut [i64; 4], b: int64x1x4_t) {
  unsafe { core::arch::aarch64::vst4_lane_s64::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4q_lane_s64<const LANE: i32>(a: &mut [i64; 4], b: int64x2x4_t) {
  unsafe { core::arch::aarch64::vst4q_lane_s64::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4q_lane_u8<const LANE: i32>(a: &mut [u8; 4], b: uint8x16x4_t) {
  unsafe { core::arch::aarch64::vst4q_lane_u8::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4_lane_u64<const LANE: i32>(a: &mut [u64; 4], b: uint64x1x4_t) {
  unsafe { core::arch::aarch64::vst4_lane_u64::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4q_lane_u64<const LANE: i32>(a: &mut [u64; 4], b: uint64x2x4_t) {
  unsafe { core::arch::aarch64::vst4q_lane_u64::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4q_lane_p8<const LANE: i32>(a: &mut [p8; 4], b: poly8x16x4_t) {
  unsafe { core::arch::aarch64::vst4q_lane_p8::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vst4_lane_p64<const LANE: i32>(a: &mut [p64; 4], b: poly64x1x4_t) {
  unsafe { core::arch::aarch64::vst4_lane_p64::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vst4q_lane_p64<const LANE: i32>(a: &mut [p64; 4], b: poly64x2x4_t) {
  unsafe { core::arch::aarch64::vst4q_lane_p64::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4_lane_f64<const LANE: i32>(a: &mut [f64; 4], b: float64x1x4_t) {
  unsafe { core::arch::aarch64::vst4_lane_f64::<LANE>(a.as_mut_ptr(), b) }
}

/// Store multiple 4-element structures from four registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vst4q_lane_f64<const LANE: i32>(a: &mut [f64; 4], b: float64x2x4_t) {
  unsafe { core::arch::aarch64::vst4q_lane_f64::<LANE>(a.as_mut_ptr(), b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_f64(a: float64x1_t, b: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vmul_f64(a, b) }
}

/// Multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vmulq_f64(a, b) }
}

/// Vector multiply by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_n_f64(a: float64x1_t, b: f64) -> float64x1_t {
  unsafe { core::arch::aarch64::vmul_n_f64(a, b) }
}

/// Vector multiply by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_n_f64(a: float64x2_t, b: f64) -> float64x2_t {
  unsafe { core::arch::aarch64::vmulq_n_f64(a, b) }
}

/// Floating-point multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_lane_f64<const LANE: i32>(a: float64x1_t, b: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vmul_lane_f64::<LANE>(a, b) }
}

/// Floating-point multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmul_laneq_f64<const LANE: i32>(a: float64x1_t, b: float64x2_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vmul_laneq_f64::<LANE>(a, b) }
}

/// Floating-point multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_lane_f64<const LANE: i32>(a: float64x2_t, b: float64x1_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vmulq_lane_f64::<LANE>(a, b) }
}

/// Floating-point multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulq_laneq_f64<const LANE: i32>(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vmulq_laneq_f64::<LANE>(a, b) }
}

/// Floating-point multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmuls_lane_f32<const LANE: i32>(a: f32, b: float32x2_t) -> f32 {
  unsafe { core::arch::aarch64::vmuls_lane_f32::<LANE>(a, b) }
}

/// Floating-point multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmuls_laneq_f32<const LANE: i32>(a: f32, b: float32x4_t) -> f32 {
  unsafe { core::arch::aarch64::vmuls_laneq_f32::<LANE>(a, b) }
}

/// Floating-point multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmuld_lane_f64<const LANE: i32>(a: f64, b: float64x1_t) -> f64 {
  unsafe { core::arch::aarch64::vmuld_lane_f64::<LANE>(a, b) }
}

/// Floating-point multiply
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmuld_laneq_f64<const LANE: i32>(a: f64, b: float64x2_t) -> f64 {
  unsafe { core::arch::aarch64::vmuld_laneq_f64::<LANE>(a, b) }
}

/// Signed multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_high_s8(a: int8x16_t, b: int8x16_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vmull_high_s8(a, b) }
}

/// Signed multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_high_s16(a: int16x8_t, b: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmull_high_s16(a, b) }
}

/// Signed multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_high_s32(a: int32x4_t, b: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vmull_high_s32(a, b) }
}

/// Unsigned multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_high_u8(a: uint8x16_t, b: uint8x16_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vmull_high_u8(a, b) }
}

/// Unsigned multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_high_u16(a: uint16x8_t, b: uint16x8_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmull_high_u16(a, b) }
}

/// Unsigned multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_high_u32(a: uint32x4_t, b: uint32x4_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmull_high_u32(a, b) }
}

/// Polynomial multiply long
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vmull_p64(a: p64, b: p64) -> p128 {
  unsafe { core::arch::aarch64::vmull_p64(a, b) }
}

/// Polynomial multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_high_p8(a: poly8x16_t, b: poly8x16_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vmull_high_p8(a, b) }
}

/// Polynomial multiply long
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vmull_high_p64(a: poly64x2_t, b: poly64x2_t) -> p128 {
  unsafe { core::arch::aarch64::vmull_high_p64(a, b) }
}

/// Multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_high_n_s16(a: int16x8_t, b: i16) -> int32x4_t {
  unsafe { core::arch::aarch64::vmull_high_n_s16(a, b) }
}

/// Multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_high_n_s32(a: int32x4_t, b: i32) -> int64x2_t {
  unsafe { core::arch::aarch64::vmull_high_n_s32(a, b) }
}

/// Multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_high_n_u16(a: uint16x8_t, b: u16) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmull_high_n_u16(a, b) }
}

/// Multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_high_n_u32(a: uint32x4_t, b: u32) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmull_high_n_u32(a, b) }
}

/// Multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_high_lane_s16<const LANE: i32>(a: int16x8_t, b: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmull_high_lane_s16::<LANE>(a, b) }
}

/// Multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_high_laneq_s16<const LANE: i32>(a: int16x8_t, b: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmull_high_laneq_s16::<LANE>(a, b) }
}

/// Multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_high_lane_s32<const LANE: i32>(a: int32x4_t, b: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vmull_high_lane_s32::<LANE>(a, b) }
}

/// Multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_high_laneq_s32<const LANE: i32>(a: int32x4_t, b: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vmull_high_laneq_s32::<LANE>(a, b) }
}

/// Multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_high_lane_u16<const LANE: i32>(a: uint16x8_t, b: uint16x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmull_high_lane_u16::<LANE>(a, b) }
}

/// Multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_high_laneq_u16<const LANE: i32>(a: uint16x8_t, b: uint16x8_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmull_high_laneq_u16::<LANE>(a, b) }
}

/// Multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_high_lane_u32<const LANE: i32>(a: uint32x4_t, b: uint32x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmull_high_lane_u32::<LANE>(a, b) }
}

/// Multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmull_high_laneq_u32<const LANE: i32>(a: uint32x4_t, b: uint32x4_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmull_high_laneq_u32::<LANE>(a, b) }
}

/// Floating-point multiply extended
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulx_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vmulx_f32(a, b) }
}

/// Floating-point multiply extended
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulxq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vmulxq_f32(a, b) }
}

/// Floating-point multiply extended
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulx_f64(a: float64x1_t, b: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vmulx_f64(a, b) }
}

/// Floating-point multiply extended
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulxq_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vmulxq_f64(a, b) }
}

/// Floating-point multiply extended
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulx_lane_f64<const LANE: i32>(a: float64x1_t, b: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vmulx_lane_f64::<LANE>(a, b) }
}

/// Floating-point multiply extended
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulx_laneq_f64<const LANE: i32>(a: float64x1_t, b: float64x2_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vmulx_laneq_f64::<LANE>(a, b) }
}

/// Floating-point multiply extended
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulx_lane_f32<const LANE: i32>(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vmulx_lane_f32::<LANE>(a, b) }
}

/// Floating-point multiply extended
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulx_laneq_f32<const LANE: i32>(a: float32x2_t, b: float32x4_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vmulx_laneq_f32::<LANE>(a, b) }
}

/// Floating-point multiply extended
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulxq_lane_f32<const LANE: i32>(a: float32x4_t, b: float32x2_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vmulxq_lane_f32::<LANE>(a, b) }
}

/// Floating-point multiply extended
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulxq_laneq_f32<const LANE: i32>(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vmulxq_laneq_f32::<LANE>(a, b) }
}

/// Floating-point multiply extended
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulxq_lane_f64<const LANE: i32>(a: float64x2_t, b: float64x1_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vmulxq_lane_f64::<LANE>(a, b) }
}

/// Floating-point multiply extended
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulxq_laneq_f64<const LANE: i32>(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vmulxq_laneq_f64::<LANE>(a, b) }
}

/// Floating-point multiply extended
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulxs_f32(a: f32, b: f32) -> f32 {
  unsafe { core::arch::aarch64::vmulxs_f32(a, b) }
}

/// Floating-point multiply extended
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulxd_f64(a: f64, b: f64) -> f64 {
  unsafe { core::arch::aarch64::vmulxd_f64(a, b) }
}

/// Floating-point multiply extended
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulxs_lane_f32<const LANE: i32>(a: f32, b: float32x2_t) -> f32 {
  unsafe { core::arch::aarch64::vmulxs_lane_f32::<LANE>(a, b) }
}

/// Floating-point multiply extended
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulxs_laneq_f32<const LANE: i32>(a: f32, b: float32x4_t) -> f32 {
  unsafe { core::arch::aarch64::vmulxs_laneq_f32::<LANE>(a, b) }
}

/// Floating-point multiply extended
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulxd_lane_f64<const LANE: i32>(a: f64, b: float64x1_t) -> f64 {
  unsafe { core::arch::aarch64::vmulxd_lane_f64::<LANE>(a, b) }
}

/// Floating-point multiply extended
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmulxd_laneq_f64<const LANE: i32>(a: f64, b: float64x2_t) -> f64 {
  unsafe { core::arch::aarch64::vmulxd_laneq_f64::<LANE>(a, b) }
}

/// Floating-point fused Multiply-Add to accumulator(vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfma_f64(a: float64x1_t, b: float64x1_t, c: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vfma_f64(a, b, c) }
}

/// Floating-point fused Multiply-Add to accumulator(vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfmaq_f64(a: float64x2_t, b: float64x2_t, c: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vfmaq_f64(a, b, c) }
}

/// Floating-point fused Multiply-Add to accumulator(vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfma_n_f64(a: float64x1_t, b: float64x1_t, c: f64) -> float64x1_t {
  unsafe { core::arch::aarch64::vfma_n_f64(a, b, c) }
}

/// Floating-point fused Multiply-Add to accumulator(vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfmaq_n_f64(a: float64x2_t, b: float64x2_t, c: f64) -> float64x2_t {
  unsafe { core::arch::aarch64::vfmaq_n_f64(a, b, c) }
}

/// Floating-point fused multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfma_lane_f32<const LANE: i32>(a: float32x2_t, b: float32x2_t, c: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vfma_lane_f32::<LANE>(a, b, c) }
}

/// Floating-point fused multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfma_laneq_f32<const LANE: i32>(a: float32x2_t, b: float32x2_t, c: float32x4_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vfma_laneq_f32::<LANE>(a, b, c) }
}

/// Floating-point fused multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfmaq_lane_f32<const LANE: i32>(a: float32x4_t, b: float32x4_t, c: float32x2_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vfmaq_lane_f32::<LANE>(a, b, c) }
}

/// Floating-point fused multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfmaq_laneq_f32<const LANE: i32>(a: float32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vfmaq_laneq_f32::<LANE>(a, b, c) }
}

/// Floating-point fused multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfma_lane_f64<const LANE: i32>(a: float64x1_t, b: float64x1_t, c: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vfma_lane_f64::<LANE>(a, b, c) }
}

/// Floating-point fused multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfma_laneq_f64<const LANE: i32>(a: float64x1_t, b: float64x1_t, c: float64x2_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vfma_laneq_f64::<LANE>(a, b, c) }
}

/// Floating-point fused multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfmaq_lane_f64<const LANE: i32>(a: float64x2_t, b: float64x2_t, c: float64x1_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vfmaq_lane_f64::<LANE>(a, b, c) }
}

/// Floating-point fused multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfmaq_laneq_f64<const LANE: i32>(a: float64x2_t, b: float64x2_t, c: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vfmaq_laneq_f64::<LANE>(a, b, c) }
}

/// Floating-point fused multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfmas_lane_f32<const LANE: i32>(a: f32, b: f32, c: float32x2_t) -> f32 {
  unsafe { core::arch::aarch64::vfmas_lane_f32::<LANE>(a, b, c) }
}

/// Floating-point fused multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfmas_laneq_f32<const LANE: i32>(a: f32, b: f32, c: float32x4_t) -> f32 {
  unsafe { core::arch::aarch64::vfmas_laneq_f32::<LANE>(a, b, c) }
}

/// Floating-point fused multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfmad_lane_f64<const LANE: i32>(a: f64, b: f64, c: float64x1_t) -> f64 {
  unsafe { core::arch::aarch64::vfmad_lane_f64::<LANE>(a, b, c) }
}

/// Floating-point fused multiply-add to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfmad_laneq_f64<const LANE: i32>(a: f64, b: f64, c: float64x2_t) -> f64 {
  unsafe { core::arch::aarch64::vfmad_laneq_f64::<LANE>(a, b, c) }
}

/// Floating-point fused multiply-subtract from accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfms_f64(a: float64x1_t, b: float64x1_t, c: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vfms_f64(a, b, c) }
}

/// Floating-point fused multiply-subtract from accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfmsq_f64(a: float64x2_t, b: float64x2_t, c: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vfmsq_f64(a, b, c) }
}

/// Floating-point fused Multiply-subtract to accumulator(vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfms_n_f64(a: float64x1_t, b: float64x1_t, c: f64) -> float64x1_t {
  unsafe { core::arch::aarch64::vfms_n_f64(a, b, c) }
}

/// Floating-point fused Multiply-subtract to accumulator(vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfmsq_n_f64(a: float64x2_t, b: float64x2_t, c: f64) -> float64x2_t {
  unsafe { core::arch::aarch64::vfmsq_n_f64(a, b, c) }
}

/// Floating-point fused multiply-subtract to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfms_lane_f32<const LANE: i32>(a: float32x2_t, b: float32x2_t, c: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vfms_lane_f32::<LANE>(a, b, c) }
}

/// Floating-point fused multiply-subtract to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfms_laneq_f32<const LANE: i32>(a: float32x2_t, b: float32x2_t, c: float32x4_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vfms_laneq_f32::<LANE>(a, b, c) }
}

/// Floating-point fused multiply-subtract to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfmsq_lane_f32<const LANE: i32>(a: float32x4_t, b: float32x4_t, c: float32x2_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vfmsq_lane_f32::<LANE>(a, b, c) }
}

/// Floating-point fused multiply-subtract to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfmsq_laneq_f32<const LANE: i32>(a: float32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vfmsq_laneq_f32::<LANE>(a, b, c) }
}

/// Floating-point fused multiply-subtract to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfms_lane_f64<const LANE: i32>(a: float64x1_t, b: float64x1_t, c: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vfms_lane_f64::<LANE>(a, b, c) }
}

/// Floating-point fused multiply-subtract to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfms_laneq_f64<const LANE: i32>(a: float64x1_t, b: float64x1_t, c: float64x2_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vfms_laneq_f64::<LANE>(a, b, c) }
}

/// Floating-point fused multiply-subtract to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfmsq_lane_f64<const LANE: i32>(a: float64x2_t, b: float64x2_t, c: float64x1_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vfmsq_lane_f64::<LANE>(a, b, c) }
}

/// Floating-point fused multiply-subtract to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfmsq_laneq_f64<const LANE: i32>(a: float64x2_t, b: float64x2_t, c: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vfmsq_laneq_f64::<LANE>(a, b, c) }
}

/// Floating-point fused multiply-subtract to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfmss_lane_f32<const LANE: i32>(a: f32, b: f32, c: float32x2_t) -> f32 {
  unsafe { core::arch::aarch64::vfmss_lane_f32::<LANE>(a, b, c) }
}

/// Floating-point fused multiply-subtract to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfmss_laneq_f32<const LANE: i32>(a: f32, b: f32, c: float32x4_t) -> f32 {
  unsafe { core::arch::aarch64::vfmss_laneq_f32::<LANE>(a, b, c) }
}

/// Floating-point fused multiply-subtract to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfmsd_lane_f64<const LANE: i32>(a: f64, b: f64, c: float64x1_t) -> f64 {
  unsafe { core::arch::aarch64::vfmsd_lane_f64::<LANE>(a, b, c) }
}

/// Floating-point fused multiply-subtract to accumulator
#[inline]
#[cfg(target_feature = "neon")]
pub fn vfmsd_laneq_f64<const LANE: i32>(a: f64, b: f64, c: float64x2_t) -> f64 {
  unsafe { core::arch::aarch64::vfmsd_laneq_f64::<LANE>(a, b, c) }
}

/// Divide
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdiv_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vdiv_f32(a, b) }
}

/// Divide
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdivq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vdivq_f32(a, b) }
}

/// Divide
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdiv_f64(a: float64x1_t, b: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vdiv_f64(a, b) }
}

/// Divide
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdivq_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vdivq_f64(a, b) }
}

/// Subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsub_f64(a: float64x1_t, b: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vsub_f64(a, b) }
}

/// Subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubq_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vsubq_f64(a, b) }
}

/// Subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubd_s64(a: i64, b: i64) -> i64 {
  unsafe { core::arch::aarch64::vsubd_s64(a, b) }
}

/// Subtract
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubd_u64(a: u64, b: u64) -> u64 {
  unsafe { core::arch::aarch64::vsubd_u64(a, b) }
}

/// Add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddd_s64(a: i64, b: i64) -> i64 {
  unsafe { core::arch::aarch64::vaddd_s64(a, b) }
}

/// Add
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddd_u64(a: u64, b: u64) -> u64 {
  unsafe { core::arch::aarch64::vaddd_u64(a, b) }
}

/// Floating-point add across vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddv_f32(a: float32x2_t) -> f32 {
  unsafe { core::arch::aarch64::vaddv_f32(a) }
}

/// Floating-point add across vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddvq_f32(a: float32x4_t) -> f32 {
  unsafe { core::arch::aarch64::vaddvq_f32(a) }
}

/// Floating-point add across vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddvq_f64(a: float64x2_t) -> f64 {
  unsafe { core::arch::aarch64::vaddvq_f64(a) }
}

/// Signed Add Long across Vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddlv_s16(a: int16x4_t) -> i32 {
  unsafe { core::arch::aarch64::vaddlv_s16(a) }
}

/// Signed Add Long across Vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddlvq_s16(a: int16x8_t) -> i32 {
  unsafe { core::arch::aarch64::vaddlvq_s16(a) }
}

/// Signed Add Long across Vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddlv_s32(a: int32x2_t) -> i64 {
  unsafe { core::arch::aarch64::vaddlv_s32(a) }
}

/// Signed Add Long across Vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddlvq_s32(a: int32x4_t) -> i64 {
  unsafe { core::arch::aarch64::vaddlvq_s32(a) }
}

/// Unsigned Add Long across Vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddlv_u16(a: uint16x4_t) -> u32 {
  unsafe { core::arch::aarch64::vaddlv_u16(a) }
}

/// Unsigned Add Long across Vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddlvq_u16(a: uint16x8_t) -> u32 {
  unsafe { core::arch::aarch64::vaddlvq_u16(a) }
}

/// Unsigned Add Long across Vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddlv_u32(a: uint32x2_t) -> u64 {
  unsafe { core::arch::aarch64::vaddlv_u32(a) }
}

/// Unsigned Add Long across Vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddlvq_u32(a: uint32x4_t) -> u64 {
  unsafe { core::arch::aarch64::vaddlvq_u32(a) }
}

/// Signed Subtract Wide
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubw_high_s8(a: int16x8_t, b: int8x16_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vsubw_high_s8(a, b) }
}

/// Signed Subtract Wide
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubw_high_s16(a: int32x4_t, b: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vsubw_high_s16(a, b) }
}

/// Signed Subtract Wide
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubw_high_s32(a: int64x2_t, b: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vsubw_high_s32(a, b) }
}

/// Unsigned Subtract Wide
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubw_high_u8(a: uint16x8_t, b: uint8x16_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vsubw_high_u8(a, b) }
}

/// Unsigned Subtract Wide
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubw_high_u16(a: uint32x4_t, b: uint16x8_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vsubw_high_u16(a, b) }
}

/// Unsigned Subtract Wide
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubw_high_u32(a: uint64x2_t, b: uint32x4_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vsubw_high_u32(a, b) }
}

/// Signed Subtract Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubl_high_s8(a: int8x16_t, b: int8x16_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vsubl_high_s8(a, b) }
}

/// Signed Subtract Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubl_high_s16(a: int16x8_t, b: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vsubl_high_s16(a, b) }
}

/// Signed Subtract Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubl_high_s32(a: int32x4_t, b: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vsubl_high_s32(a, b) }
}

/// Unsigned Subtract Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubl_high_u8(a: uint8x16_t, b: uint8x16_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vsubl_high_u8(a, b) }
}

/// Unsigned Subtract Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubl_high_u16(a: uint16x8_t, b: uint16x8_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vsubl_high_u16(a, b) }
}

/// Unsigned Subtract Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsubl_high_u32(a: uint32x4_t, b: uint32x4_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vsubl_high_u32(a, b) }
}

/// Bit clear and exclusive OR
#[inline]
#[cfg(target_feature = "neon,sha3")]
pub fn vbcaxq_s8(a: int8x16_t, b: int8x16_t, c: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vbcaxq_s8(a, b, c) }
}

/// Bit clear and exclusive OR
#[inline]
#[cfg(target_feature = "neon,sha3")]
pub fn vbcaxq_s16(a: int16x8_t, b: int16x8_t, c: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vbcaxq_s16(a, b, c) }
}

/// Bit clear and exclusive OR
#[inline]
#[cfg(target_feature = "neon,sha3")]
pub fn vbcaxq_s32(a: int32x4_t, b: int32x4_t, c: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vbcaxq_s32(a, b, c) }
}

/// Bit clear and exclusive OR
#[inline]
#[cfg(target_feature = "neon,sha3")]
pub fn vbcaxq_s64(a: int64x2_t, b: int64x2_t, c: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vbcaxq_s64(a, b, c) }
}

/// Bit clear and exclusive OR
#[inline]
#[cfg(target_feature = "neon,sha3")]
pub fn vbcaxq_u8(a: uint8x16_t, b: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vbcaxq_u8(a, b, c) }
}

/// Bit clear and exclusive OR
#[inline]
#[cfg(target_feature = "neon,sha3")]
pub fn vbcaxq_u16(a: uint16x8_t, b: uint16x8_t, c: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vbcaxq_u16(a, b, c) }
}

/// Bit clear and exclusive OR
#[inline]
#[cfg(target_feature = "neon,sha3")]
pub fn vbcaxq_u32(a: uint32x4_t, b: uint32x4_t, c: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vbcaxq_u32(a, b, c) }
}

/// Bit clear and exclusive OR
#[inline]
#[cfg(target_feature = "neon,sha3")]
pub fn vbcaxq_u64(a: uint64x2_t, b: uint64x2_t, c: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vbcaxq_u64(a, b, c) }
}

/// Floating-point complex add
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcadd_rot270_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vcadd_rot270_f32(a, b) }
}

/// Floating-point complex add
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcaddq_rot270_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vcaddq_rot270_f32(a, b) }
}

/// Floating-point complex add
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcaddq_rot270_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vcaddq_rot270_f64(a, b) }
}

/// Floating-point complex add
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcadd_rot90_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vcadd_rot90_f32(a, b) }
}

/// Floating-point complex add
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcaddq_rot90_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vcaddq_rot90_f32(a, b) }
}

/// Floating-point complex add
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcaddq_rot90_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vcaddq_rot90_f64(a, b) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmla_f32(a: float32x2_t, b: float32x2_t, c: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vcmla_f32(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmlaq_f32(a: float32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vcmlaq_f32(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmlaq_f64(a: float64x2_t, b: float64x2_t, c: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vcmlaq_f64(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmla_rot90_f32(a: float32x2_t, b: float32x2_t, c: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vcmla_rot90_f32(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmlaq_rot90_f32(a: float32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vcmlaq_rot90_f32(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmlaq_rot90_f64(a: float64x2_t, b: float64x2_t, c: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vcmlaq_rot90_f64(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmla_rot180_f32(a: float32x2_t, b: float32x2_t, c: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vcmla_rot180_f32(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmlaq_rot180_f32(a: float32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vcmlaq_rot180_f32(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmlaq_rot180_f64(a: float64x2_t, b: float64x2_t, c: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vcmlaq_rot180_f64(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmla_rot270_f32(a: float32x2_t, b: float32x2_t, c: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vcmla_rot270_f32(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmlaq_rot270_f32(a: float32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vcmlaq_rot270_f32(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmlaq_rot270_f64(a: float64x2_t, b: float64x2_t, c: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vcmlaq_rot270_f64(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmla_lane_f32<const LANE: i32>(a: float32x2_t, b: float32x2_t, c: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vcmla_lane_f32::<LANE>(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmla_laneq_f32<const LANE: i32>(a: float32x2_t, b: float32x2_t, c: float32x4_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vcmla_laneq_f32::<LANE>(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmlaq_lane_f32<const LANE: i32>(a: float32x4_t, b: float32x4_t, c: float32x2_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vcmlaq_lane_f32::<LANE>(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmlaq_laneq_f32<const LANE: i32>(a: float32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vcmlaq_laneq_f32::<LANE>(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmla_rot90_lane_f32<const LANE: i32>(a: float32x2_t, b: float32x2_t, c: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vcmla_rot90_lane_f32::<LANE>(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmla_rot90_laneq_f32<const LANE: i32>(a: float32x2_t, b: float32x2_t, c: float32x4_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vcmla_rot90_laneq_f32::<LANE>(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmlaq_rot90_lane_f32<const LANE: i32>(a: float32x4_t, b: float32x4_t, c: float32x2_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vcmlaq_rot90_lane_f32::<LANE>(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmlaq_rot90_laneq_f32<const LANE: i32>(a: float32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vcmlaq_rot90_laneq_f32::<LANE>(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmla_rot180_lane_f32<const LANE: i32>(a: float32x2_t, b: float32x2_t, c: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vcmla_rot180_lane_f32::<LANE>(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmla_rot180_laneq_f32<const LANE: i32>(a: float32x2_t, b: float32x2_t, c: float32x4_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vcmla_rot180_laneq_f32::<LANE>(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmlaq_rot180_lane_f32<const LANE: i32>(a: float32x4_t, b: float32x4_t, c: float32x2_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vcmlaq_rot180_lane_f32::<LANE>(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmlaq_rot180_laneq_f32<const LANE: i32>(a: float32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vcmlaq_rot180_laneq_f32::<LANE>(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmla_rot270_lane_f32<const LANE: i32>(a: float32x2_t, b: float32x2_t, c: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vcmla_rot270_lane_f32::<LANE>(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmla_rot270_laneq_f32<const LANE: i32>(a: float32x2_t, b: float32x2_t, c: float32x4_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vcmla_rot270_laneq_f32::<LANE>(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmlaq_rot270_lane_f32<const LANE: i32>(a: float32x4_t, b: float32x4_t, c: float32x2_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vcmlaq_rot270_lane_f32::<LANE>(a, b, c) }
}

/// Floating-point complex multiply accumulate
#[inline]
#[cfg(target_feature = "neon,fcma")]
pub fn vcmlaq_rot270_laneq_f32<const LANE: i32>(a: float32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vcmlaq_rot270_laneq_f32::<LANE>(a, b, c) }
}

/// Dot product arithmetic
#[inline]
#[cfg(target_feature = "neon,dotprod")]
pub fn vdot_s32(a: int32x2_t, b: int8x8_t, c: int8x8_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vdot_s32(a, b, c) }
}

/// Dot product arithmetic
#[inline]
#[cfg(target_feature = "neon,dotprod")]
pub fn vdotq_s32(a: int32x4_t, b: int8x16_t, c: int8x16_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vdotq_s32(a, b, c) }
}

/// Dot product arithmetic
#[inline]
#[cfg(target_feature = "neon,dotprod")]
pub fn vdot_u32(a: uint32x2_t, b: uint8x8_t, c: uint8x8_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vdot_u32(a, b, c) }
}

/// Dot product arithmetic
#[inline]
#[cfg(target_feature = "neon,dotprod")]
pub fn vdotq_u32(a: uint32x4_t, b: uint8x16_t, c: uint8x16_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vdotq_u32(a, b, c) }
}

/// Dot product arithmetic
#[inline]
#[cfg(target_feature = "neon,dotprod")]
pub fn vdot_lane_s32<const LANE: i32>(a: int32x2_t, b: int8x8_t, c: int8x8_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vdot_lane_s32::<LANE>(a, b, c) }
}

/// Dot product arithmetic
#[inline]
#[cfg(target_feature = "neon,dotprod")]
pub fn vdot_laneq_s32<const LANE: i32>(a: int32x2_t, b: int8x8_t, c: int8x16_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vdot_laneq_s32::<LANE>(a, b, c) }
}

/// Dot product arithmetic
#[inline]
#[cfg(target_feature = "neon,dotprod")]
pub fn vdotq_lane_s32<const LANE: i32>(a: int32x4_t, b: int8x16_t, c: int8x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vdotq_lane_s32::<LANE>(a, b, c) }
}

/// Dot product arithmetic
#[inline]
#[cfg(target_feature = "neon,dotprod")]
pub fn vdotq_laneq_s32<const LANE: i32>(a: int32x4_t, b: int8x16_t, c: int8x16_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vdotq_laneq_s32::<LANE>(a, b, c) }
}

/// Dot product arithmetic
#[inline]
#[cfg(target_feature = "neon,dotprod")]
pub fn vdot_lane_u32<const LANE: i32>(a: uint32x2_t, b: uint8x8_t, c: uint8x8_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vdot_lane_u32::<LANE>(a, b, c) }
}

/// Dot product arithmetic
#[inline]
#[cfg(target_feature = "neon,dotprod")]
pub fn vdot_laneq_u32<const LANE: i32>(a: uint32x2_t, b: uint8x8_t, c: uint8x16_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vdot_laneq_u32::<LANE>(a, b, c) }
}

/// Dot product arithmetic
#[inline]
#[cfg(target_feature = "neon,dotprod")]
pub fn vdotq_lane_u32<const LANE: i32>(a: uint32x4_t, b: uint8x16_t, c: uint8x8_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vdotq_lane_u32::<LANE>(a, b, c) }
}

/// Dot product arithmetic
#[inline]
#[cfg(target_feature = "neon,dotprod")]
pub fn vdotq_laneq_u32<const LANE: i32>(a: uint32x4_t, b: uint8x16_t, c: uint8x16_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vdotq_laneq_u32::<LANE>(a, b, c) }
}

/// Maximum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmax_f64(a: float64x1_t, b: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vmax_f64(a, b) }
}

/// Maximum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxq_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vmaxq_f64(a, b) }
}

/// Floating-point Maximum Number (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxnm_f64(a: float64x1_t, b: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vmaxnm_f64(a, b) }
}

/// Floating-point Maximum Number (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxnmq_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vmaxnmq_f64(a, b) }
}

/// Floating-point maximum number across vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxnmv_f32(a: float32x2_t) -> f32 {
  unsafe { core::arch::aarch64::vmaxnmv_f32(a) }
}

/// Floating-point maximum number across vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxnmvq_f64(a: float64x2_t) -> f64 {
  unsafe { core::arch::aarch64::vmaxnmvq_f64(a) }
}

/// Floating-point maximum number across vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxnmvq_f32(a: float32x4_t) -> f32 {
  unsafe { core::arch::aarch64::vmaxnmvq_f32(a) }
}

/// Floating-point Maximum Number Pairwise (vector).
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpmaxnm_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vpmaxnm_f32(a, b) }
}

/// Floating-point Maximum Number Pairwise (vector).
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpmaxnmq_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vpmaxnmq_f64(a, b) }
}

/// Floating-point Maximum Number Pairwise (vector).
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpmaxnmq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vpmaxnmq_f32(a, b) }
}

/// Floating-point maximum number pairwise
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpmaxnms_f32(a: float32x2_t) -> f32 {
  unsafe { core::arch::aarch64::vpmaxnms_f32(a) }
}

/// Floating-point maximum number pairwise
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpmaxnmqd_f64(a: float64x2_t) -> f64 {
  unsafe { core::arch::aarch64::vpmaxnmqd_f64(a) }
}

/// Floating-point maximum pairwise
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpmaxs_f32(a: float32x2_t) -> f32 {
  unsafe { core::arch::aarch64::vpmaxs_f32(a) }
}

/// Floating-point maximum pairwise
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpmaxqd_f64(a: float64x2_t) -> f64 {
  unsafe { core::arch::aarch64::vpmaxqd_f64(a) }
}

/// Minimum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmin_f64(a: float64x1_t, b: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vmin_f64(a, b) }
}

/// Minimum (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminq_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vminq_f64(a, b) }
}

/// Floating-point Minimum Number (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminnm_f64(a: float64x1_t, b: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vminnm_f64(a, b) }
}

/// Floating-point Minimum Number (vector)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminnmq_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vminnmq_f64(a, b) }
}

/// Floating-point minimum number across vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminnmv_f32(a: float32x2_t) -> f32 {
  unsafe { core::arch::aarch64::vminnmv_f32(a) }
}

/// Floating-point minimum number across vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminnmvq_f64(a: float64x2_t) -> f64 {
  unsafe { core::arch::aarch64::vminnmvq_f64(a) }
}

/// Floating-point minimum number across vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminnmvq_f32(a: float32x4_t) -> f32 {
  unsafe { core::arch::aarch64::vminnmvq_f32(a) }
}

/// Vector move
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmovl_high_s8(a: int8x16_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vmovl_high_s8(a) }
}

/// Vector move
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmovl_high_s16(a: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vmovl_high_s16(a) }
}

/// Vector move
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmovl_high_s32(a: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vmovl_high_s32(a) }
}

/// Vector move
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmovl_high_u8(a: uint8x16_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vmovl_high_u8(a) }
}

/// Vector move
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmovl_high_u16(a: uint16x8_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vmovl_high_u16(a) }
}

/// Vector move
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmovl_high_u32(a: uint32x4_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vmovl_high_u32(a) }
}

/// Floating-point add pairwise
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpaddq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vpaddq_f32(a, b) }
}

/// Floating-point add pairwise
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpaddq_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vpaddq_f64(a, b) }
}

/// Floating-point add pairwise
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpadds_f32(a: float32x2_t) -> f32 {
  unsafe { core::arch::aarch64::vpadds_f32(a) }
}

/// Floating-point add pairwise
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpaddd_f64(a: float64x2_t) -> f64 {
  unsafe { core::arch::aarch64::vpaddd_f64(a) }
}

/// Floating-point Minimum Number Pairwise (vector).
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpminnm_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vpminnm_f32(a, b) }
}

/// Floating-point Minimum Number Pairwise (vector).
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpminnmq_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vpminnmq_f64(a, b) }
}

/// Floating-point Minimum Number Pairwise (vector).
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpminnmq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vpminnmq_f32(a, b) }
}

/// Floating-point minimum number pairwise
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpminnms_f32(a: float32x2_t) -> f32 {
  unsafe { core::arch::aarch64::vpminnms_f32(a) }
}

/// Floating-point minimum number pairwise
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpminnmqd_f64(a: float64x2_t) -> f64 {
  unsafe { core::arch::aarch64::vpminnmqd_f64(a) }
}

/// Floating-point minimum pairwise
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpmins_f32(a: float32x2_t) -> f32 {
  unsafe { core::arch::aarch64::vpmins_f32(a) }
}

/// Floating-point minimum pairwise
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpminqd_f64(a: float64x2_t) -> f64 {
  unsafe { core::arch::aarch64::vpminqd_f64(a) }
}

/// Signed saturating doubling multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmullh_s16(a: i16, b: i16) -> i32 {
  unsafe { core::arch::aarch64::vqdmullh_s16(a, b) }
}

/// Signed saturating doubling multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulls_s32(a: i32, b: i32) -> i64 {
  unsafe { core::arch::aarch64::vqdmulls_s32(a, b) }
}

/// Signed saturating doubling multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmull_high_s16(a: int16x8_t, b: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmull_high_s16(a, b) }
}

/// Signed saturating doubling multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmull_high_s32(a: int32x4_t, b: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqdmull_high_s32(a, b) }
}

/// Signed saturating doubling multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmull_high_n_s16(a: int16x8_t, b: i16) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmull_high_n_s16(a, b) }
}

/// Signed saturating doubling multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmull_high_n_s32(a: int32x4_t, b: i32) -> int64x2_t {
  unsafe { core::arch::aarch64::vqdmull_high_n_s32(a, b) }
}

/// Vector saturating doubling long multiply by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmull_laneq_s16<const N: i32>(a: int16x4_t, b: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmull_laneq_s16::<N>(a, b) }
}

/// Vector saturating doubling long multiply by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmull_laneq_s32<const N: i32>(a: int32x2_t, b: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqdmull_laneq_s32::<N>(a, b) }
}

/// Signed saturating doubling multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmullh_lane_s16<const N: i32>(a: i16, b: int16x4_t) -> i32 {
  unsafe { core::arch::aarch64::vqdmullh_lane_s16::<N>(a, b) }
}

/// Signed saturating doubling multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmullh_laneq_s16<const N: i32>(a: i16, b: int16x8_t) -> i32 {
  unsafe { core::arch::aarch64::vqdmullh_laneq_s16::<N>(a, b) }
}

/// Signed saturating doubling multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulls_lane_s32<const N: i32>(a: i32, b: int32x2_t) -> i64 {
  unsafe { core::arch::aarch64::vqdmulls_lane_s32::<N>(a, b) }
}

/// Signed saturating doubling multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulls_laneq_s32<const N: i32>(a: i32, b: int32x4_t) -> i64 {
  unsafe { core::arch::aarch64::vqdmulls_laneq_s32::<N>(a, b) }
}

/// Signed saturating doubling multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmull_high_lane_s16<const N: i32>(a: int16x8_t, b: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmull_high_lane_s16::<N>(a, b) }
}

/// Signed saturating doubling multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmull_high_lane_s32<const N: i32>(a: int32x4_t, b: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqdmull_high_lane_s32::<N>(a, b) }
}

/// Signed saturating doubling multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmull_high_laneq_s16<const N: i32>(a: int16x8_t, b: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmull_high_laneq_s16::<N>(a, b) }
}

/// Signed saturating doubling multiply long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmull_high_laneq_s32<const N: i32>(a: int32x4_t, b: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqdmull_high_laneq_s32::<N>(a, b) }
}

/// Signed saturating doubling multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlal_high_s16(a: int32x4_t, b: int16x8_t, c: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmlal_high_s16(a, b, c) }
}

/// Signed saturating doubling multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlal_high_s32(a: int64x2_t, b: int32x4_t, c: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqdmlal_high_s32(a, b, c) }
}

/// Signed saturating doubling multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlal_high_n_s16(a: int32x4_t, b: int16x8_t, c: i16) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmlal_high_n_s16(a, b, c) }
}

/// Signed saturating doubling multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlal_high_n_s32(a: int64x2_t, b: int32x4_t, c: i32) -> int64x2_t {
  unsafe { core::arch::aarch64::vqdmlal_high_n_s32(a, b, c) }
}

/// Vector widening saturating doubling multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlal_laneq_s16<const N: i32>(a: int32x4_t, b: int16x4_t, c: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmlal_laneq_s16::<N>(a, b, c) }
}

/// Vector widening saturating doubling multiply accumulate with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlal_laneq_s32<const N: i32>(a: int64x2_t, b: int32x2_t, c: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqdmlal_laneq_s32::<N>(a, b, c) }
}

/// Signed saturating doubling multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlal_high_lane_s16<const N: i32>(a: int32x4_t, b: int16x8_t, c: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmlal_high_lane_s16::<N>(a, b, c) }
}

/// Signed saturating doubling multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlal_high_laneq_s16<const N: i32>(a: int32x4_t, b: int16x8_t, c: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmlal_high_laneq_s16::<N>(a, b, c) }
}

/// Signed saturating doubling multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlal_high_lane_s32<const N: i32>(a: int64x2_t, b: int32x4_t, c: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqdmlal_high_lane_s32::<N>(a, b, c) }
}

/// Signed saturating doubling multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlal_high_laneq_s32<const N: i32>(a: int64x2_t, b: int32x4_t, c: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqdmlal_high_laneq_s32::<N>(a, b, c) }
}

/// Signed saturating doubling multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlalh_s16(a: i32, b: i16, c: i16) -> i32 {
  unsafe { core::arch::aarch64::vqdmlalh_s16(a, b, c) }
}

/// Signed saturating doubling multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlals_s32(a: i64, b: i32, c: i32) -> i64 {
  unsafe { core::arch::aarch64::vqdmlals_s32(a, b, c) }
}

/// Signed saturating doubling multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlalh_lane_s16<const LANE: i32>(a: i32, b: i16, c: int16x4_t) -> i32 {
  unsafe { core::arch::aarch64::vqdmlalh_lane_s16::<LANE>(a, b, c) }
}

/// Signed saturating doubling multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlalh_laneq_s16<const LANE: i32>(a: i32, b: i16, c: int16x8_t) -> i32 {
  unsafe { core::arch::aarch64::vqdmlalh_laneq_s16::<LANE>(a, b, c) }
}

/// Signed saturating doubling multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlals_lane_s32<const LANE: i32>(a: i64, b: i32, c: int32x2_t) -> i64 {
  unsafe { core::arch::aarch64::vqdmlals_lane_s32::<LANE>(a, b, c) }
}

/// Signed saturating doubling multiply-add long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlals_laneq_s32<const LANE: i32>(a: i64, b: i32, c: int32x4_t) -> i64 {
  unsafe { core::arch::aarch64::vqdmlals_laneq_s32::<LANE>(a, b, c) }
}

/// Signed saturating doubling multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlsl_high_s16(a: int32x4_t, b: int16x8_t, c: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmlsl_high_s16(a, b, c) }
}

/// Signed saturating doubling multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlsl_high_s32(a: int64x2_t, b: int32x4_t, c: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqdmlsl_high_s32(a, b, c) }
}

/// Signed saturating doubling multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlsl_high_n_s16(a: int32x4_t, b: int16x8_t, c: i16) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmlsl_high_n_s16(a, b, c) }
}

/// Signed saturating doubling multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlsl_high_n_s32(a: int64x2_t, b: int32x4_t, c: i32) -> int64x2_t {
  unsafe { core::arch::aarch64::vqdmlsl_high_n_s32(a, b, c) }
}

/// Vector widening saturating doubling multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlsl_laneq_s16<const N: i32>(a: int32x4_t, b: int16x4_t, c: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmlsl_laneq_s16::<N>(a, b, c) }
}

/// Vector widening saturating doubling multiply subtract with scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlsl_laneq_s32<const N: i32>(a: int64x2_t, b: int32x2_t, c: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqdmlsl_laneq_s32::<N>(a, b, c) }
}

/// Signed saturating doubling multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlsl_high_lane_s16<const N: i32>(a: int32x4_t, b: int16x8_t, c: int16x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmlsl_high_lane_s16::<N>(a, b, c) }
}

/// Signed saturating doubling multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlsl_high_laneq_s16<const N: i32>(a: int32x4_t, b: int16x8_t, c: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmlsl_high_laneq_s16::<N>(a, b, c) }
}

/// Signed saturating doubling multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlsl_high_lane_s32<const N: i32>(a: int64x2_t, b: int32x4_t, c: int32x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqdmlsl_high_lane_s32::<N>(a, b, c) }
}

/// Signed saturating doubling multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlsl_high_laneq_s32<const N: i32>(a: int64x2_t, b: int32x4_t, c: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqdmlsl_high_laneq_s32::<N>(a, b, c) }
}

/// Signed saturating doubling multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlslh_s16(a: i32, b: i16, c: i16) -> i32 {
  unsafe { core::arch::aarch64::vqdmlslh_s16(a, b, c) }
}

/// Signed saturating doubling multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlsls_s32(a: i64, b: i32, c: i32) -> i64 {
  unsafe { core::arch::aarch64::vqdmlsls_s32(a, b, c) }
}

/// Signed saturating doubling multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlslh_lane_s16<const LANE: i32>(a: i32, b: i16, c: int16x4_t) -> i32 {
  unsafe { core::arch::aarch64::vqdmlslh_lane_s16::<LANE>(a, b, c) }
}

/// Signed saturating doubling multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlslh_laneq_s16<const LANE: i32>(a: i32, b: i16, c: int16x8_t) -> i32 {
  unsafe { core::arch::aarch64::vqdmlslh_laneq_s16::<LANE>(a, b, c) }
}

/// Signed saturating doubling multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlsls_lane_s32<const LANE: i32>(a: i64, b: i32, c: int32x2_t) -> i64 {
  unsafe { core::arch::aarch64::vqdmlsls_lane_s32::<LANE>(a, b, c) }
}

/// Signed saturating doubling multiply-subtract long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmlsls_laneq_s32<const LANE: i32>(a: i64, b: i32, c: int32x4_t) -> i64 {
  unsafe { core::arch::aarch64::vqdmlsls_laneq_s32::<LANE>(a, b, c) }
}

/// Signed saturating doubling multiply returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulhh_s16(a: i16, b: i16) -> i16 {
  unsafe { core::arch::aarch64::vqdmulhh_s16(a, b) }
}

/// Signed saturating doubling multiply returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulhs_s32(a: i32, b: i32) -> i32 {
  unsafe { core::arch::aarch64::vqdmulhs_s32(a, b) }
}

/// Signed saturating doubling multiply returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulhh_lane_s16<const N: i32>(a: i16, b: int16x4_t) -> i16 {
  unsafe { core::arch::aarch64::vqdmulhh_lane_s16::<N>(a, b) }
}

/// Signed saturating doubling multiply returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulhh_laneq_s16<const N: i32>(a: i16, b: int16x8_t) -> i16 {
  unsafe { core::arch::aarch64::vqdmulhh_laneq_s16::<N>(a, b) }
}

/// Signed saturating doubling multiply returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulhs_lane_s32<const N: i32>(a: i32, b: int32x2_t) -> i32 {
  unsafe { core::arch::aarch64::vqdmulhs_lane_s32::<N>(a, b) }
}

/// Signed saturating doubling multiply returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulhs_laneq_s32<const N: i32>(a: i32, b: int32x4_t) -> i32 {
  unsafe { core::arch::aarch64::vqdmulhs_laneq_s32::<N>(a, b) }
}

/// Vector saturating doubling multiply high by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulh_lane_s16<const LANE: i32>(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vqdmulh_lane_s16::<LANE>(a, b) }
}

/// Vector saturating doubling multiply high by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulhq_lane_s16<const LANE: i32>(a: int16x8_t, b: int16x4_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vqdmulhq_lane_s16::<LANE>(a, b) }
}

/// Vector saturating doubling multiply high by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulh_lane_s32<const LANE: i32>(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vqdmulh_lane_s32::<LANE>(a, b) }
}

/// Vector saturating doubling multiply high by scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqdmulhq_lane_s32<const LANE: i32>(a: int32x4_t, b: int32x2_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqdmulhq_lane_s32::<LANE>(a, b) }
}

/// Saturating extract narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovnh_s16(a: i16) -> i8 {
  unsafe { core::arch::aarch64::vqmovnh_s16(a) }
}

/// Saturating extract narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovns_s32(a: i32) -> i16 {
  unsafe { core::arch::aarch64::vqmovns_s32(a) }
}

/// Saturating extract narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovnh_u16(a: u16) -> u8 {
  unsafe { core::arch::aarch64::vqmovnh_u16(a) }
}

/// Saturating extract narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovns_u32(a: u32) -> u16 {
  unsafe { core::arch::aarch64::vqmovns_u32(a) }
}

/// Saturating extract narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovnd_s64(a: i64) -> i32 {
  unsafe { core::arch::aarch64::vqmovnd_s64(a) }
}

/// Saturating extract narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovnd_u64(a: u64) -> u32 {
  unsafe { core::arch::aarch64::vqmovnd_u64(a) }
}

/// Signed saturating extract narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovn_high_s16(a: int8x8_t, b: int16x8_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vqmovn_high_s16(a, b) }
}

/// Signed saturating extract narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovn_high_s32(a: int16x4_t, b: int32x4_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vqmovn_high_s32(a, b) }
}

/// Signed saturating extract narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovn_high_s64(a: int32x2_t, b: int64x2_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqmovn_high_s64(a, b) }
}

/// Signed saturating extract narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovn_high_u16(a: uint8x8_t, b: uint16x8_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vqmovn_high_u16(a, b) }
}

/// Signed saturating extract narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovn_high_u32(a: uint16x4_t, b: uint32x4_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vqmovn_high_u32(a, b) }
}

/// Signed saturating extract narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovn_high_u64(a: uint32x2_t, b: uint64x2_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vqmovn_high_u64(a, b) }
}

/// Signed saturating extract unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovunh_s16(a: i16) -> u8 {
  unsafe { core::arch::aarch64::vqmovunh_s16(a) }
}

/// Signed saturating extract unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovuns_s32(a: i32) -> u16 {
  unsafe { core::arch::aarch64::vqmovuns_s32(a) }
}

/// Signed saturating extract unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovund_s64(a: i64) -> u32 {
  unsafe { core::arch::aarch64::vqmovund_s64(a) }
}

/// Signed saturating extract unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovun_high_s16(a: uint8x8_t, b: int16x8_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vqmovun_high_s16(a, b) }
}

/// Signed saturating extract unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovun_high_s32(a: uint16x4_t, b: int32x4_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vqmovun_high_s32(a, b) }
}

/// Signed saturating extract unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqmovun_high_s64(a: uint32x2_t, b: int64x2_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vqmovun_high_s64(a, b) }
}

/// Signed saturating rounding doubling multiply returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmulhh_s16(a: i16, b: i16) -> i16 {
  unsafe { core::arch::aarch64::vqrdmulhh_s16(a, b) }
}

/// Signed saturating rounding doubling multiply returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmulhs_s32(a: i32, b: i32) -> i32 {
  unsafe { core::arch::aarch64::vqrdmulhs_s32(a, b) }
}

/// Signed saturating rounding doubling multiply returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmulhh_lane_s16<const LANE: i32>(a: i16, b: int16x4_t) -> i16 {
  unsafe { core::arch::aarch64::vqrdmulhh_lane_s16::<LANE>(a, b) }
}

/// Signed saturating rounding doubling multiply returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmulhh_laneq_s16<const LANE: i32>(a: i16, b: int16x8_t) -> i16 {
  unsafe { core::arch::aarch64::vqrdmulhh_laneq_s16::<LANE>(a, b) }
}

/// Signed saturating rounding doubling multiply returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmulhs_lane_s32<const LANE: i32>(a: i32, b: int32x2_t) -> i32 {
  unsafe { core::arch::aarch64::vqrdmulhs_lane_s32::<LANE>(a, b) }
}

/// Signed saturating rounding doubling multiply returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmulhs_laneq_s32<const LANE: i32>(a: i32, b: int32x4_t) -> i32 {
  unsafe { core::arch::aarch64::vqrdmulhs_laneq_s32::<LANE>(a, b) }
}

/// Signed saturating rounding doubling multiply accumulate returning high half
#[inline]
#[cfg(target_feature = "rdm")]
pub fn vqrdmlah_s16(a: int16x4_t, b: int16x4_t, c: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vqrdmlah_s16(a, b, c) }
}

/// Signed saturating rounding doubling multiply accumulate returning high half
#[inline]
#[cfg(target_feature = "rdm")]
pub fn vqrdmlahq_s16(a: int16x8_t, b: int16x8_t, c: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vqrdmlahq_s16(a, b, c) }
}

/// Signed saturating rounding doubling multiply accumulate returning high half
#[inline]
#[cfg(target_feature = "rdm")]
pub fn vqrdmlah_s32(a: int32x2_t, b: int32x2_t, c: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vqrdmlah_s32(a, b, c) }
}

/// Signed saturating rounding doubling multiply accumulate returning high half
#[inline]
#[cfg(target_feature = "rdm")]
pub fn vqrdmlahq_s32(a: int32x4_t, b: int32x4_t, c: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqrdmlahq_s32(a, b, c) }
}

/// Signed saturating rounding doubling multiply accumulate returning high half
#[inline]
#[cfg(target_feature = "rdm")]
pub fn vqrdmlahh_s16(a: i16, b: i16, c: i16) -> i16 {
  unsafe { core::arch::aarch64::vqrdmlahh_s16(a, b, c) }
}

/// Signed saturating rounding doubling multiply accumulate returning high half
#[inline]
#[cfg(target_feature = "rdm")]
pub fn vqrdmlahs_s32(a: i32, b: i32, c: i32) -> i32 {
  unsafe { core::arch::aarch64::vqrdmlahs_s32(a, b, c) }
}

/// Signed saturating rounding doubling multiply accumulate returning high half
#[inline]
#[cfg(target_feature = "rdm")]
pub fn vqrdmlah_lane_s16<const LANE: i32>(a: int16x4_t, b: int16x4_t, c: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vqrdmlah_lane_s16::<LANE>(a, b, c) }
}

/// Signed saturating rounding doubling multiply accumulate returning high half
#[inline]
#[cfg(target_feature = "rdm")]
pub fn vqrdmlah_laneq_s16<const LANE: i32>(a: int16x4_t, b: int16x4_t, c: int16x8_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vqrdmlah_laneq_s16::<LANE>(a, b, c) }
}

/// Signed saturating rounding doubling multiply accumulate returning high half
#[inline]
#[cfg(target_feature = "rdm")]
pub fn vqrdmlahq_lane_s16<const LANE: i32>(a: int16x8_t, b: int16x8_t, c: int16x4_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vqrdmlahq_lane_s16::<LANE>(a, b, c) }
}

/// Signed saturating rounding doubling multiply accumulate returning high half
#[inline]
#[cfg(target_feature = "rdm")]
pub fn vqrdmlahq_laneq_s16<const LANE: i32>(a: int16x8_t, b: int16x8_t, c: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vqrdmlahq_laneq_s16::<LANE>(a, b, c) }
}

/// Signed saturating rounding doubling multiply accumulate returning high half
#[inline]
#[cfg(target_feature = "rdm")]
pub fn vqrdmlah_lane_s32<const LANE: i32>(a: int32x2_t, b: int32x2_t, c: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vqrdmlah_lane_s32::<LANE>(a, b, c) }
}

/// Signed saturating rounding doubling multiply accumulate returning high half
#[inline]
#[cfg(target_feature = "rdm")]
pub fn vqrdmlah_laneq_s32<const LANE: i32>(a: int32x2_t, b: int32x2_t, c: int32x4_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vqrdmlah_laneq_s32::<LANE>(a, b, c) }
}

/// Signed saturating rounding doubling multiply accumulate returning high half
#[inline]
#[cfg(target_feature = "rdm")]
pub fn vqrdmlahq_lane_s32<const LANE: i32>(a: int32x4_t, b: int32x4_t, c: int32x2_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqrdmlahq_lane_s32::<LANE>(a, b, c) }
}

/// Signed saturating rounding doubling multiply accumulate returning high half
#[inline]
#[cfg(target_feature = "rdm")]
pub fn vqrdmlahq_laneq_s32<const LANE: i32>(a: int32x4_t, b: int32x4_t, c: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqrdmlahq_laneq_s32::<LANE>(a, b, c) }
}

/// Signed saturating rounding doubling multiply accumulate returning high half
#[inline]
#[cfg(target_feature = "rdm")]
pub fn vqrdmlahh_lane_s16<const LANE: i32>(a: i16, b: i16, c: int16x4_t) -> i16 {
  unsafe { core::arch::aarch64::vqrdmlahh_lane_s16::<LANE>(a, b, c) }
}

/// Signed saturating rounding doubling multiply accumulate returning high half
#[inline]
#[cfg(target_feature = "rdm")]
pub fn vqrdmlahh_laneq_s16<const LANE: i32>(a: i16, b: i16, c: int16x8_t) -> i16 {
  unsafe { core::arch::aarch64::vqrdmlahh_laneq_s16::<LANE>(a, b, c) }
}

/// Signed saturating rounding doubling multiply accumulate returning high half
#[inline]
#[cfg(target_feature = "rdm")]
pub fn vqrdmlahs_lane_s32<const LANE: i32>(a: i32, b: i32, c: int32x2_t) -> i32 {
  unsafe { core::arch::aarch64::vqrdmlahs_lane_s32::<LANE>(a, b, c) }
}

/// Signed saturating rounding doubling multiply accumulate returning high half
#[inline]
#[cfg(target_feature = "rdm")]
pub fn vqrdmlahs_laneq_s32<const LANE: i32>(a: i32, b: i32, c: int32x4_t) -> i32 {
  unsafe { core::arch::aarch64::vqrdmlahs_laneq_s32::<LANE>(a, b, c) }
}

/// Signed saturating rounding doubling multiply subtract returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmlshh_s16(a: i16, b: i16, c: i16) -> i16 {
  unsafe { core::arch::aarch64::vqrdmlshh_s16(a, b, c) }
}

/// Signed saturating rounding doubling multiply subtract returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmlshs_s32(a: i32, b: i32, c: i32) -> i32 {
  unsafe { core::arch::aarch64::vqrdmlshs_s32(a, b, c) }
}

/// Signed saturating rounding doubling multiply subtract returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmlshh_lane_s16<const LANE: i32>(a: i16, b: i16, c: int16x4_t) -> i16 {
  unsafe { core::arch::aarch64::vqrdmlshh_lane_s16::<LANE>(a, b, c) }
}

/// Signed saturating rounding doubling multiply subtract returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmlshh_laneq_s16<const LANE: i32>(a: i16, b: i16, c: int16x8_t) -> i16 {
  unsafe { core::arch::aarch64::vqrdmlshh_laneq_s16::<LANE>(a, b, c) }
}

/// Signed saturating rounding doubling multiply subtract returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmlshs_lane_s32<const LANE: i32>(a: i32, b: i32, c: int32x2_t) -> i32 {
  unsafe { core::arch::aarch64::vqrdmlshs_lane_s32::<LANE>(a, b, c) }
}

/// Signed saturating rounding doubling multiply subtract returning high half
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrdmlshs_laneq_s32<const LANE: i32>(a: i32, b: i32, c: int32x4_t) -> i32 {
  unsafe { core::arch::aarch64::vqrdmlshs_laneq_s32::<LANE>(a, b, c) }
}

/// Signed saturating rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshls_s32(a: i32, b: i32) -> i32 {
  unsafe { core::arch::aarch64::vqrshls_s32(a, b) }
}

/// Signed saturating rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshld_s64(a: i64, b: i64) -> i64 {
  unsafe { core::arch::aarch64::vqrshld_s64(a, b) }
}

/// Signed saturating rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshlb_s8(a: i8, b: i8) -> i8 {
  unsafe { core::arch::aarch64::vqrshlb_s8(a, b) }
}

/// Signed saturating rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshlh_s16(a: i16, b: i16) -> i16 {
  unsafe { core::arch::aarch64::vqrshlh_s16(a, b) }
}

/// Unsigned signed saturating rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshls_u32(a: u32, b: i32) -> u32 {
  unsafe { core::arch::aarch64::vqrshls_u32(a, b) }
}

/// Unsigned signed saturating rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshld_u64(a: u64, b: i64) -> u64 {
  unsafe { core::arch::aarch64::vqrshld_u64(a, b) }
}

/// Unsigned signed saturating rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshlb_u8(a: u8, b: i8) -> u8 {
  unsafe { core::arch::aarch64::vqrshlb_u8(a, b) }
}

/// Unsigned signed saturating rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshlh_u16(a: u16, b: i16) -> u16 {
  unsafe { core::arch::aarch64::vqrshlh_u16(a, b) }
}

/// Signed saturating rounded shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrnh_n_s16<const N: i32>(a: i16) -> i8 {
  unsafe { core::arch::aarch64::vqrshrnh_n_s16::<N>(a) }
}

/// Signed saturating rounded shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrns_n_s32<const N: i32>(a: i32) -> i16 {
  unsafe { core::arch::aarch64::vqrshrns_n_s32::<N>(a) }
}

/// Signed saturating rounded shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrnd_n_s64<const N: i32>(a: i64) -> i32 {
  unsafe { core::arch::aarch64::vqrshrnd_n_s64::<N>(a) }
}

/// Signed saturating rounded shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrn_high_n_s16<const N: i32>(a: int8x8_t, b: int16x8_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vqrshrn_high_n_s16::<N>(a, b) }
}

/// Signed saturating rounded shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrn_high_n_s32<const N: i32>(a: int16x4_t, b: int32x4_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vqrshrn_high_n_s32::<N>(a, b) }
}

/// Signed saturating rounded shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrn_high_n_s64<const N: i32>(a: int32x2_t, b: int64x2_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqrshrn_high_n_s64::<N>(a, b) }
}

/// Unsigned saturating rounded shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrnh_n_u16<const N: i32>(a: u16) -> u8 {
  unsafe { core::arch::aarch64::vqrshrnh_n_u16::<N>(a) }
}

/// Unsigned saturating rounded shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrns_n_u32<const N: i32>(a: u32) -> u16 {
  unsafe { core::arch::aarch64::vqrshrns_n_u32::<N>(a) }
}

/// Unsigned saturating rounded shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrnd_n_u64<const N: i32>(a: u64) -> u32 {
  unsafe { core::arch::aarch64::vqrshrnd_n_u64::<N>(a) }
}

/// Unsigned saturating rounded shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrn_high_n_u16<const N: i32>(a: uint8x8_t, b: uint16x8_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vqrshrn_high_n_u16::<N>(a, b) }
}

/// Unsigned saturating rounded shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrn_high_n_u32<const N: i32>(a: uint16x4_t, b: uint32x4_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vqrshrn_high_n_u32::<N>(a, b) }
}

/// Unsigned saturating rounded shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrn_high_n_u64<const N: i32>(a: uint32x2_t, b: uint64x2_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vqrshrn_high_n_u64::<N>(a, b) }
}

/// Signed saturating rounded shift right unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrunh_n_s16<const N: i32>(a: i16) -> u8 {
  unsafe { core::arch::aarch64::vqrshrunh_n_s16::<N>(a) }
}

/// Signed saturating rounded shift right unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshruns_n_s32<const N: i32>(a: i32) -> u16 {
  unsafe { core::arch::aarch64::vqrshruns_n_s32::<N>(a) }
}

/// Signed saturating rounded shift right unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrund_n_s64<const N: i32>(a: i64) -> u32 {
  unsafe { core::arch::aarch64::vqrshrund_n_s64::<N>(a) }
}

/// Signed saturating rounded shift right unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrun_high_n_s16<const N: i32>(a: uint8x8_t, b: int16x8_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vqrshrun_high_n_s16::<N>(a, b) }
}

/// Signed saturating rounded shift right unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrun_high_n_s32<const N: i32>(a: uint16x4_t, b: int32x4_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vqrshrun_high_n_s32::<N>(a, b) }
}

/// Signed saturating rounded shift right unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqrshrun_high_n_s64<const N: i32>(a: uint32x2_t, b: int64x2_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vqrshrun_high_n_s64::<N>(a, b) }
}

/// Signed saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshld_s64(a: i64, b: i64) -> i64 {
  unsafe { core::arch::aarch64::vqshld_s64(a, b) }
}

/// Signed saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlb_s8(a: i8, b: i8) -> i8 {
  unsafe { core::arch::aarch64::vqshlb_s8(a, b) }
}

/// Signed saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlh_s16(a: i16, b: i16) -> i16 {
  unsafe { core::arch::aarch64::vqshlh_s16(a, b) }
}

/// Signed saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshls_s32(a: i32, b: i32) -> i32 {
  unsafe { core::arch::aarch64::vqshls_s32(a, b) }
}

/// Unsigned saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshld_u64(a: u64, b: i64) -> u64 {
  unsafe { core::arch::aarch64::vqshld_u64(a, b) }
}

/// Unsigned saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlb_u8(a: u8, b: i8) -> u8 {
  unsafe { core::arch::aarch64::vqshlb_u8(a, b) }
}

/// Unsigned saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlh_u16(a: u16, b: i16) -> u16 {
  unsafe { core::arch::aarch64::vqshlh_u16(a, b) }
}

/// Unsigned saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshls_u32(a: u32, b: i32) -> u32 {
  unsafe { core::arch::aarch64::vqshls_u32(a, b) }
}

/// Signed saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlb_n_s8<const N: i32>(a: i8) -> i8 {
  unsafe { core::arch::aarch64::vqshlb_n_s8::<N>(a) }
}

/// Signed saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlh_n_s16<const N: i32>(a: i16) -> i16 {
  unsafe { core::arch::aarch64::vqshlh_n_s16::<N>(a) }
}

/// Signed saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshls_n_s32<const N: i32>(a: i32) -> i32 {
  unsafe { core::arch::aarch64::vqshls_n_s32::<N>(a) }
}

/// Signed saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshld_n_s64<const N: i32>(a: i64) -> i64 {
  unsafe { core::arch::aarch64::vqshld_n_s64::<N>(a) }
}

/// Unsigned saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlb_n_u8<const N: i32>(a: u8) -> u8 {
  unsafe { core::arch::aarch64::vqshlb_n_u8::<N>(a) }
}

/// Unsigned saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlh_n_u16<const N: i32>(a: u16) -> u16 {
  unsafe { core::arch::aarch64::vqshlh_n_u16::<N>(a) }
}

/// Unsigned saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshls_n_u32<const N: i32>(a: u32) -> u32 {
  unsafe { core::arch::aarch64::vqshls_n_u32::<N>(a) }
}

/// Unsigned saturating shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshld_n_u64<const N: i32>(a: u64) -> u64 {
  unsafe { core::arch::aarch64::vqshld_n_u64::<N>(a) }
}

/// Signed saturating shift left unsigned
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlub_n_s8<const N: i32>(a: i8) -> u8 {
  unsafe { core::arch::aarch64::vqshlub_n_s8::<N>(a) }
}

/// Signed saturating shift left unsigned
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshluh_n_s16<const N: i32>(a: i16) -> u16 {
  unsafe { core::arch::aarch64::vqshluh_n_s16::<N>(a) }
}

/// Signed saturating shift left unsigned
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlus_n_s32<const N: i32>(a: i32) -> u32 {
  unsafe { core::arch::aarch64::vqshlus_n_s32::<N>(a) }
}

/// Signed saturating shift left unsigned
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshlud_n_s64<const N: i32>(a: i64) -> u64 {
  unsafe { core::arch::aarch64::vqshlud_n_s64::<N>(a) }
}

/// Signed saturating shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrnd_n_s64<const N: i32>(a: i64) -> i32 {
  unsafe { core::arch::aarch64::vqshrnd_n_s64::<N>(a) }
}

/// Signed saturating shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrnh_n_s16<const N: i32>(a: i16) -> i8 {
  unsafe { core::arch::aarch64::vqshrnh_n_s16::<N>(a) }
}

/// Signed saturating shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrns_n_s32<const N: i32>(a: i32) -> i16 {
  unsafe { core::arch::aarch64::vqshrns_n_s32::<N>(a) }
}

/// Signed saturating shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrn_high_n_s16<const N: i32>(a: int8x8_t, b: int16x8_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vqshrn_high_n_s16::<N>(a, b) }
}

/// Signed saturating shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrn_high_n_s32<const N: i32>(a: int16x4_t, b: int32x4_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vqshrn_high_n_s32::<N>(a, b) }
}

/// Signed saturating shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrn_high_n_s64<const N: i32>(a: int32x2_t, b: int64x2_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vqshrn_high_n_s64::<N>(a, b) }
}

/// Unsigned saturating shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrnd_n_u64<const N: i32>(a: u64) -> u32 {
  unsafe { core::arch::aarch64::vqshrnd_n_u64::<N>(a) }
}

/// Unsigned saturating shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrnh_n_u16<const N: i32>(a: u16) -> u8 {
  unsafe { core::arch::aarch64::vqshrnh_n_u16::<N>(a) }
}

/// Unsigned saturating shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrns_n_u32<const N: i32>(a: u32) -> u16 {
  unsafe { core::arch::aarch64::vqshrns_n_u32::<N>(a) }
}

/// Unsigned saturating shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrn_high_n_u16<const N: i32>(a: uint8x8_t, b: uint16x8_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vqshrn_high_n_u16::<N>(a, b) }
}

/// Unsigned saturating shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrn_high_n_u32<const N: i32>(a: uint16x4_t, b: uint32x4_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vqshrn_high_n_u32::<N>(a, b) }
}

/// Unsigned saturating shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrn_high_n_u64<const N: i32>(a: uint32x2_t, b: uint64x2_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vqshrn_high_n_u64::<N>(a, b) }
}

/// Signed saturating shift right unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrunh_n_s16<const N: i32>(a: i16) -> u8 {
  unsafe { core::arch::aarch64::vqshrunh_n_s16::<N>(a) }
}

/// Signed saturating shift right unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshruns_n_s32<const N: i32>(a: i32) -> u16 {
  unsafe { core::arch::aarch64::vqshruns_n_s32::<N>(a) }
}

/// Signed saturating shift right unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrund_n_s64<const N: i32>(a: i64) -> u32 {
  unsafe { core::arch::aarch64::vqshrund_n_s64::<N>(a) }
}

/// Signed saturating shift right unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrun_high_n_s16<const N: i32>(a: uint8x8_t, b: int16x8_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vqshrun_high_n_s16::<N>(a, b) }
}

/// Signed saturating shift right unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrun_high_n_s32<const N: i32>(a: uint16x4_t, b: int32x4_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vqshrun_high_n_s32::<N>(a, b) }
}

/// Signed saturating shift right unsigned narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqshrun_high_n_s64<const N: i32>(a: uint32x2_t, b: int64x2_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vqshrun_high_n_s64::<N>(a, b) }
}

/// Unsigned saturating accumulate of signed value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsqaddb_u8(a: u8, b: i8) -> u8 {
  unsafe { core::arch::aarch64::vsqaddb_u8(a, b) }
}

/// Unsigned saturating accumulate of signed value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsqaddh_u16(a: u16, b: i16) -> u16 {
  unsafe { core::arch::aarch64::vsqaddh_u16(a, b) }
}

/// Unsigned saturating accumulate of signed value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsqadds_u32(a: u32, b: i32) -> u32 {
  unsafe { core::arch::aarch64::vsqadds_u32(a, b) }
}

/// Unsigned saturating accumulate of signed value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsqaddd_u64(a: u64, b: i64) -> u64 {
  unsafe { core::arch::aarch64::vsqaddd_u64(a, b) }
}

/// Calculates the square root of each lane.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsqrt_f32(a: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vsqrt_f32(a) }
}

/// Calculates the square root of each lane.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsqrtq_f32(a: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vsqrtq_f32(a) }
}

/// Calculates the square root of each lane.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsqrt_f64(a: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vsqrt_f64(a) }
}

/// Calculates the square root of each lane.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsqrtq_f64(a: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vsqrtq_f64(a) }
}

/// Reciprocal square-root estimate.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsqrte_f64(a: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vrsqrte_f64(a) }
}

/// Reciprocal square-root estimate.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsqrteq_f64(a: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vrsqrteq_f64(a) }
}

/// Reciprocal square-root estimate.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsqrtes_f32(a: f32) -> f32 {
  unsafe { core::arch::aarch64::vrsqrtes_f32(a) }
}

/// Reciprocal square-root estimate.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsqrted_f64(a: f64) -> f64 {
  unsafe { core::arch::aarch64::vrsqrted_f64(a) }
}

/// Floating-point reciprocal square root step
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsqrts_f64(a: float64x1_t, b: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vrsqrts_f64(a, b) }
}

/// Floating-point reciprocal square root step
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsqrtsq_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vrsqrtsq_f64(a, b) }
}

/// Floating-point reciprocal square root step
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsqrtss_f32(a: f32, b: f32) -> f32 {
  unsafe { core::arch::aarch64::vrsqrtss_f32(a, b) }
}

/// Floating-point reciprocal square root step
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsqrtsd_f64(a: f64, b: f64) -> f64 {
  unsafe { core::arch::aarch64::vrsqrtsd_f64(a, b) }
}

/// Reciprocal estimate.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrecpe_f64(a: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vrecpe_f64(a) }
}

/// Reciprocal estimate.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrecpeq_f64(a: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vrecpeq_f64(a) }
}

/// Reciprocal estimate.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrecpes_f32(a: f32) -> f32 {
  unsafe { core::arch::aarch64::vrecpes_f32(a) }
}

/// Reciprocal estimate.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrecped_f64(a: f64) -> f64 {
  unsafe { core::arch::aarch64::vrecped_f64(a) }
}

/// Floating-point reciprocal step
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrecps_f64(a: float64x1_t, b: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vrecps_f64(a, b) }
}

/// Floating-point reciprocal step
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrecpsq_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vrecpsq_f64(a, b) }
}

/// Floating-point reciprocal step
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrecpss_f32(a: f32, b: f32) -> f32 {
  unsafe { core::arch::aarch64::vrecpss_f32(a, b) }
}

/// Floating-point reciprocal step
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrecpsd_f64(a: f64, b: f64) -> f64 {
  unsafe { core::arch::aarch64::vrecpsd_f64(a, b) }
}

/// Floating-point reciprocal exponent
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrecpxs_f32(a: f32) -> f32 {
  unsafe { core::arch::aarch64::vrecpxs_f32(a) }
}

/// Floating-point reciprocal exponent
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrecpxd_f64(a: f64) -> f64 {
  unsafe { core::arch::aarch64::vrecpxd_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s64_p64(a: poly64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_s64_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u64_p64(a: poly64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_u64_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p64_s64(a: int64x1_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_p64_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p64_u64(a: uint64x1_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_p64_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s64_p64(a: poly64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_s64_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u64_p64(a: poly64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_u64_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p64_s64(a: int64x2_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_p64_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p64_u64(a: uint64x2_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_p64_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s8_f64(a: float64x1_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_s8_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s16_f64(a: float64x1_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_s16_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s32_f64(a: float64x1_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_s32_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_s64_f64(a: float64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_s64_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s8_f64(a: float64x2_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_s8_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s16_f64(a: float64x2_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_s16_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s32_f64(a: float64x2_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_s32_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_s64_f64(a: float64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_s64_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u8_f64(a: float64x1_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_u8_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u16_f64(a: float64x1_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_u16_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u32_f64(a: float64x1_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_u32_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_u64_f64(a: float64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_u64_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u8_f64(a: float64x2_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_u8_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u16_f64(a: float64x2_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_u16_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u32_f64(a: float64x2_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_u32_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_u64_f64(a: float64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_u64_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p8_f64(a: float64x1_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vreinterpret_p8_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p16_f64(a: float64x1_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vreinterpret_p16_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p64_f32(a: float32x2_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_p64_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_p64_f64(a: float64x1_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_p64_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p8_f64(a: float64x2_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vreinterpretq_p8_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p16_f64(a: float64x2_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vreinterpretq_p16_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p64_f32(a: float32x4_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_p64_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p64_f64(a: float64x2_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_p64_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_p128_f64(a: float64x2_t) -> p128 {
  unsafe { core::arch::aarch64::vreinterpretq_p128_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_f64_s8(a: int8x8_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_f64_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_f64_s16(a: int16x4_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_f64_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_f64_s32(a: int32x2_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_f64_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_f64_s64(a: int64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_f64_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f64_s8(a: int8x16_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_f64_s8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f64_s16(a: int16x8_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_f64_s16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f64_s32(a: int32x4_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_f64_s32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f64_s64(a: int64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_f64_s64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_f64_p8(a: poly8x8_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_f64_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_f64_u16(a: uint16x4_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_f64_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_f64_u32(a: uint32x2_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_f64_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_f64_u64(a: uint64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_f64_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f64_p8(a: poly8x16_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_f64_p8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f64_u16(a: uint16x8_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_f64_u16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f64_u32(a: uint32x4_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_f64_u32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f64_u64(a: uint64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_f64_u64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_f64_u8(a: uint8x8_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_f64_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_f64_p16(a: poly16x4_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_f64_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_f64_p64(a: poly64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_f64_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_f32_p64(a: poly64x1_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_f32_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f64_u8(a: uint8x16_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_f64_u8(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f64_p16(a: poly16x8_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_f64_p16(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f64_p64(a: poly64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_f64_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f32_p64(a: poly64x2_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_f32_p64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f64_p128(a: p128) -> float64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_f64_p128(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_f64_f32(a: float32x2_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vreinterpret_f64_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpret_f32_f64(a: float64x1_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vreinterpret_f32_f64(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f64_f32(a: float32x4_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vreinterpretq_f64_f32(a) }
}

/// Vector reinterpret cast operation
#[inline]
#[cfg(target_feature = "neon")]
pub fn vreinterpretq_f32_f64(a: float64x2_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vreinterpretq_f32_f64(a) }
}

/// Signed rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshld_s64(a: i64, b: i64) -> i64 {
  unsafe { core::arch::aarch64::vrshld_s64(a, b) }
}

/// Unsigned rounding shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshld_u64(a: u64, b: i64) -> u64 {
  unsafe { core::arch::aarch64::vrshld_u64(a, b) }
}

/// Signed rounding shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshrd_n_s64<const N: i32>(a: i64) -> i64 {
  unsafe { core::arch::aarch64::vrshrd_n_s64::<N>(a) }
}

/// Unsigned rounding shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshrd_n_u64<const N: i32>(a: u64) -> u64 {
  unsafe { core::arch::aarch64::vrshrd_n_u64::<N>(a) }
}

/// Rounding shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshrn_high_n_s16<const N: i32>(a: int8x8_t, b: int16x8_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vrshrn_high_n_s16::<N>(a, b) }
}

/// Rounding shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshrn_high_n_s32<const N: i32>(a: int16x4_t, b: int32x4_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vrshrn_high_n_s32::<N>(a, b) }
}

/// Rounding shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshrn_high_n_s64<const N: i32>(a: int32x2_t, b: int64x2_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vrshrn_high_n_s64::<N>(a, b) }
}

/// Rounding shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshrn_high_n_u16<const N: i32>(a: uint8x8_t, b: uint16x8_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vrshrn_high_n_u16::<N>(a, b) }
}

/// Rounding shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshrn_high_n_u32<const N: i32>(a: uint16x4_t, b: uint32x4_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vrshrn_high_n_u32::<N>(a, b) }
}

/// Rounding shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrshrn_high_n_u64<const N: i32>(a: uint32x2_t, b: uint64x2_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vrshrn_high_n_u64::<N>(a, b) }
}

/// Signed rounding shift right and accumulate.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsrad_n_s64<const N: i32>(a: i64, b: i64) -> i64 {
  unsafe { core::arch::aarch64::vrsrad_n_s64::<N>(a, b) }
}

/// Ungisned rounding shift right and accumulate.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsrad_n_u64<const N: i32>(a: u64, b: u64) -> u64 {
  unsafe { core::arch::aarch64::vrsrad_n_u64::<N>(a, b) }
}

/// Rounding subtract returning high narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsubhn_high_s16(a: int8x8_t, b: int16x8_t, c: int16x8_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vrsubhn_high_s16(a, b, c) }
}

/// Rounding subtract returning high narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsubhn_high_s32(a: int16x4_t, b: int32x4_t, c: int32x4_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vrsubhn_high_s32(a, b, c) }
}

/// Rounding subtract returning high narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsubhn_high_s64(a: int32x2_t, b: int64x2_t, c: int64x2_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vrsubhn_high_s64(a, b, c) }
}

/// Rounding subtract returning high narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsubhn_high_u16(a: uint8x8_t, b: uint16x8_t, c: uint16x8_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vrsubhn_high_u16(a, b, c) }
}

/// Rounding subtract returning high narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsubhn_high_u32(a: uint16x4_t, b: uint32x4_t, c: uint32x4_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vrsubhn_high_u32(a, b, c) }
}

/// Rounding subtract returning high narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vrsubhn_high_u64(a: uint32x2_t, b: uint64x2_t, c: uint64x2_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vrsubhn_high_u64(a, b, c) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vset_lane_f64<const LANE: i32>(a: f64, b: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vset_lane_f64::<LANE>(a, b) }
}

/// Insert vector element from another vector element
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsetq_lane_f64<const LANE: i32>(a: f64, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vsetq_lane_f64::<LANE>(a, b) }
}

/// Signed Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshld_s64(a: i64, b: i64) -> i64 {
  unsafe { core::arch::aarch64::vshld_s64(a, b) }
}

/// Unsigned Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshld_u64(a: u64, b: i64) -> u64 {
  unsafe { core::arch::aarch64::vshld_u64(a, b) }
}

/// Signed shift left long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshll_high_n_s8<const N: i32>(a: int8x16_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vshll_high_n_s8::<N>(a) }
}

/// Signed shift left long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshll_high_n_s16<const N: i32>(a: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vshll_high_n_s16::<N>(a) }
}

/// Signed shift left long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshll_high_n_s32<const N: i32>(a: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vshll_high_n_s32::<N>(a) }
}

/// Signed shift left long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshll_high_n_u8<const N: i32>(a: uint8x16_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vshll_high_n_u8::<N>(a) }
}

/// Signed shift left long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshll_high_n_u16<const N: i32>(a: uint16x8_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vshll_high_n_u16::<N>(a) }
}

/// Signed shift left long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshll_high_n_u32<const N: i32>(a: uint32x4_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vshll_high_n_u32::<N>(a) }
}

/// Shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshrn_high_n_s16<const N: i32>(a: int8x8_t, b: int16x8_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vshrn_high_n_s16::<N>(a, b) }
}

/// Shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshrn_high_n_s32<const N: i32>(a: int16x4_t, b: int32x4_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vshrn_high_n_s32::<N>(a, b) }
}

/// Shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshrn_high_n_s64<const N: i32>(a: int32x2_t, b: int64x2_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vshrn_high_n_s64::<N>(a, b) }
}

/// Shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshrn_high_n_u16<const N: i32>(a: uint8x8_t, b: uint16x8_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vshrn_high_n_u16::<N>(a, b) }
}

/// Shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshrn_high_n_u32<const N: i32>(a: uint16x4_t, b: uint32x4_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vshrn_high_n_u32::<N>(a, b) }
}

/// Shift right narrow
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshrn_high_n_u64<const N: i32>(a: uint32x2_t, b: uint64x2_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vshrn_high_n_u64::<N>(a, b) }
}

/// SM3PARTW1
#[inline]
#[cfg(target_feature = "neon,sm4")]
pub fn vsm3partw1q_u32(a: uint32x4_t, b: uint32x4_t, c: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vsm3partw1q_u32(a, b, c) }
}

/// SM3PARTW2
#[inline]
#[cfg(target_feature = "neon,sm4")]
pub fn vsm3partw2q_u32(a: uint32x4_t, b: uint32x4_t, c: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vsm3partw2q_u32(a, b, c) }
}

/// SM3SS1
#[inline]
#[cfg(target_feature = "neon,sm4")]
pub fn vsm3ss1q_u32(a: uint32x4_t, b: uint32x4_t, c: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vsm3ss1q_u32(a, b, c) }
}

/// SM4 key
#[inline]
#[cfg(target_feature = "neon,sm4")]
pub fn vsm4ekeyq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vsm4ekeyq_u32(a, b) }
}

/// SM4 encode
#[inline]
#[cfg(target_feature = "neon,sm4")]
pub fn vsm4eq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vsm4eq_u32(a, b) }
}

/// Rotate and exclusive OR
#[inline]
#[cfg(target_feature = "neon,sha3")]
pub fn vrax1q_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vrax1q_u64(a, b) }
}

/// SHA512 hash update part 1
#[inline]
#[cfg(target_feature = "neon,sha3")]
pub fn vsha512hq_u64(a: uint64x2_t, b: uint64x2_t, c: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vsha512hq_u64(a, b, c) }
}

/// SHA512 hash update part 2
#[inline]
#[cfg(target_feature = "neon,sha3")]
pub fn vsha512h2q_u64(a: uint64x2_t, b: uint64x2_t, c: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vsha512h2q_u64(a, b, c) }
}

/// SHA512 schedule update 0
#[inline]
#[cfg(target_feature = "neon,sha3")]
pub fn vsha512su0q_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vsha512su0q_u64(a, b) }
}

/// SHA512 schedule update 1
#[inline]
#[cfg(target_feature = "neon,sha3")]
pub fn vsha512su1q_u64(a: uint64x2_t, b: uint64x2_t, c: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vsha512su1q_u64(a, b, c) }
}

/// Floating-point round to 32-bit integer, using current rounding mode
#[inline]
#[cfg(target_feature = "neon,frintts")]
pub fn vrnd32x_f32(a: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vrnd32x_f32(a) }
}

/// Floating-point round to 32-bit integer, using current rounding mode
#[inline]
#[cfg(target_feature = "neon,frintts")]
pub fn vrnd32xq_f32(a: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vrnd32xq_f32(a) }
}

/// Floating-point round to 32-bit integer toward zero
#[inline]
#[cfg(target_feature = "neon,frintts")]
pub fn vrnd32z_f32(a: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vrnd32z_f32(a) }
}

/// Floating-point round to 32-bit integer toward zero
#[inline]
#[cfg(target_feature = "neon,frintts")]
pub fn vrnd32zq_f32(a: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vrnd32zq_f32(a) }
}

/// Floating-point round to 64-bit integer, using current rounding mode
#[inline]
#[cfg(target_feature = "neon,frintts")]
pub fn vrnd64x_f32(a: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vrnd64x_f32(a) }
}

/// Floating-point round to 64-bit integer, using current rounding mode
#[inline]
#[cfg(target_feature = "neon,frintts")]
pub fn vrnd64xq_f32(a: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vrnd64xq_f32(a) }
}

/// Floating-point round to 64-bit integer toward zero
#[inline]
#[cfg(target_feature = "neon,frintts")]
pub fn vrnd64z_f32(a: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vrnd64z_f32(a) }
}

/// Floating-point round to 64-bit integer toward zero
#[inline]
#[cfg(target_feature = "neon,frintts")]
pub fn vrnd64zq_f32(a: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vrnd64zq_f32(a) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn1_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vtrn1_s8(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn1q_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vtrn1q_s8(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn1_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vtrn1_s16(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn1q_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vtrn1q_s16(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn1q_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vtrn1q_s32(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn1_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vtrn1_u8(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn1q_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vtrn1q_u8(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn1_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vtrn1_u16(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn1q_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vtrn1q_u16(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn1q_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vtrn1q_u32(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn1_p8(a: poly8x8_t, b: poly8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vtrn1_p8(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn1q_p8(a: poly8x16_t, b: poly8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vtrn1q_p8(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn1_p16(a: poly16x4_t, b: poly16x4_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vtrn1_p16(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn1q_p16(a: poly16x8_t, b: poly16x8_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vtrn1q_p16(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn1_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vtrn1_s32(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn1q_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vtrn1q_s64(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn1_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vtrn1_u32(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn1q_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vtrn1q_u64(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn1q_p64(a: poly64x2_t, b: poly64x2_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vtrn1q_p64(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn1q_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vtrn1q_f32(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn1_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vtrn1_f32(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn1q_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vtrn1q_f64(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn2_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vtrn2_s8(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn2q_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vtrn2q_s8(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn2_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vtrn2_s16(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn2q_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vtrn2q_s16(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn2q_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vtrn2q_s32(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn2_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vtrn2_u8(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn2q_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vtrn2q_u8(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn2_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vtrn2_u16(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn2q_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vtrn2q_u16(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn2q_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vtrn2q_u32(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn2_p8(a: poly8x8_t, b: poly8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vtrn2_p8(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn2q_p8(a: poly8x16_t, b: poly8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vtrn2q_p8(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn2_p16(a: poly16x4_t, b: poly16x4_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vtrn2_p16(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn2q_p16(a: poly16x8_t, b: poly16x8_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vtrn2q_p16(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn2_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vtrn2_s32(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn2q_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vtrn2q_s64(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn2_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vtrn2_u32(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn2q_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vtrn2q_u64(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn2q_p64(a: poly64x2_t, b: poly64x2_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vtrn2q_p64(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn2q_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vtrn2q_f32(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn2_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vtrn2_f32(a, b) }
}

/// Transpose vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vtrn2q_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vtrn2q_f64(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip1_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vzip1_s8(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip1q_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vzip1q_s8(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip1_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vzip1_s16(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip1q_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vzip1q_s16(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip1_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vzip1_s32(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip1q_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vzip1q_s32(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip1q_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vzip1q_s64(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip1_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vzip1_u8(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip1q_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vzip1q_u8(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip1_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vzip1_u16(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip1q_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vzip1q_u16(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip1_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vzip1_u32(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip1q_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vzip1q_u32(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip1q_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vzip1q_u64(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip1_p8(a: poly8x8_t, b: poly8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vzip1_p8(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip1q_p8(a: poly8x16_t, b: poly8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vzip1q_p8(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip1_p16(a: poly16x4_t, b: poly16x4_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vzip1_p16(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip1q_p16(a: poly16x8_t, b: poly16x8_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vzip1q_p16(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip1q_p64(a: poly64x2_t, b: poly64x2_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vzip1q_p64(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip1_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vzip1_f32(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip1q_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vzip1q_f32(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip1q_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vzip1q_f64(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip2_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vzip2_s8(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip2q_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vzip2q_s8(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip2_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vzip2_s16(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip2q_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vzip2q_s16(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip2_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vzip2_s32(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip2q_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vzip2q_s32(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip2q_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vzip2q_s64(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip2_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vzip2_u8(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip2q_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vzip2q_u8(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip2_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vzip2_u16(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip2q_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vzip2q_u16(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip2_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vzip2_u32(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip2q_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vzip2q_u32(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip2q_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vzip2q_u64(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip2_p8(a: poly8x8_t, b: poly8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vzip2_p8(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip2q_p8(a: poly8x16_t, b: poly8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vzip2q_p8(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip2_p16(a: poly16x4_t, b: poly16x4_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vzip2_p16(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip2q_p16(a: poly16x8_t, b: poly16x8_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vzip2q_p16(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip2q_p64(a: poly64x2_t, b: poly64x2_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vzip2q_p64(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip2_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vzip2_f32(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip2q_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vzip2q_f32(a, b) }
}

/// Zip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vzip2q_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vzip2q_f64(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp1_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vuzp1_s8(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp1q_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vuzp1q_s8(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp1_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vuzp1_s16(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp1q_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vuzp1q_s16(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp1q_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vuzp1q_s32(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp1_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vuzp1_u8(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp1q_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vuzp1q_u8(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp1_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vuzp1_u16(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp1q_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vuzp1q_u16(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp1q_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vuzp1q_u32(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp1_p8(a: poly8x8_t, b: poly8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vuzp1_p8(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp1q_p8(a: poly8x16_t, b: poly8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vuzp1q_p8(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp1_p16(a: poly16x4_t, b: poly16x4_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vuzp1_p16(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp1q_p16(a: poly16x8_t, b: poly16x8_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vuzp1q_p16(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp1_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vuzp1_s32(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp1q_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vuzp1q_s64(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp1_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vuzp1_u32(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp1q_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vuzp1q_u64(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp1q_p64(a: poly64x2_t, b: poly64x2_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vuzp1q_p64(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp1q_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vuzp1q_f32(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp1_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vuzp1_f32(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp1q_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vuzp1q_f64(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp2_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vuzp2_s8(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp2q_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vuzp2q_s8(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp2_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vuzp2_s16(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp2q_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vuzp2q_s16(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp2q_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vuzp2q_s32(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp2_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vuzp2_u8(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp2q_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vuzp2q_u8(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp2_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vuzp2_u16(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp2q_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vuzp2q_u16(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp2q_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vuzp2q_u32(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp2_p8(a: poly8x8_t, b: poly8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vuzp2_p8(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp2q_p8(a: poly8x16_t, b: poly8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vuzp2q_p8(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp2_p16(a: poly16x4_t, b: poly16x4_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vuzp2_p16(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp2q_p16(a: poly16x8_t, b: poly16x8_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vuzp2q_p16(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp2_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vuzp2_s32(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp2q_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vuzp2q_s64(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp2_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vuzp2_u32(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp2q_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vuzp2q_u64(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp2q_p64(a: poly64x2_t, b: poly64x2_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vuzp2q_p64(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp2q_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vuzp2q_f32(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp2_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
  unsafe { core::arch::aarch64::vuzp2_f32(a, b) }
}

/// Unzip vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuzp2q_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vuzp2q_f64(a, b) }
}

/// Unsigned Absolute difference and Accumulate Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabal_high_u8(a: uint16x8_t, b: uint8x16_t, c: uint8x16_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vabal_high_u8(a, b, c) }
}

/// Unsigned Absolute difference and Accumulate Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabal_high_u16(a: uint32x4_t, b: uint16x8_t, c: uint16x8_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vabal_high_u16(a, b, c) }
}

/// Unsigned Absolute difference and Accumulate Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabal_high_u32(a: uint64x2_t, b: uint32x4_t, c: uint32x4_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vabal_high_u32(a, b, c) }
}

/// Signed Absolute difference and Accumulate Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabal_high_s8(a: int16x8_t, b: int8x16_t, c: int8x16_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vabal_high_s8(a, b, c) }
}

/// Signed Absolute difference and Accumulate Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabal_high_s16(a: int32x4_t, b: int16x8_t, c: int16x8_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vabal_high_s16(a, b, c) }
}

/// Signed Absolute difference and Accumulate Long
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabal_high_s32(a: int64x2_t, b: int32x4_t, c: int32x4_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vabal_high_s32(a, b, c) }
}

/// Singned saturating Absolute value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqabs_s64(a: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vqabs_s64(a) }
}

/// Singned saturating Absolute value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqabsq_s64(a: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vqabsq_s64(a) }
}

/// Signed saturating absolute value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqabsb_s8(a: i8) -> i8 {
  unsafe { core::arch::aarch64::vqabsb_s8(a) }
}

/// Signed saturating absolute value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqabsh_s16(a: i16) -> i16 {
  unsafe { core::arch::aarch64::vqabsh_s16(a) }
}

/// Signed saturating absolute value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqabss_s32(a: i32) -> i32 {
  unsafe { core::arch::aarch64::vqabss_s32(a) }
}

/// Signed saturating absolute value
#[inline]
#[cfg(target_feature = "neon")]
pub fn vqabsd_s64(a: i64) -> i64 {
  unsafe { core::arch::aarch64::vqabsd_s64(a) }
}

/// Shift left and insert
#[inline]
#[cfg(target_feature = "neon")]
pub fn vslid_n_s64<const N: i32>(a: i64, b: i64) -> i64 {
  unsafe { core::arch::aarch64::vslid_n_s64::<N>(a, b) }
}

/// Shift left and insert
#[inline]
#[cfg(target_feature = "neon")]
pub fn vslid_n_u64<const N: i32>(a: u64, b: u64) -> u64 {
  unsafe { core::arch::aarch64::vslid_n_u64::<N>(a, b) }
}

/// Shift right and insert
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsrid_n_s64<const N: i32>(a: i64, b: i64) -> i64 {
  unsafe { core::arch::aarch64::vsrid_n_s64::<N>(a, b) }
}

/// Shift right and insert
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsrid_n_u64<const N: i32>(a: u64, b: u64) -> u64 {
  unsafe { core::arch::aarch64::vsrid_n_u64::<N>(a, b) }
}
