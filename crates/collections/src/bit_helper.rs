/// ABOUT:
/// Helps with operations that rely on bit marking to indicate whether an item in the
/// collection should be added, removed, visited already, etc.
///
/// BitHelper doesn't allocate the array; you must pass in a Vec<u32> or slice of u32s.
/// `to_int_array_length()` tells you the u32 array size you must allocate.
///
/// USAGE:
/// Suppose you need to represent a bit array of length (i.e. logical bit array length)
/// BIT_ARRAY_LENGTH. Then this is the suggested way to instantiate BitHelper:
/// ***************************************************************************
/// let int_array_length = BitHelper::to_int_array_length(BIT_ARRAY_LENGTH);
/// let mut array = vec![0u32; int_array_length];
/// let bit_helper = BitHelper::new(&mut array);
/// ***************************************************************************
///
/// IMPORTANT:
/// The length passed to `new()` should be specified as the length of the u32 array, not
/// the logical bit array. Because length is used for bounds checking into the u32 array,
/// it's especially important to get this correct. See the code samples above; this is
/// the value gotten from `to_int_array_length()`.
///
/// The length constructor argument is the only exception; for other methods -- `mark_bit`
/// and `is_marked` -- pass in values as indices into the logical bit array, and it will
/// be mapped to the position within the array of u32s.
pub struct BitHelper<'a> {
    /// Length of underlying u32 array (not logical bit array)
    length: usize,
    /// Reference to array of u32s
    array: &'a mut [u32],
}

const MARKED_BIT_FLAG: u32 = 1;
const INT_BIT_SIZE: usize = 32;

impl<'a> BitHelper<'a> {
    /// Instantiates a BitHelper with a slice of u32s
    ///
    /// # Arguments
    /// * `array` - Slice of u32s to hold bits
    ///
    /// # Panics
    /// Panics if the array is empty
    pub fn new(array: &'a mut [u32]) -> Self {
        assert!(!array.is_empty(), "Array must not be empty");
        Self {
            length: array.len(),
            array,
        }
    }

    /// Mark bit at specified position
    ///
    /// # Arguments
    /// * `bit_position` - Position in the logical bit array to mark
    pub fn mark_bit(&mut self, bit_position: usize) {
        let bit_array_index = bit_position / INT_BIT_SIZE;
        if bit_array_index < self.length {
            let bit_offset = bit_position % INT_BIT_SIZE;
            self.array[bit_array_index] |= MARKED_BIT_FLAG << bit_offset;
        }
    }

    /// Check if bit at specified position is marked
    ///
    /// # Arguments
    /// * `bit_position` - Position in the logical bit array to check
    ///
    /// # Returns
    /// `true` if the bit is marked, `false` otherwise
    pub fn is_marked(&self, bit_position: usize) -> bool {
        let bit_array_index = bit_position / INT_BIT_SIZE;
        if bit_array_index < self.length {
            let bit_offset = bit_position % INT_BIT_SIZE;
            (self.array[bit_array_index] & (MARKED_BIT_FLAG << bit_offset)) != 0
        } else {
            false
        }
    }

    /// Copy a block of bits from a byte array
    ///
    /// # Arguments
    /// * `bytes` - Source byte array
    /// * `offset` - Bit offset to start copying from
    /// * `length` - Number of bits to copy
    ///
    /// # Returns
    /// A new byte array containing the copied bits
    ///
    /// # Panics
    /// Panics if offset or length are out of bounds
    pub fn copy_block(bytes: &[u8], offset: usize, length: usize) -> Vec<u8> {
        if offset >= bytes.len() * 8 {
            panic!("offset out of range");
        }
        if length == 0 {
            return Vec::new();
        }
        if offset + length > bytes.len() * 8 {
            panic!("offset + length out of range");
        }

        let start_byte = offset / 8;
        let end_byte = (offset + length - 1) / 8;
        let shift_a = offset % 8;
        let shift_b = 8 - shift_a;
        let mut dst = vec![0u8; (length + 7) / 8];

        if shift_a == 0 {
            let copy_len = dst.len().min(bytes.len() - start_byte);
            dst[..copy_len].copy_from_slice(&bytes[start_byte..start_byte + copy_len]);
        } else {
            let mut i = 0;
            while i < end_byte - start_byte {
                dst[i] =
                    (bytes[start_byte + i] << shift_a) | (bytes[start_byte + i + 1] >> shift_b);
                i += 1;
            }

            if i < dst.len() {
                dst[i] = bytes[start_byte + i] << shift_a;
            }
        }

        let dst_len = dst.len();
        let last_byte_bits = (dst_len * 8) - length;
        if last_byte_bits > 0 && last_byte_bits < 8 {
            dst[dst_len - 1] &= 0xFF << last_byte_bits;
        }
        dst
    }

    /// Copy bytes from source to destination at specified offset
    ///
    /// # Arguments
    /// * `dst` - Destination byte array
    /// * `dst_offset` - Offset in destination to start copying
    /// * `src` - Source byte array
    ///
    /// # Panics
    /// Panics if dst_offset + src.len() exceeds dst.len()
    pub fn copy_bytes(dst: &mut [u8], dst_offset: usize, src: &[u8]) {
        if dst_offset + src.len() > dst.len() {
            panic!("destination buffer too small");
        }
        dst[dst_offset..dst_offset + src.len()].copy_from_slice(src);
    }

    /// Read bits from a u64 value
    ///
    /// # Arguments
    /// * `x` - Mutable reference to u64 value
    /// * `length` - Number of bits to read
    ///
    /// # Returns
    /// The bits read as a u64 (right-aligned)
    ///
    /// # Note
    /// This function modifies `x` by shifting it left by `length` bits
    pub fn read(x: &mut u64, length: usize) -> u64 {
        let r = *x >> (64 - length);
        *x <<= length;
        r
    }

    /// Read bits from a byte array
    ///
    /// # Arguments
    /// * `bytes` - Source byte array
    /// * `offset` - Mutable reference to bit offset (will be updated)
    /// * `length` - Number of bits to read
    ///
    /// # Returns
    /// The bits read as a u64 (right-aligned)
    ///
    /// # Panics
    /// Panics if offset or length are out of bounds
    pub fn read_from_bytes(bytes: &[u8], offset: &mut usize, length: usize) -> u64 {
        if *offset >= bytes.len() * 8 {
            panic!("offset out of range");
        }
        if length == 0 {
            return 0;
        }
        if *offset + length > bytes.len() * 8 {
            panic!("offset + length out of range");
        }

        let start_byte = *offset / 8;
        let end_byte = (*offset + length - 1) / 8;
        let skip_bits = *offset % 8;
        let mut bits = 0u64;

        let max_bytes = (end_byte - start_byte + 1).min(8);
        for i in 0..max_bytes {
            if start_byte + i < bytes.len() {
                bits |= (bytes[start_byte + i] as u64) << (56 - i * 8);
            }
        }

        if skip_bits != 0 {
            Self::read(&mut bits, skip_bits);
        }
        *offset += length;
        Self::read(&mut bits, length)
    }

    /// Write bits to a u64 value
    ///
    /// # Arguments
    /// * `x` - Mutable reference to u64 value
    /// * `length` - Number of bits to write
    /// * `value` - Value to write (only the lower `length` bits will be used)
    ///
    /// # Note
    /// This function modifies `x` by shifting it left and ORing the value
    pub fn write(x: &mut u64, length: usize, value: u64) {
        let mask = 0xFFFFFFFFFFFFFFFFu64 >> (64 - length);
        *x = (*x << length) | (value & mask);
    }

    /// Get bit size for i8
    pub fn get_bit_size(value: i8) -> usize {
        if value < 1 { 0 } else { value as usize * 8 }
    }

    /// Get bit size for u8
    pub fn get_bit_size_u8(value: u8) -> usize {
        if value < 1 { 0 } else { value as usize * 8 }
    }

    /// Get bit size for i16
    pub fn get_bit_size_i16(value: i16) -> usize {
        if value < 1 { 0 } else { value as usize * 8 }
    }

    /// Get bit size for u16
    pub fn get_bit_size_u16(value: u16) -> usize {
        if value < 1 { 0 } else { value as usize * 8 }
    }

    /// Get bit size for i32
    pub fn get_bit_size_i32(value: i32) -> usize {
        if value < 1 { 0 } else { value as usize * 8 }
    }

    /// Get bit size for u32
    pub fn get_bit_size_u32(value: u32) -> usize {
        if value < 1 { 0 } else { value as usize * 8 }
    }

    /// Get bit size for i64
    pub fn get_bit_size_i64(value: i64) -> usize {
        if value < 1 { 0 } else { value as usize * 8 }
    }

    /// Get bit size for u64
    pub fn get_bit_size_u64(value: u64) -> usize {
        if value < 1 { 0 } else { value as usize * 8 }
    }

    /// Get bit size for i128
    pub fn get_bit_size_i128(value: i128) -> usize {
        if value < 1 { 0 } else { value as usize * 8 }
    }

    /// Get bit size for u128
    pub fn get_bit_size_u128(value: u128) -> usize {
        if value < 1 { 0 } else { value as usize * 8 }
    }

    /// Get bit size for isize
    pub fn get_bit_size_isize(value: isize) -> usize {
        if value < 1 { 0 } else { value as usize * 8 }
    }

    /// Get bit size for usize
    pub fn get_bit_size_usize(value: usize) -> usize {
        if value < 1 { 0 } else { value * 8 }
    }

    /// Calculate the length of u32 array needed to hold `n` bits
    ///
    /// # Arguments
    /// * `n` - Number of bits needed
    ///
    /// # Returns
    /// The number of u32s needed to hold `n` bits
    pub fn to_int_array_length(n: usize) -> usize {
        if n > 0 { (n - 1) / INT_BIT_SIZE + 1 } else { 0 }
    }
}
