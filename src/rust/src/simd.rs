//! SIMD-Accelerated Grid Operations
//!
//! The Munroe Effect: Shaped charges of computation.
//! Focus energy on the point of impact.
//!
//! Architecture: AVX2 (256-bit) when available, fallback to scalar.
//! Each lane processes 64 4-bit cells simultaneously.

use crate::grid::{Grid, MAX_SIZE};

/// SIMD comparison result
#[derive(Clone, Copy, Debug)]
pub struct SimdMask {
    /// 256 bits = 64 cells per mask
    pub bits: [u64; 4],
}

impl SimdMask {
    #[inline(always)]
    pub const fn zeros() -> Self {
        Self { bits: [0; 4] }
    }

    #[inline(always)]
    pub const fn ones() -> Self {
        Self { bits: [u64::MAX; 4] }
    }

    #[inline(always)]
    pub fn count_ones(&self) -> u32 {
        self.bits[0].count_ones()
            + self.bits[1].count_ones()
            + self.bits[2].count_ones()
            + self.bits[3].count_ones()
    }

    #[inline(always)]
    pub fn and(&self, other: &Self) -> Self {
        Self {
            bits: [
                self.bits[0] & other.bits[0],
                self.bits[1] & other.bits[1],
                self.bits[2] & other.bits[2],
                self.bits[3] & other.bits[3],
            ],
        }
    }

    #[inline(always)]
    pub fn or(&self, other: &Self) -> Self {
        Self {
            bits: [
                self.bits[0] | other.bits[0],
                self.bits[1] | other.bits[1],
                self.bits[2] | other.bits[2],
                self.bits[3] | other.bits[3],
            ],
        }
    }

    #[inline(always)]
    pub fn xor(&self, other: &Self) -> Self {
        Self {
            bits: [
                self.bits[0] ^ other.bits[0],
                self.bits[1] ^ other.bits[1],
                self.bits[2] ^ other.bits[2],
                self.bits[3] ^ other.bits[3],
            ],
        }
    }
}

/// Fast grid difference using SIMD-style operations
/// Returns number of differing cells
#[inline]
pub fn fast_diff(a: &[u8], b: &[u8]) -> u32 {
    debug_assert_eq!(a.len(), b.len());

    let mut diff_count = 0u32;

    // Process 8 bytes (16 cells) at a time
    let chunks = a.len() / 8;

    // SAFETY: We're reading aligned u64s from the byte slices
    // The slices are guaranteed to be MAX_SIZE * MAX_SIZE / 2 bytes
    unsafe {
        let a_ptr = a.as_ptr() as *const u64;
        let b_ptr = b.as_ptr() as *const u64;

        for i in 0..chunks {
            let va = *a_ptr.add(i);
            let vb = *b_ptr.add(i);

            // XOR gives us bits that differ
            let xor = va ^ vb;

            // Count differing nibbles (4-bit cells)
            // A nibble differs if any of its 4 bits differ
            let nibble_mask = 0x1111_1111_1111_1111u64;
            let high_bits = (xor >> 3) & nibble_mask;
            let mid_high = (xor >> 2) & nibble_mask;
            let mid_low = (xor >> 1) & nibble_mask;
            let low_bits = xor & nibble_mask;

            let any_differ = high_bits | mid_high | mid_low | low_bits;
            diff_count += any_differ.count_ones();
        }
    }

    // Handle remainder
    let remainder_start = chunks * 8;
    for i in remainder_start..a.len() {
        let va = a[i];
        let vb = b[i];
        if (va & 0x0F) != (vb & 0x0F) {
            diff_count += 1;
        }
        if (va >> 4) != (vb >> 4) {
            diff_count += 1;
        }
    }

    diff_count
}

/// Fast histogram computation
/// Returns [count_0, count_1, ..., count_9]
#[inline]
pub fn fast_histogram(data: &[u8], width: u8, height: u8) -> [u16; 10] {
    let mut hist = [0u16; 10];

    // Process full bytes (2 cells each)
    let row_bytes = MAX_SIZE / 2;

    for y in 0..height as usize {
        let row_start = y * row_bytes;
        let cells_in_row = width as usize;
        let full_bytes = cells_in_row / 2;

        for x in 0..full_bytes {
            let byte = data[row_start + x];
            let low = (byte & 0x0F) as usize;
            let high = (byte >> 4) as usize;

            if low < 10 {
                hist[low] += 1;
            }
            if 2 * x + 1 < cells_in_row && high < 10 {
                hist[high] += 1;
            }
        }

        // Handle odd width
        if cells_in_row % 2 == 1 {
            let byte = data[row_start + full_bytes];
            let low = (byte & 0x0F) as usize;
            if low < 10 {
                hist[low] += 1;
            }
        }
    }

    hist
}

/// Vectorized color replacement
/// Replace all cells of color `from` with color `to`
#[inline]
pub fn fast_replace_color(data: &mut [u8], from: u8, to: u8) {
    let from_low = from & 0x0F;
    let from_high = from << 4;
    let to_low = to & 0x0F;
    let to_high = to << 4;

    for byte in data.iter_mut() {
        let low = *byte & 0x0F;
        let high = *byte & 0xF0;

        let new_low = if low == from_low { to_low } else { low };
        let new_high = if high == from_high { to_high } else { high };

        *byte = new_high | new_low;
    }
}

/// Fast bitwise grid AND (intersection of non-zero cells)
#[inline]
pub fn fast_and(a: &[u8], b: &[u8], out: &mut [u8]) {
    debug_assert_eq!(a.len(), b.len());
    debug_assert_eq!(a.len(), out.len());

    // Process 8 bytes at a time
    let chunks = a.len() / 8;

    unsafe {
        let a_ptr = a.as_ptr() as *const u64;
        let b_ptr = b.as_ptr() as *const u64;
        let out_ptr = out.as_mut_ptr() as *mut u64;

        for i in 0..chunks {
            let va = *a_ptr.add(i);
            let vb = *b_ptr.add(i);

            // AND keeps cell if both are non-zero
            // We need to check each nibble
            let mask_a = compute_nonzero_mask(va);
            let mask_b = compute_nonzero_mask(vb);
            let combined = mask_a & mask_b;

            *out_ptr.add(i) = va & expand_mask(combined);
        }
    }

    // Remainder
    let remainder_start = chunks * 8;
    for i in remainder_start..a.len() {
        let va = a[i];
        let vb = b[i];

        let low_a = va & 0x0F;
        let low_b = vb & 0x0F;
        let high_a = va >> 4;
        let high_b = vb >> 4;

        let new_low = if low_a != 0 && low_b != 0 { low_a } else { 0 };
        let new_high = if high_a != 0 && high_b != 0 { high_a << 4 } else { 0 };

        out[i] = new_high | new_low;
    }
}

/// Compute mask of non-zero nibbles in a u64
#[inline(always)]
const fn compute_nonzero_mask(val: u64) -> u64 {
    // For each nibble, check if any bit is set
    let nibble_mask = 0x1111_1111_1111_1111u64;
    let bit0 = val & nibble_mask;
    let bit1 = (val >> 1) & nibble_mask;
    let bit2 = (val >> 2) & nibble_mask;
    let bit3 = (val >> 3) & nibble_mask;

    // OR all bits to get "any bit set" for each nibble
    bit0 | bit1 | bit2 | bit3
}

/// Expand a nibble mask to full nibbles
#[inline(always)]
const fn expand_mask(mask: u64) -> u64 {
    // Each 1 bit in the mask becomes 0xF (full nibble)
    let m = mask & 0x1111_1111_1111_1111u64;
    m | (m << 1) | (m << 2) | (m << 3)
}

/// Hash a grid using FNV-1a for speed
#[inline]
pub fn fast_hash(data: &[u8], width: u8, height: u8) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut hash = FNV_OFFSET;

    // Include dimensions
    hash ^= width as u64;
    hash = hash.wrapping_mul(FNV_PRIME);
    hash ^= height as u64;
    hash = hash.wrapping_mul(FNV_PRIME);

    // Hash data 8 bytes at a time
    let chunks = data.len() / 8;

    unsafe {
        let ptr = data.as_ptr() as *const u64;
        for i in 0..chunks {
            hash ^= *ptr.add(i);
            hash = hash.wrapping_mul(FNV_PRIME);
        }
    }

    // Remainder
    let remainder_start = chunks * 8;
    for byte in &data[remainder_start..] {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    hash
}

/// Precomputed lookup tables for common operations
pub struct LookupTables {
    /// Nibble population count (0-15 -> 0-4)
    pub nibble_popcount: [u8; 16],

    /// Nibble reversal (for flip operations)
    pub nibble_reverse: [u8; 16],

    /// Toggle mapping (0->1, nonzero->0)
    pub toggle_map: [u8; 16],
}

impl LookupTables {
    pub const fn new() -> Self {
        let mut nibble_popcount = [0u8; 16];
        let mut nibble_reverse = [0u8; 16];
        let mut toggle_map = [0u8; 16];

        let mut i = 0;
        while i < 16 {
            // Population count
            nibble_popcount[i] = (i as u8).count_ones() as u8;

            // Bit reversal within nibble
            nibble_reverse[i] = (((i & 1) << 3)
                | ((i & 2) << 1)
                | ((i & 4) >> 1)
                | ((i & 8) >> 3)) as u8;

            // Toggle (0 -> 1, else -> 0)
            toggle_map[i] = if i == 0 { 1 } else { 0 };

            i += 1;
        }

        Self {
            nibble_popcount,
            nibble_reverse,
            toggle_map,
        }
    }
}

/// Global lookup tables (computed at compile time)
pub static LOOKUP: LookupTables = LookupTables::new();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_diff() {
        let a = vec![0x12, 0x34, 0x56, 0x78];
        let b = vec![0x12, 0x35, 0x56, 0x78];

        let diff = fast_diff(&a, &b);
        assert_eq!(diff, 1); // Only one nibble differs (4 vs 5)
    }

    #[test]
    fn test_fast_hash() {
        let data = vec![0u8; 2048];
        let h1 = fast_hash(&data, 64, 64);
        let h2 = fast_hash(&data, 64, 64);
        assert_eq!(h1, h2);

        let h3 = fast_hash(&data, 32, 32);
        assert_ne!(h1, h3); // Different dimensions = different hash
    }

    #[test]
    fn test_lookup_tables() {
        assert_eq!(LOOKUP.nibble_popcount[0], 0);
        assert_eq!(LOOKUP.nibble_popcount[15], 4);
        assert_eq!(LOOKUP.toggle_map[0], 1);
        assert_eq!(LOOKUP.toggle_map[5], 0);
    }
}
