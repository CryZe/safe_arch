use super::*;

type p8 = u8;
type p16 = u16;
type p64 = u64;
type p128 = u128;

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_lane_s64<const N1: i32, const N2: i32>(_a: int64x1_t, b: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vcopy_lane_s64::<N1, N2>(_a, b) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_lane_u64<const N1: i32, const N2: i32>(_a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcopy_lane_u64::<N1, N2>(_a, b) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_lane_p64<const N1: i32, const N2: i32>(_a: poly64x1_t, b: poly64x1_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vcopy_lane_p64::<N1, N2>(_a, b) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_lane_f64<const N1: i32, const N2: i32>(_a: float64x1_t, b: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vcopy_lane_f64::<N1, N2>(_a, b) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_laneq_s64<const LANE1: i32, const LANE2: i32>(_a: int64x1_t, b: int64x2_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vcopy_laneq_s64::<LANE1, LANE2>(_a, b) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_laneq_u64<const LANE1: i32, const LANE2: i32>(_a: uint64x1_t, b: uint64x2_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vcopy_laneq_u64::<LANE1, LANE2>(_a, b) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_laneq_p64<const LANE1: i32, const LANE2: i32>(_a: poly64x1_t, b: poly64x2_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vcopy_laneq_p64::<LANE1, LANE2>(_a, b) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcopy_laneq_f64<const LANE1: i32, const LANE2: i32>(_a: float64x1_t, b: float64x2_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vcopy_laneq_f64::<LANE1, LANE2>(_a, b) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_s8(ptr: &[i8; 8]) -> int8x8_t {
  unsafe { core::arch::aarch64::vld1_s8(ptr.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_s8(ptr: &[i8; 16]) -> int8x16_t {
  unsafe { core::arch::aarch64::vld1q_s8(ptr.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_s16(ptr: &[i16; 4]) -> int16x4_t {
  unsafe { core::arch::aarch64::vld1_s16(ptr.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_s16(ptr: &[i16; 8]) -> int16x8_t {
  unsafe { core::arch::aarch64::vld1q_s16(ptr.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_s32(ptr: &[i32; 2]) -> int32x2_t {
  unsafe { core::arch::aarch64::vld1_s32(ptr.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_s32(ptr: &[i32; 4]) -> int32x4_t {
  unsafe { core::arch::aarch64::vld1q_s32(ptr.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_s64(ptr: &i64) -> int64x1_t {
  unsafe { core::arch::aarch64::vld1_s64(ptr) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_s64(ptr: &[i64; 2]) -> int64x2_t {
  unsafe { core::arch::aarch64::vld1q_s64(ptr.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_u8(ptr: &[u8; 8]) -> uint8x8_t {
  unsafe { core::arch::aarch64::vld1_u8(ptr.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_u8(ptr: &[u8; 16]) -> uint8x16_t {
  unsafe { core::arch::aarch64::vld1q_u8(ptr.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_u16(ptr: &[u16; 4]) -> uint16x4_t {
  unsafe { core::arch::aarch64::vld1_u16(ptr.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_u16(ptr: &[u16; 8]) -> uint16x8_t {
  unsafe { core::arch::aarch64::vld1q_u16(ptr.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_u32(ptr: &[u32; 2]) -> uint32x2_t {
  unsafe { core::arch::aarch64::vld1_u32(ptr.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_u32(ptr: &[u32; 4]) -> uint32x4_t {
  unsafe { core::arch::aarch64::vld1q_u32(ptr.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_u64(ptr: &u64) -> uint64x1_t {
  unsafe { core::arch::aarch64::vld1_u64(ptr) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_u64(ptr: &[u64; 2]) -> uint64x2_t {
  unsafe { core::arch::aarch64::vld1q_u64(ptr.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_p8(ptr: &[p8; 8]) -> poly8x8_t {
  unsafe { core::arch::aarch64::vld1_p8(ptr.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_p8(ptr: &[p8; 16]) -> poly8x16_t {
  unsafe { core::arch::aarch64::vld1q_p8(ptr.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_p16(ptr: &[p16; 4]) -> poly16x4_t {
  unsafe { core::arch::aarch64::vld1_p16(ptr.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_p16(ptr: &[p16; 8]) -> poly16x8_t {
  unsafe { core::arch::aarch64::vld1q_p16(ptr.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld1_p64(ptr: &p64) -> poly64x1_t {
  unsafe { core::arch::aarch64::vld1_p64(ptr) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vld1q_p64(ptr: &[p64; 2]) -> poly64x2_t {
  unsafe { core::arch::aarch64::vld1q_p64(ptr.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_f32(ptr: &[f32; 2]) -> float32x2_t {
  unsafe { core::arch::aarch64::vld1_f32(ptr.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_f32(ptr: &[f32; 4]) -> float32x4_t {
  unsafe { core::arch::aarch64::vld1q_f32(ptr.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_f64(ptr: &f64) -> float64x1_t {
  unsafe { core::arch::aarch64::vld1_f64(ptr) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_f64(ptr: &[f64; 2]) -> float64x2_t {
  unsafe { core::arch::aarch64::vld1q_f64(ptr.as_ptr()) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_dup_f64(ptr: &f64) -> float64x1_t {
  unsafe { core::arch::aarch64::vld1_dup_f64(ptr) }
}

/// Load multiple single-element structures to one, two, three, or four
/// registers
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_dup_f64(ptr: &f64) -> float64x2_t {
  unsafe { core::arch::aarch64::vld1q_dup_f64(ptr) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1_lane_f64<const LANE: i32>(ptr: &f64, src: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vld1_lane_f64::<LANE>(ptr, src) }
}

/// Load one single-element structure to one lane of one register.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vld1q_lane_f64<const LANE: i32>(ptr: &f64, src: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vld1q_lane_f64::<LANE>(ptr, src) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1_s8(ptr: &mut [i8; 8], a: int8x8_t) {
  unsafe { core::arch::aarch64::vst1_s8(ptr.as_mut_ptr(), a) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1q_s8(ptr: &mut [i8; 16], a: int8x16_t) {
  unsafe { core::arch::aarch64::vst1q_s8(ptr.as_mut_ptr(), a) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1_s16(ptr: &mut [i16; 4], a: int16x4_t) {
  unsafe { core::arch::aarch64::vst1_s16(ptr.as_mut_ptr(), a) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1q_s16(ptr: &mut [i16; 8], a: int16x8_t) {
  unsafe { core::arch::aarch64::vst1q_s16(ptr.as_mut_ptr(), a) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1_s32(ptr: &mut [i32; 2], a: int32x2_t) {
  unsafe { core::arch::aarch64::vst1_s32(ptr.as_mut_ptr(), a) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1q_s32(ptr: &mut [i32; 4], a: int32x4_t) {
  unsafe { core::arch::aarch64::vst1q_s32(ptr.as_mut_ptr(), a) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1_s64(ptr: &mut i64, a: int64x1_t) {
  unsafe { core::arch::aarch64::vst1_s64(ptr, a) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1q_s64(ptr: &mut [i64; 2], a: int64x2_t) {
  unsafe { core::arch::aarch64::vst1q_s64(ptr.as_mut_ptr(), a) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1_u8(ptr: &mut [u8; 8], a: uint8x8_t) {
  unsafe { core::arch::aarch64::vst1_u8(ptr.as_mut_ptr(), a) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1q_u8(ptr: &mut [u8; 16], a: uint8x16_t) {
  unsafe { core::arch::aarch64::vst1q_u8(ptr.as_mut_ptr(), a) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1_u16(ptr: &mut [u16; 4], a: uint16x4_t) {
  unsafe { core::arch::aarch64::vst1_u16(ptr.as_mut_ptr(), a) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1q_u16(ptr: &mut [u16; 8], a: uint16x8_t) {
  unsafe { core::arch::aarch64::vst1q_u16(ptr.as_mut_ptr(), a) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1_u32(ptr: &mut [u32; 2], a: uint32x2_t) {
  unsafe { core::arch::aarch64::vst1_u32(ptr.as_mut_ptr(), a) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1q_u32(ptr: &mut [u32; 4], a: uint32x4_t) {
  unsafe { core::arch::aarch64::vst1q_u32(ptr.as_mut_ptr(), a) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1_u64(ptr: &mut u64, a: uint64x1_t) {
  unsafe { core::arch::aarch64::vst1_u64(ptr, a) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1q_u64(ptr: &mut [u64; 2], a: uint64x2_t) {
  unsafe { core::arch::aarch64::vst1q_u64(ptr.as_mut_ptr(), a) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1_p8(ptr: &mut [p8; 8], a: poly8x8_t) {
  unsafe { core::arch::aarch64::vst1_p8(ptr.as_mut_ptr(), a) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1q_p8(ptr: &mut [p8; 16], a: poly8x16_t) {
  unsafe { core::arch::aarch64::vst1q_p8(ptr.as_mut_ptr(), a) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1_p16(ptr: &mut [p16; 4], a: poly16x4_t) {
  unsafe { core::arch::aarch64::vst1_p16(ptr.as_mut_ptr(), a) }
}

/// Store multiple single-element structures from one, two, three, or four
/// registers.
#[inline]
#[cfg(target_feature = "neon")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1q_p16(ptr: &mut [p16; 8], a: poly16x8_t) {
  unsafe { core::arch::aarch64::vst1q_p16(ptr.as_mut_ptr(), a) }
}

// Store multiple single-element structures from one, two, three, or four
// registers.
#[inline]
#[cfg(target_feature = "neon,aes")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1_p64(ptr: &mut p64, a: poly64x1_t) {
  unsafe { core::arch::aarch64::vst1_p64(ptr, a) }
}

// Store multiple single-element structures from one, two, three, or four
// registers.
#[inline]
#[cfg(target_feature = "neon,aes")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1q_p64(ptr: &mut [p64; 2], a: poly64x2_t) {
  unsafe { core::arch::aarch64::vst1q_p64(ptr.as_mut_ptr(), a) }
}

// Store multiple single-element structures from one, two, three, or four
// registers.
#[inline]
#[cfg(target_feature = "neon")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1_f32(ptr: &mut [f32; 2], a: float32x2_t) {
  unsafe { core::arch::aarch64::vst1_f32(ptr.as_mut_ptr(), a) }
}

// Store multiple single-element structures from one, two, three, or four
// registers.
#[inline]
#[cfg(target_feature = "neon")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1q_f32(ptr: &mut [f32; 4], a: float32x4_t) {
  unsafe { core::arch::aarch64::vst1q_f32(ptr.as_mut_ptr(), a) }
}

// Store multiple single-element structures from one, two, three, or four
// registers.
#[inline]
#[cfg(target_feature = "neon")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1_f64(ptr: &mut f64, a: float64x1_t) {
  unsafe { core::arch::aarch64::vst1_f64(ptr, a) }
}

// Store multiple single-element structures from one, two, three, or four
// registers.
#[inline]
#[cfg(target_feature = "neon")]
#[allow(clippy::cast_ptr_alignment)]
pub fn vst1q_f64(ptr: &mut [f64; 2], a: float64x2_t) {
  unsafe { core::arch::aarch64::vst1q_f64(ptr.as_mut_ptr(), a) }
}

/// Absolute Value (wrapping).
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabsd_s64(a: i64) -> i64 {
  unsafe { core::arch::aarch64::vabsd_s64(a) }
}
/// Absolute Value (wrapping).
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabs_s64(a: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vabs_s64(a) }
}
/// Absolute Value (wrapping).
#[inline]
#[cfg(target_feature = "neon")]
pub fn vabsq_s64(a: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vabsq_s64(a) }
}

/// Bitwise Select instructions. This instruction sets each bit in the
/// destination SIMD&FP register to the corresponding bit from the first source
/// SIMD&FP register when the original destination bit was 1, otherwise from the
/// second source SIMD&FP register.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vbsl_f64(a: uint64x1_t, b: float64x1_t, c: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vbsl_f64(a, b, c) }
}
/// Bitwise Select.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vbsl_p64(a: poly64x1_t, b: poly64x1_t, c: poly64x1_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vbsl_p64(a, b, c) }
}
/// Bitwise Select. (128-bit)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vbslq_f64(a: uint64x2_t, b: float64x2_t, c: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vbslq_f64(a, b, c) }
}
/// Bitwise Select. (128-bit)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vbslq_p64(a: poly64x2_t, b: poly64x2_t, c: poly64x2_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vbslq_p64(a, b, c) }
}

/// Signed saturating Accumulate of Unsigned value.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuqadd_s8(a: int8x8_t, b: uint8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vuqadd_s8(a, b) }
}
/// Signed saturating Accumulate of Unsigned value.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuqaddq_s8(a: int8x16_t, b: uint8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vuqaddq_s8(a, b) }
}
/// Signed saturating Accumulate of Unsigned value.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuqadd_s16(a: int16x4_t, b: uint16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vuqadd_s16(a, b) }
}
/// Signed saturating Accumulate of Unsigned value.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuqaddq_s16(a: int16x8_t, b: uint16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vuqaddq_s16(a, b) }
}
/// Signed saturating Accumulate of Unsigned value.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuqadd_s32(a: int32x2_t, b: uint32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vuqadd_s32(a, b) }
}
/// Signed saturating Accumulate of Unsigned value.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuqaddq_s32(a: int32x4_t, b: uint32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vuqaddq_s32(a, b) }
}
/// Signed saturating Accumulate of Unsigned value.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuqadd_s64(a: int64x1_t, b: uint64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vuqadd_s64(a, b) }
}
/// Signed saturating Accumulate of Unsigned value.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vuqaddq_s64(a: int64x2_t, b: uint64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vuqaddq_s64(a, b) }
}

/// Unsigned saturating Accumulate of Signed value.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsqadd_u8(a: uint8x8_t, b: int8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vsqadd_u8(a, b) }
}
/// Unsigned saturating Accumulate of Signed value.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsqaddq_u8(a: uint8x16_t, b: int8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vsqaddq_u8(a, b) }
}
/// Unsigned saturating Accumulate of Signed value.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsqadd_u16(a: uint16x4_t, b: int16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vsqadd_u16(a, b) }
}
/// Unsigned saturating Accumulate of Signed value.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsqaddq_u16(a: uint16x8_t, b: int16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vsqaddq_u16(a, b) }
}
/// Unsigned saturating Accumulate of Signed value.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsqadd_u32(a: uint32x2_t, b: int32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vsqadd_u32(a, b) }
}
/// Unsigned saturating Accumulate of Signed value.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsqaddq_u32(a: uint32x4_t, b: int32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vsqaddq_u32(a, b) }
}
/// Unsigned saturating Accumulate of Signed value.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsqadd_u64(a: uint64x1_t, b: int64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vsqadd_u64(a, b) }
}
/// Unsigned saturating Accumulate of Signed value.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsqaddq_u64(a: uint64x2_t, b: int64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vsqaddq_u64(a, b) }
}

/// Add pairwise
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpaddq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vpaddq_s16(a, b) }
}
/// Add pairwise
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpaddq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vpaddq_u16(a, b) }
}
/// Add pairwise
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpaddq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vpaddq_s32(a, b) }
}
/// Add pairwise
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpaddq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vpaddq_u32(a, b) }
}
/// Add pairwise
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpaddq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vpaddq_s64(a, b) }
}
/// Add pairwise
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpaddq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vpaddq_u64(a, b) }
}
/// Add pairwise
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpaddq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vpaddq_s8(a, b) }
}
/// Add pairwise
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpaddq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vpaddq_u8(a, b) }
}
/// Add pairwise
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpaddd_s64(a: int64x2_t) -> i64 {
  unsafe { core::arch::aarch64::vpaddd_s64(a) }
}
/// Add pairwise
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpaddd_u64(a: uint64x2_t) -> u64 {
  unsafe { core::arch::aarch64::vpaddd_u64(a) }
}

/// Add across vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddv_s16(a: int16x4_t) -> i16 {
  unsafe { core::arch::aarch64::vaddv_s16(a) }
}

/// Add across vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddv_s32(a: int32x2_t) -> i32 {
  unsafe { core::arch::aarch64::vaddv_s32(a) }
}

/// Add across vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddv_s8(a: int8x8_t) -> i8 {
  unsafe { core::arch::aarch64::vaddv_s8(a) }
}

/// Add across vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddv_u16(a: uint16x4_t) -> u16 {
  unsafe { core::arch::aarch64::vaddv_u16(a) }
}

/// Add across vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddv_u32(a: uint32x2_t) -> u32 {
  unsafe { core::arch::aarch64::vaddv_u32(a) }
}

/// Add across vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddv_u8(a: uint8x8_t) -> u8 {
  unsafe { core::arch::aarch64::vaddv_u8(a) }
}

/// Add across vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddvq_s16(a: int16x8_t) -> i16 {
  unsafe { core::arch::aarch64::vaddvq_s16(a) }
}

/// Add across vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddvq_s32(a: int32x4_t) -> i32 {
  unsafe { core::arch::aarch64::vaddvq_s32(a) }
}

/// Add across vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddvq_s8(a: int8x16_t) -> i8 {
  unsafe { core::arch::aarch64::vaddvq_s8(a) }
}

/// Add across vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddvq_u16(a: uint16x8_t) -> u16 {
  unsafe { core::arch::aarch64::vaddvq_u16(a) }
}

/// Add across vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddvq_u32(a: uint32x4_t) -> u32 {
  unsafe { core::arch::aarch64::vaddvq_u32(a) }
}

/// Add across vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddvq_u8(a: uint8x16_t) -> u8 {
  unsafe { core::arch::aarch64::vaddvq_u8(a) }
}

/// Add across vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddvq_s64(a: int64x2_t) -> i64 {
  unsafe { core::arch::aarch64::vaddvq_s64(a) }
}

/// Add across vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddvq_u64(a: uint64x2_t) -> u64 {
  unsafe { core::arch::aarch64::vaddvq_u64(a) }
}

/// Signed Add Long across Vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddlv_s8(a: int8x8_t) -> i16 {
  unsafe { core::arch::aarch64::vaddlv_s8(a) }
}

/// Signed Add Long across Vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddlvq_s8(a: int8x16_t) -> i16 {
  unsafe { core::arch::aarch64::vaddlvq_s8(a) }
}

/// Unsigned Add Long across Vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddlv_u8(a: uint8x8_t) -> u16 {
  unsafe { core::arch::aarch64::vaddlv_u8(a) }
}

/// Unsigned Add Long across Vector
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddlvq_u8(a: uint8x16_t) -> u16 {
  unsafe { core::arch::aarch64::vaddlvq_u8(a) }
}

/// Vector add.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vadd_f64(a: float64x1_t, b: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vadd_f64(a, b) }
}

/// Vector add.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddq_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vaddq_f64(a, b) }
}

/// Vector add.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vadd_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vadd_s64(a, b) }
}

/// Vector add.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vadd_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vadd_u64(a, b) }
}

/// Vector add.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddd_s64(a: i64, b: i64) -> i64 {
  unsafe { core::arch::aarch64::vaddd_s64(a, b) }
}

/// Vector add.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vaddd_u64(a: u64, b: u64) -> u64 {
  unsafe { core::arch::aarch64::vaddd_u64(a, b) }
}

/// Horizontal vector max.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxv_s8(a: int8x8_t) -> i8 {
  unsafe { core::arch::aarch64::vmaxv_s8(a) }
}

/// Horizontal vector max.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxvq_s8(a: int8x16_t) -> i8 {
  unsafe { core::arch::aarch64::vmaxvq_s8(a) }
}

/// Horizontal vector max.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxv_s16(a: int16x4_t) -> i16 {
  unsafe { core::arch::aarch64::vmaxv_s16(a) }
}

/// Horizontal vector max.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxvq_s16(a: int16x8_t) -> i16 {
  unsafe { core::arch::aarch64::vmaxvq_s16(a) }
}

/// Horizontal vector max.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxv_s32(a: int32x2_t) -> i32 {
  unsafe { core::arch::aarch64::vmaxv_s32(a) }
}

/// Horizontal vector max.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxvq_s32(a: int32x4_t) -> i32 {
  unsafe { core::arch::aarch64::vmaxvq_s32(a) }
}

/// Horizontal vector max.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxv_u8(a: uint8x8_t) -> u8 {
  unsafe { core::arch::aarch64::vmaxv_u8(a) }
}

/// Horizontal vector max.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxvq_u8(a: uint8x16_t) -> u8 {
  unsafe { core::arch::aarch64::vmaxvq_u8(a) }
}

/// Horizontal vector max.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxv_u16(a: uint16x4_t) -> u16 {
  unsafe { core::arch::aarch64::vmaxv_u16(a) }
}

/// Horizontal vector max.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxvq_u16(a: uint16x8_t) -> u16 {
  unsafe { core::arch::aarch64::vmaxvq_u16(a) }
}

/// Horizontal vector max.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxv_u32(a: uint32x2_t) -> u32 {
  unsafe { core::arch::aarch64::vmaxv_u32(a) }
}

/// Horizontal vector max.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxvq_u32(a: uint32x4_t) -> u32 {
  unsafe { core::arch::aarch64::vmaxvq_u32(a) }
}

/// Horizontal vector max.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxv_f32(a: float32x2_t) -> f32 {
  unsafe { core::arch::aarch64::vmaxv_f32(a) }
}

/// Horizontal vector max.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxvq_f32(a: float32x4_t) -> f32 {
  unsafe { core::arch::aarch64::vmaxvq_f32(a) }
}

/// Horizontal vector max.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmaxvq_f64(a: float64x2_t) -> f64 {
  unsafe { core::arch::aarch64::vmaxvq_f64(a) }
}

/// Horizontal vector min.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminv_s8(a: int8x8_t) -> i8 {
  unsafe { core::arch::aarch64::vminv_s8(a) }
}

/// Horizontal vector min.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminvq_s8(a: int8x16_t) -> i8 {
  unsafe { core::arch::aarch64::vminvq_s8(a) }
}

/// Horizontal vector min.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminv_s16(a: int16x4_t) -> i16 {
  unsafe { core::arch::aarch64::vminv_s16(a) }
}

/// Horizontal vector min.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminvq_s16(a: int16x8_t) -> i16 {
  unsafe { core::arch::aarch64::vminvq_s16(a) }
}

/// Horizontal vector min.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminv_s32(a: int32x2_t) -> i32 {
  unsafe { core::arch::aarch64::vminv_s32(a) }
}

/// Horizontal vector min.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminvq_s32(a: int32x4_t) -> i32 {
  unsafe { core::arch::aarch64::vminvq_s32(a) }
}

/// Horizontal vector min.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminv_u8(a: uint8x8_t) -> u8 {
  unsafe { core::arch::aarch64::vminv_u8(a) }
}

/// Horizontal vector min.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminvq_u8(a: uint8x16_t) -> u8 {
  unsafe { core::arch::aarch64::vminvq_u8(a) }
}

/// Horizontal vector min.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminv_u16(a: uint16x4_t) -> u16 {
  unsafe { core::arch::aarch64::vminv_u16(a) }
}

/// Horizontal vector min.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminvq_u16(a: uint16x8_t) -> u16 {
  unsafe { core::arch::aarch64::vminvq_u16(a) }
}

/// Horizontal vector min.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminv_u32(a: uint32x2_t) -> u32 {
  unsafe { core::arch::aarch64::vminv_u32(a) }
}

/// Horizontal vector min.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminvq_u32(a: uint32x4_t) -> u32 {
  unsafe { core::arch::aarch64::vminvq_u32(a) }
}

/// Horizontal vector min.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminv_f32(a: float32x2_t) -> f32 {
  unsafe { core::arch::aarch64::vminv_f32(a) }
}

/// Horizontal vector min.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminvq_f32(a: float32x4_t) -> f32 {
  unsafe { core::arch::aarch64::vminvq_f32(a) }
}

/// Horizontal vector min.
#[inline]
#[cfg(target_feature = "neon")]
pub fn vminvq_f64(a: float64x2_t) -> f64 {
  unsafe { core::arch::aarch64::vminvq_f64(a) }
}

/// Folding minimum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpminq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vpminq_s8(a, b) }
}

/// Folding minimum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpminq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vpminq_s16(a, b) }
}

/// Folding minimum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpminq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vpminq_s32(a, b) }
}

/// Folding minimum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpminq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vpminq_u8(a, b) }
}

/// Folding minimum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpminq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vpminq_u16(a, b) }
}

/// Folding minimum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpminq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vpminq_u32(a, b) }
}

/// Folding minimum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpminq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vpminq_f32(a, b) }
}

/// Folding minimum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpminq_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vpminq_f64(a, b) }
}

/// Folding maximum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpmaxq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vpmaxq_s8(a, b) }
}

/// Folding maximum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpmaxq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vpmaxq_s16(a, b) }
}

/// Folding maximum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpmaxq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vpmaxq_s32(a, b) }
}

/// Folding maximum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpmaxq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vpmaxq_u8(a, b) }
}

/// Folding maximum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpmaxq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vpmaxq_u16(a, b) }
}

/// Folding maximum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpmaxq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vpmaxq_u32(a, b) }
}

/// Folding maximum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpmaxq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vpmaxq_f32(a, b) }
}

/// Folding maximum of adjacent pairs
#[inline]
#[cfg(target_feature = "neon")]
pub fn vpmaxq_f64(a: float64x2_t, b: float64x2_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vpmaxq_f64(a, b) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vext_p64<const N: i32>(a: poly64x1_t, _b: poly64x1_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vext_p64::<N>(a, _b) }
}

/// Extract vector from pair of vectors
#[inline]
#[cfg(target_feature = "neon")]
pub fn vext_f64<const N: i32>(a: float64x1_t, _b: float64x1_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vext_f64::<N>(a, _b) }
}
/// Vector combine
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcombine_s8(low: int8x8_t, high: int8x8_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vcombine_s8(low, high) }
}

/// Vector combine
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcombine_s16(low: int16x4_t, high: int16x4_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vcombine_s16(low, high) }
}

/// Vector combine
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcombine_s32(low: int32x2_t, high: int32x2_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vcombine_s32(low, high) }
}

/// Vector combine
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcombine_s64(low: int64x1_t, high: int64x1_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vcombine_s64(low, high) }
}

/// Vector combine
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcombine_u8(low: uint8x8_t, high: uint8x8_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vcombine_u8(low, high) }
}

/// Vector combine
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcombine_u16(low: uint16x4_t, high: uint16x4_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vcombine_u16(low, high) }
}

/// Vector combine
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcombine_u32(low: uint32x2_t, high: uint32x2_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vcombine_u32(low, high) }
}

/// Vector combine
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcombine_u64(low: uint64x1_t, high: uint64x1_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vcombine_u64(low, high) }
}

/// Vector combine
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcombine_p64(low: poly64x1_t, high: poly64x1_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vcombine_p64(low, high) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_n_p64(value: p64) -> poly64x1_t {
  unsafe { core::arch::aarch64::vdup_n_p64(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdup_n_f64(value: f64) -> float64x1_t {
  unsafe { core::arch::aarch64::vdup_n_f64(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_n_p64(value: p64) -> poly64x2_t {
  unsafe { core::arch::aarch64::vdupq_n_p64(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vdupq_n_f64(value: f64) -> float64x2_t {
  unsafe { core::arch::aarch64::vdupq_n_f64(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmov_n_p64(value: p64) -> poly64x1_t {
  unsafe { core::arch::aarch64::vmov_n_p64(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmov_n_f64(value: f64) -> float64x1_t {
  unsafe { core::arch::aarch64::vmov_n_f64(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmovq_n_p64(value: p64) -> poly64x2_t {
  unsafe { core::arch::aarch64::vmovq_n_p64(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vmovq_n_f64(value: f64) -> float64x2_t {
  unsafe { core::arch::aarch64::vmovq_n_f64(value) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vget_high_f64(a: float64x2_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vget_high_f64(a) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vget_high_p64(a: poly64x2_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vget_high_p64(a) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vget_low_f64(a: float64x2_t) -> float64x1_t {
  unsafe { core::arch::aarch64::vget_low_f64(a) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vget_low_p64(a: poly64x2_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vget_low_p64(a) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vget_lane_f64<const IMM5: i32>(v: float64x1_t) -> f64 {
  unsafe { core::arch::aarch64::vget_lane_f64::<IMM5>(v) }
}

/// Duplicate vector element to vector or scalar
#[inline]
#[cfg(target_feature = "neon")]
pub fn vgetq_lane_f64<const IMM5: i32>(v: float64x2_t) -> f64 {
  unsafe { core::arch::aarch64::vgetq_lane_f64::<IMM5>(v) }
}

/// Vector combine
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcombine_f32(low: float32x2_t, high: float32x2_t) -> float32x4_t {
  unsafe { core::arch::aarch64::vcombine_f32(low, high) }
}

/// Vector combine
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcombine_p8(low: poly8x8_t, high: poly8x8_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vcombine_p8(low, high) }
}

/// Vector combine
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcombine_p16(low: poly16x4_t, high: poly16x4_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vcombine_p16(low, high) }
}

/// Vector combine
#[inline]
#[cfg(target_feature = "neon")]
pub fn vcombine_f64(low: float64x1_t, high: float64x1_t) -> float64x2_t {
  unsafe { core::arch::aarch64::vcombine_f64(low, high) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vtbl1_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vtbl1_s8(a, b) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vtbl1_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vtbl1_u8(a, b) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vtbl1_p8(a: poly8x8_t, b: uint8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vtbl1_p8(a, b) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vtbl2_s8(a: int8x8x2_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vtbl2_s8(a, b) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vtbl2_u8(a: uint8x8x2_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vtbl2_u8(a, b) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vtbl2_p8(a: poly8x8x2_t, b: uint8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vtbl2_p8(a, b) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vtbl3_s8(a: int8x8x3_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vtbl3_s8(a, b) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vtbl3_u8(a: uint8x8x3_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vtbl3_u8(a, b) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vtbl3_p8(a: poly8x8x3_t, b: uint8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vtbl3_p8(a, b) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vtbl4_s8(a: int8x8x4_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vtbl4_s8(a, b) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vtbl4_u8(a: uint8x8x4_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vtbl4_u8(a, b) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vtbl4_p8(a: poly8x8x4_t, b: uint8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vtbl4_p8(a, b) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vtbx1_s8(a: int8x8_t, b: int8x8_t, c: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vtbx1_s8(a, b, c) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vtbx1_u8(a: uint8x8_t, b: uint8x8_t, c: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vtbx1_u8(a, b, c) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vtbx1_p8(a: poly8x8_t, b: poly8x8_t, c: uint8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vtbx1_p8(a, b, c) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vtbx2_s8(a: int8x8_t, b: int8x8x2_t, c: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vtbx2_s8(a, b, c) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vtbx2_u8(a: uint8x8_t, b: uint8x8x2_t, c: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vtbx2_u8(a, b, c) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vtbx2_p8(a: poly8x8_t, b: poly8x8x2_t, c: uint8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vtbx2_p8(a, b, c) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vtbx3_s8(a: int8x8_t, b: int8x8x3_t, c: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vtbx3_s8(a, b, c) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vtbx3_u8(a: uint8x8_t, b: uint8x8x3_t, c: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vtbx3_u8(a, b, c) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vtbx3_p8(a: poly8x8_t, b: poly8x8x3_t, c: uint8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vtbx3_p8(a, b, c) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vtbx4_s8(a: int8x8_t, b: int8x8x4_t, c: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vtbx4_s8(a, b, c) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vtbx4_u8(a: uint8x8_t, b: uint8x8x4_t, c: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vtbx4_u8(a, b, c) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vtbx4_p8(a: poly8x8_t, b: poly8x8x4_t, c: uint8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vtbx4_p8(a, b, c) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbl1_s8(t: int8x16_t, idx: uint8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vqtbl1_s8(t, idx) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbl1q_s8(t: int8x16_t, idx: uint8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vqtbl1q_s8(t, idx) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbl1_u8(t: uint8x16_t, idx: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vqtbl1_u8(t, idx) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbl1q_u8(t: uint8x16_t, idx: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vqtbl1q_u8(t, idx) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbl1_p8(t: poly8x16_t, idx: uint8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vqtbl1_p8(t, idx) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbl1q_p8(t: poly8x16_t, idx: uint8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vqtbl1q_p8(t, idx) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbx1_s8(a: int8x8_t, t: int8x16_t, idx: uint8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vqtbx1_s8(a, t, idx) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbx1q_s8(a: int8x16_t, t: int8x16_t, idx: uint8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vqtbx1q_s8(a, t, idx) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbx1_u8(a: uint8x8_t, t: uint8x16_t, idx: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vqtbx1_u8(a, t, idx) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbx1q_u8(a: uint8x16_t, t: uint8x16_t, idx: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vqtbx1q_u8(a, t, idx) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbx1_p8(a: poly8x8_t, t: poly8x16_t, idx: uint8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vqtbx1_p8(a, t, idx) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbx1q_p8(a: poly8x16_t, t: poly8x16_t, idx: uint8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vqtbx1q_p8(a, t, idx) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbl2_s8(t: int8x16x2_t, idx: uint8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vqtbl2_s8(t, idx) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbl2q_s8(t: int8x16x2_t, idx: uint8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vqtbl2q_s8(t, idx) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbl2_u8(t: uint8x16x2_t, idx: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vqtbl2_u8(t, idx) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbl2q_u8(t: uint8x16x2_t, idx: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vqtbl2q_u8(t, idx) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbl2_p8(t: poly8x16x2_t, idx: uint8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vqtbl2_p8(t, idx) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbl2q_p8(t: poly8x16x2_t, idx: uint8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vqtbl2q_p8(t, idx) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbx2_s8(a: int8x8_t, t: int8x16x2_t, idx: uint8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vqtbx2_s8(a, t, idx) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbx2q_s8(a: int8x16_t, t: int8x16x2_t, idx: uint8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vqtbx2q_s8(a, t, idx) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbx2_u8(a: uint8x8_t, t: uint8x16x2_t, idx: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vqtbx2_u8(a, t, idx) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbx2q_u8(a: uint8x16_t, t: uint8x16x2_t, idx: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vqtbx2q_u8(a, t, idx) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbx2_p8(a: poly8x8_t, t: poly8x16x2_t, idx: uint8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vqtbx2_p8(a, t, idx) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbx2q_p8(a: poly8x16_t, t: poly8x16x2_t, idx: uint8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vqtbx2q_p8(a, t, idx) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbl3_s8(t: int8x16x3_t, idx: uint8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vqtbl3_s8(t, idx) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbl3q_s8(t: int8x16x3_t, idx: uint8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vqtbl3q_s8(t, idx) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbl3_u8(t: uint8x16x3_t, idx: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vqtbl3_u8(t, idx) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbl3q_u8(t: uint8x16x3_t, idx: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vqtbl3q_u8(t, idx) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbl3_p8(t: poly8x16x3_t, idx: uint8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vqtbl3_p8(t, idx) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbl3q_p8(t: poly8x16x3_t, idx: uint8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vqtbl3q_p8(t, idx) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbx3_s8(a: int8x8_t, t: int8x16x3_t, idx: uint8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vqtbx3_s8(a, t, idx) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbx3q_s8(a: int8x16_t, t: int8x16x3_t, idx: uint8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vqtbx3q_s8(a, t, idx) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbx3_u8(a: uint8x8_t, t: uint8x16x3_t, idx: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vqtbx3_u8(a, t, idx) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbx3q_u8(a: uint8x16_t, t: uint8x16x3_t, idx: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vqtbx3q_u8(a, t, idx) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbx3_p8(a: poly8x8_t, t: poly8x16x3_t, idx: uint8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vqtbx3_p8(a, t, idx) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbx3q_p8(a: poly8x16_t, t: poly8x16x3_t, idx: uint8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vqtbx3q_p8(a, t, idx) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbl4_s8(t: int8x16x4_t, idx: uint8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vqtbl4_s8(t, idx) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbl4q_s8(t: int8x16x4_t, idx: uint8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vqtbl4q_s8(t, idx) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbl4_u8(t: uint8x16x4_t, idx: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vqtbl4_u8(t, idx) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbl4q_u8(t: uint8x16x4_t, idx: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vqtbl4q_u8(t, idx) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbl4_p8(t: poly8x16x4_t, idx: uint8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vqtbl4_p8(t, idx) }
}

/// Table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbl4q_p8(t: poly8x16x4_t, idx: uint8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vqtbl4q_p8(t, idx) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbx4_s8(a: int8x8_t, t: int8x16x4_t, idx: uint8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vqtbx4_s8(a, t, idx) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbx4q_s8(a: int8x16_t, t: int8x16x4_t, idx: uint8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vqtbx4q_s8(a, t, idx) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbx4_u8(a: uint8x8_t, t: uint8x16x4_t, idx: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vqtbx4_u8(a, t, idx) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbx4q_u8(a: uint8x16_t, t: uint8x16x4_t, idx: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vqtbx4q_u8(a, t, idx) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbx4_p8(a: poly8x8_t, t: poly8x16x4_t, idx: uint8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vqtbx4_p8(a, t, idx) }
}

/// Extended table look-up
#[inline]
#[cfg(target_endian = "little")]
#[cfg(target_feature = "neon")]
pub fn vqtbx4q_p8(a: poly8x16_t, t: poly8x16x4_t, idx: uint8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vqtbx4q_p8(a, t, idx) }
}

/// Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshld_n_s64<const N: i32>(a: i64) -> i64 {
  unsafe { core::arch::aarch64::vshld_n_s64::<N>(a) }
}

/// Shift left
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshld_n_u64<const N: i32>(a: u64) -> u64 {
  unsafe { core::arch::aarch64::vshld_n_u64::<N>(a) }
}

/// Signed shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshrd_n_s64<const N: i32>(a: i64) -> i64 {
  unsafe { core::arch::aarch64::vshrd_n_s64::<N>(a) }
}

/// Unsigned shift right
#[inline]
#[cfg(target_feature = "neon")]
pub fn vshrd_n_u64<const N: i32>(a: u64) -> u64 {
  unsafe { core::arch::aarch64::vshrd_n_u64::<N>(a) }
}

/// Signed shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsrad_n_s64<const N: i32>(a: i64, b: i64) -> i64 {
  unsafe { core::arch::aarch64::vsrad_n_s64::<N>(a, b) }
}

/// Unsigned shift right and accumulate
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsrad_n_u64<const N: i32>(a: u64, b: u64) -> u64 {
  unsafe { core::arch::aarch64::vsrad_n_u64::<N>(a, b) }
}

/// Shift Left and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsli_n_s8<const N: i32>(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vsli_n_s8::<N>(a, b) }
}
/// Shift Left and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsliq_n_s8<const N: i32>(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vsliq_n_s8::<N>(a, b) }
}
/// Shift Left and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsli_n_s16<const N: i32>(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vsli_n_s16::<N>(a, b) }
}
/// Shift Left and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsliq_n_s16<const N: i32>(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vsliq_n_s16::<N>(a, b) }
}
/// Shift Left and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsli_n_s32<const N: i32>(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vsli_n_s32::<N>(a, b) }
}
/// Shift Left and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsliq_n_s32<const N: i32>(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vsliq_n_s32::<N>(a, b) }
}
/// Shift Left and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsli_n_s64<const N: i32>(a: int64x1_t, b: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vsli_n_s64::<N>(a, b) }
}
/// Shift Left and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsliq_n_s64<const N: i32>(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vsliq_n_s64::<N>(a, b) }
}
/// Shift Left and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsli_n_u8<const N: i32>(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vsli_n_u8::<N>(a, b) }
}
/// Shift Left and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsliq_n_u8<const N: i32>(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vsliq_n_u8::<N>(a, b) }
}
/// Shift Left and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsli_n_u16<const N: i32>(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vsli_n_u16::<N>(a, b) }
}
/// Shift Left and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsliq_n_u16<const N: i32>(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vsliq_n_u16::<N>(a, b) }
}
/// Shift Left and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsli_n_u32<const N: i32>(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vsli_n_u32::<N>(a, b) }
}
/// Shift Left and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsliq_n_u32<const N: i32>(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vsliq_n_u32::<N>(a, b) }
}
/// Shift Left and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsli_n_u64<const N: i32>(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vsli_n_u64::<N>(a, b) }
}
/// Shift Left and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsliq_n_u64<const N: i32>(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vsliq_n_u64::<N>(a, b) }
}
/// Shift Left and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsli_n_p8<const N: i32>(a: poly8x8_t, b: poly8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vsli_n_p8::<N>(a, b) }
}
/// Shift Left and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsliq_n_p8<const N: i32>(a: poly8x16_t, b: poly8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vsliq_n_p8::<N>(a, b) }
}
/// Shift Left and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsli_n_p16<const N: i32>(a: poly16x4_t, b: poly16x4_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vsli_n_p16::<N>(a, b) }
}
/// Shift Left and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsliq_n_p16<const N: i32>(a: poly16x8_t, b: poly16x8_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vsliq_n_p16::<N>(a, b) }
}
/// Shift Left and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vsli_n_p64<const N: i32>(a: poly64x1_t, b: poly64x1_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vsli_n_p64::<N>(a, b) }
}
/// Shift Left and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vsliq_n_p64<const N: i32>(a: poly64x2_t, b: poly64x2_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vsliq_n_p64::<N>(a, b) }
}
/// Shift Right and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsri_n_s8<const N: i32>(a: int8x8_t, b: int8x8_t) -> int8x8_t {
  unsafe { core::arch::aarch64::vsri_n_s8::<N>(a, b) }
}
/// Shift Right and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsriq_n_s8<const N: i32>(a: int8x16_t, b: int8x16_t) -> int8x16_t {
  unsafe { core::arch::aarch64::vsriq_n_s8::<N>(a, b) }
}
/// Shift Right and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsri_n_s16<const N: i32>(a: int16x4_t, b: int16x4_t) -> int16x4_t {
  unsafe { core::arch::aarch64::vsri_n_s16::<N>(a, b) }
}
/// Shift Right and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsriq_n_s16<const N: i32>(a: int16x8_t, b: int16x8_t) -> int16x8_t {
  unsafe { core::arch::aarch64::vsriq_n_s16::<N>(a, b) }
}
/// Shift Right and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsri_n_s32<const N: i32>(a: int32x2_t, b: int32x2_t) -> int32x2_t {
  unsafe { core::arch::aarch64::vsri_n_s32::<N>(a, b) }
}
/// Shift Right and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsriq_n_s32<const N: i32>(a: int32x4_t, b: int32x4_t) -> int32x4_t {
  unsafe { core::arch::aarch64::vsriq_n_s32::<N>(a, b) }
}
/// Shift Right and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsri_n_s64<const N: i32>(a: int64x1_t, b: int64x1_t) -> int64x1_t {
  unsafe { core::arch::aarch64::vsri_n_s64::<N>(a, b) }
}
/// Shift Right and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsriq_n_s64<const N: i32>(a: int64x2_t, b: int64x2_t) -> int64x2_t {
  unsafe { core::arch::aarch64::vsriq_n_s64::<N>(a, b) }
}
/// Shift Right and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsri_n_u8<const N: i32>(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
  unsafe { core::arch::aarch64::vsri_n_u8::<N>(a, b) }
}
/// Shift Right and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsriq_n_u8<const N: i32>(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
  unsafe { core::arch::aarch64::vsriq_n_u8::<N>(a, b) }
}
/// Shift Right and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsri_n_u16<const N: i32>(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
  unsafe { core::arch::aarch64::vsri_n_u16::<N>(a, b) }
}
/// Shift Right and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsriq_n_u16<const N: i32>(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
  unsafe { core::arch::aarch64::vsriq_n_u16::<N>(a, b) }
}
/// Shift Right and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsri_n_u32<const N: i32>(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
  unsafe { core::arch::aarch64::vsri_n_u32::<N>(a, b) }
}
/// Shift Right and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsriq_n_u32<const N: i32>(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
  unsafe { core::arch::aarch64::vsriq_n_u32::<N>(a, b) }
}
/// Shift Right and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsri_n_u64<const N: i32>(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
  unsafe { core::arch::aarch64::vsri_n_u64::<N>(a, b) }
}
/// Shift Right and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsriq_n_u64<const N: i32>(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
  unsafe { core::arch::aarch64::vsriq_n_u64::<N>(a, b) }
}
/// Shift Right and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsri_n_p8<const N: i32>(a: poly8x8_t, b: poly8x8_t) -> poly8x8_t {
  unsafe { core::arch::aarch64::vsri_n_p8::<N>(a, b) }
}
/// Shift Right and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsriq_n_p8<const N: i32>(a: poly8x16_t, b: poly8x16_t) -> poly8x16_t {
  unsafe { core::arch::aarch64::vsriq_n_p8::<N>(a, b) }
}
/// Shift Right and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsri_n_p16<const N: i32>(a: poly16x4_t, b: poly16x4_t) -> poly16x4_t {
  unsafe { core::arch::aarch64::vsri_n_p16::<N>(a, b) }
}
/// Shift Right and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon")]
pub fn vsriq_n_p16<const N: i32>(a: poly16x8_t, b: poly16x8_t) -> poly16x8_t {
  unsafe { core::arch::aarch64::vsriq_n_p16::<N>(a, b) }
}
/// Shift Right and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vsri_n_p64<const N: i32>(a: poly64x1_t, b: poly64x1_t) -> poly64x1_t {
  unsafe { core::arch::aarch64::vsri_n_p64::<N>(a, b) }
}
/// Shift Right and Insert (immediate)
#[inline]
#[cfg(target_feature = "neon,aes")]
pub fn vsriq_n_p64<const N: i32>(a: poly64x2_t, b: poly64x2_t) -> poly64x2_t {
  unsafe { core::arch::aarch64::vsriq_n_p64::<N>(a, b) }
}
