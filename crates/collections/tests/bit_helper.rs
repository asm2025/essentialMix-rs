use emixcollections::bit_helper::BitHelper;

#[test]
fn test_to_int_array_length() {
    assert_eq!(BitHelper::to_int_array_length(0), 0);
    assert_eq!(BitHelper::to_int_array_length(1), 1);
    assert_eq!(BitHelper::to_int_array_length(32), 1);
    assert_eq!(BitHelper::to_int_array_length(33), 2);
    assert_eq!(BitHelper::to_int_array_length(64), 2);
    assert_eq!(BitHelper::to_int_array_length(65), 3);
    assert_eq!(BitHelper::to_int_array_length(100), 4);
    assert_eq!(BitHelper::to_int_array_length(128), 4);
    assert_eq!(BitHelper::to_int_array_length(129), 5);
}

#[test]
fn test_mark_and_check_bit() {
    let int_array_length = BitHelper::to_int_array_length(100);
    let mut array = vec![0u32; int_array_length];
    let mut bit_helper = BitHelper::new(&mut array);

    // Initially no bits are marked
    assert!(!bit_helper.is_marked(0));
    assert!(!bit_helper.is_marked(31));
    assert!(!bit_helper.is_marked(32));
    assert!(!bit_helper.is_marked(63));

    // Mark some bits
    bit_helper.mark_bit(0);
    bit_helper.mark_bit(1);
    bit_helper.mark_bit(31);
    bit_helper.mark_bit(32);
    bit_helper.mark_bit(63);
    bit_helper.mark_bit(64);
    bit_helper.mark_bit(99);

    // Check marked bits
    assert!(bit_helper.is_marked(0));
    assert!(bit_helper.is_marked(1));
    assert!(!bit_helper.is_marked(2));
    assert!(bit_helper.is_marked(31));
    assert!(bit_helper.is_marked(32));
    assert!(bit_helper.is_marked(63));
    assert!(bit_helper.is_marked(64));
    assert!(bit_helper.is_marked(99));
    assert!(!bit_helper.is_marked(65));
    assert!(!bit_helper.is_marked(98));
}

#[test]
fn test_mark_all_bits_in_range() {
    let int_array_length = BitHelper::to_int_array_length(100);
    let mut array = vec![0u32; int_array_length];
    let mut bit_helper = BitHelper::new(&mut array);

    // Mark all bits from 0 to 99
    for i in 0..100 {
        bit_helper.mark_bit(i);
        assert!(bit_helper.is_marked(i));
    }

    // Verify all are marked
    for i in 0..100 {
        assert!(bit_helper.is_marked(i));
    }
}

#[test]
fn test_mark_bits_across_boundaries() {
    let int_array_length = BitHelper::to_int_array_length(100);
    let mut array = vec![0u32; int_array_length];
    let mut bit_helper = BitHelper::new(&mut array);

    // Mark bits at u32 boundaries
    bit_helper.mark_bit(31);  // Last bit of first u32
    bit_helper.mark_bit(32);  // First bit of second u32
    bit_helper.mark_bit(63);  // Last bit of second u32
    bit_helper.mark_bit(64);  // First bit of third u32

    assert!(bit_helper.is_marked(31));
    assert!(bit_helper.is_marked(32));
    assert!(bit_helper.is_marked(63));
    assert!(bit_helper.is_marked(64));
}

#[test]
fn test_out_of_bounds() {
    let int_array_length = BitHelper::to_int_array_length(100);
    let mut array = vec![0u32; int_array_length];
    let mut bit_helper = BitHelper::new(&mut array);

    // Marking out of bounds should not panic but also not mark
    bit_helper.mark_bit(1000);
    assert!(!bit_helper.is_marked(1000));
    assert!(!bit_helper.is_marked(100));
}

#[test]
fn test_copy_block() {
    let bytes = vec![0xFFu8, 0x00u8, 0xAAu8, 0x55u8];
        
    // Copy first 8 bits (should be 0xFF)
    let result = BitHelper::copy_block(&bytes, 0, 8);
    assert_eq!(result, vec![0xFF]);

    // Copy bits 8-15 (should be 0x00)
    let result = BitHelper::copy_block(&bytes, 8, 8);
    assert_eq!(result, vec![0x00]);

    // Copy first 16 bits
    let result = BitHelper::copy_block(&bytes, 0, 16);
    assert_eq!(result, vec![0xFF, 0x00]);

    // Copy with 4-bit offset
    let result = BitHelper::copy_block(&bytes, 4, 8);
    assert_eq!(result, vec![0xF0]);

    // Copy partial byte
    let result = BitHelper::copy_block(&bytes, 0, 4);
    assert_eq!(result, vec![0xF0]);

    // Copy across byte boundaries
    let result = BitHelper::copy_block(&bytes, 4, 12);
    assert_eq!(result, vec![0xF0, 0x0A]);
}

#[test]
fn test_copy_block_empty() {
    let bytes = vec![0xFFu8, 0x00u8];
    let result = BitHelper::copy_block(&bytes, 0, 0);
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
#[should_panic(expected = "offset out of range")]
fn test_copy_block_invalid_offset() {
    let bytes = vec![0xFFu8];
    BitHelper::copy_block(&bytes, 100, 8);
}

#[test]
#[should_panic(expected = "offset + length out of range")]
fn test_copy_block_invalid_length() {
    let bytes = vec![0xFFu8];
    BitHelper::copy_block(&bytes, 0, 100);
}

#[test]
fn test_copy_bytes() {
    let mut dst = vec![0u8; 10];
    let src = vec![1u8, 2u8, 3u8];
        
    BitHelper::copy_bytes(&mut dst, 2, &src);
    assert_eq!(dst[0], 0);
    assert_eq!(dst[1], 0);
    assert_eq!(dst[2], 1);
    assert_eq!(dst[3], 2);
    assert_eq!(dst[4], 3);
    assert_eq!(dst[5], 0);
}

#[test]
fn test_copy_bytes_at_start() {
    let mut dst = vec![0u8; 5];
    let src = vec![0xAAu8, 0xBBu8];
    
    BitHelper::copy_bytes(&mut dst, 0, &src);
    assert_eq!(dst[0], 0xAA);
    assert_eq!(dst[1], 0xBB);
}

#[test]
#[should_panic(expected = "destination buffer too small")]
fn test_copy_bytes_buffer_too_small() {
    let mut dst = vec![0u8; 2];
    let src = vec![1u8, 2u8, 3u8];
    BitHelper::copy_bytes(&mut dst, 0, &src);
}

#[test]
fn test_read_u64() {
        let mut x = 0x123456789ABCDEF0u64;
        let result = BitHelper::read(&mut x, 8);
        assert_eq!(result, 0x12);
        assert_eq!(x, 0x3456789ABCDEF000u64);
        
        let result = BitHelper::read(&mut x, 4);
        assert_eq!(result, 0x3);
        assert_eq!(x, 0x456789ABCDEF0000u64);
}

#[test]
fn test_read_u64_multiple() {
    let mut x = 0x123456789ABCDEF0u64;
    
    let r1 = BitHelper::read(&mut x, 8);
    assert_eq!(r1, 0x12);
    
    let r2 = BitHelper::read(&mut x, 8);
    assert_eq!(r2, 0x34);
    
    let r3 = BitHelper::read(&mut x, 8);
    assert_eq!(r3, 0x56);
}

#[test]
fn test_read_from_bytes() {
    let bytes = vec![0x12u8, 0x34u8, 0x56u8, 0x78u8];
    let mut offset = 0;
        
    let result = BitHelper::read_from_bytes(&bytes, &mut offset, 8);
    assert_eq!(result, 0x12);
    assert_eq!(offset, 8);
        
    let result = BitHelper::read_from_bytes(&bytes, &mut offset, 8);
    assert_eq!(result, 0x34);
    assert_eq!(offset, 16);
    
    let result = BitHelper::read_from_bytes(&bytes, &mut offset, 8);
    assert_eq!(result, 0x56);
    assert_eq!(offset, 24);
}

#[test]
fn test_read_from_bytes_with_offset() {
    let bytes = vec![0x12u8, 0x34u8, 0x56u8, 0x78u8];
    let mut offset = 4;  // Start at bit 4
        
    let result = BitHelper::read_from_bytes(&bytes, &mut offset, 8);
    assert_eq!(result, 0x23);
    assert_eq!(offset, 12);
}

#[test]
fn test_read_from_bytes_partial() {
    let bytes = vec![0xFFu8, 0x00u8];
    let mut offset = 0;
    
    let result = BitHelper::read_from_bytes(&bytes, &mut offset, 4);
    assert_eq!(result, 0xF);
    assert_eq!(offset, 4);
}

#[test]
#[should_panic(expected = "offset out of range")]
fn test_read_from_bytes_invalid_offset() {
    let bytes = vec![0xFFu8];
    let mut offset = 100;
    BitHelper::read_from_bytes(&bytes, &mut offset, 8);
}

#[test]
#[should_panic(expected = "offset + length out of range")]
fn test_read_from_bytes_invalid_length() {
    let bytes = vec![0xFFu8];
    let mut offset = 0;
    BitHelper::read_from_bytes(&bytes, &mut offset, 100);
}

#[test]
fn test_write_u64() {
    let mut x = 0x123456789ABCDEF0u64;
    BitHelper::write(&mut x, 8, 0xFF);
    assert_eq!(x, 0x3456789ABCDEF0FFu64);
    
    BitHelper::write(&mut x, 4, 0x5);
    assert_eq!(x, 0x456789ABCDEF0FF5u64);
}

#[test]
fn test_write_u64_multiple() {
    let mut x = 0u64;
    
    BitHelper::write(&mut x, 8, 0x12);
    assert_eq!(x, 0x12);
    
    BitHelper::write(&mut x, 8, 0x34);
    assert_eq!(x, 0x1234);
    
    BitHelper::write(&mut x, 8, 0x56);
    assert_eq!(x, 0x123456);
}

#[test]
fn test_write_u64_mask() {
    let mut x = 0u64;
    // Write only 4 bits, higher bits should be masked
    BitHelper::write(&mut x, 4, 0xFF);
    assert_eq!(x, 0xF);
}

#[test]
fn test_get_bit_size_signed() {
    assert_eq!(BitHelper::get_bit_size(0i8), 0);
    assert_eq!(BitHelper::get_bit_size(1i8), 8);
    assert_eq!(BitHelper::get_bit_size(2i8), 16);
    assert_eq!(BitHelper::get_bit_size(-1i8), 0);  // Negative values return 0
    
    assert_eq!(BitHelper::get_bit_size_i16(0i16), 0);
    assert_eq!(BitHelper::get_bit_size_i16(1i16), 8);
    
    assert_eq!(BitHelper::get_bit_size_i32(0i32), 0);
    assert_eq!(BitHelper::get_bit_size_i32(1i32), 8);
    
    assert_eq!(BitHelper::get_bit_size_i64(0i64), 0);
    assert_eq!(BitHelper::get_bit_size_i64(1i64), 8);
    
    assert_eq!(BitHelper::get_bit_size_i128(0i128), 0);
    assert_eq!(BitHelper::get_bit_size_i128(1i128), 8);
    
    assert_eq!(BitHelper::get_bit_size_isize(0isize), 0);
    assert_eq!(BitHelper::get_bit_size_isize(1isize), 8);
}

#[test]
fn test_get_bit_size_unsigned() {
    assert_eq!(BitHelper::get_bit_size_u8(0u8), 0);
    assert_eq!(BitHelper::get_bit_size_u8(1u8), 8);
    assert_eq!(BitHelper::get_bit_size_u8(2u8), 16);
    
    assert_eq!(BitHelper::get_bit_size_u16(0u16), 0);
    assert_eq!(BitHelper::get_bit_size_u16(1u16), 8);
    
    assert_eq!(BitHelper::get_bit_size_u32(0u32), 0);
    assert_eq!(BitHelper::get_bit_size_u32(1u32), 8);
    
    assert_eq!(BitHelper::get_bit_size_u64(0u64), 0);
    assert_eq!(BitHelper::get_bit_size_u64(1u64), 8);
    
    assert_eq!(BitHelper::get_bit_size_u128(0u128), 0);
    assert_eq!(BitHelper::get_bit_size_u128(1u128), 8);
    
    assert_eq!(BitHelper::get_bit_size_usize(0usize), 0);
    assert_eq!(BitHelper::get_bit_size_usize(1usize), 8);
}

#[test]
fn test_get_bit_size_large_values() {
    assert_eq!(BitHelper::get_bit_size_u64(100u64), 800);
    assert_eq!(BitHelper::get_bit_size_u32(1000u32), 8000);
    assert_eq!(BitHelper::get_bit_size_usize(42usize), 336);
}

#[test]
fn test_integration_scenario() {
    // Simulate a scenario where we track visited items in a collection
    let num_items = 200;
    let int_array_length = BitHelper::to_int_array_length(num_items);
    let mut array = vec![0u32; int_array_length];
    let mut bit_helper = BitHelper::new(&mut array);
    
    // Mark items at various positions
    let items_to_mark = vec![0, 1, 31, 32, 63, 64, 99, 100, 150, 199];
    
    for item in &items_to_mark {
        bit_helper.mark_bit(*item);
    }
    
    // Verify all marked items are marked
    for item in &items_to_mark {
        assert!(bit_helper.is_marked(*item), "Item {} should be marked", item);
    }
    
    // Verify some unmarked items are not marked
    let unmarked_items = vec![2, 30, 33, 62, 65, 98, 101, 149, 151, 198];
    for item in &unmarked_items {
        assert!(!bit_helper.is_marked(*item), "Item {} should not be marked", item);
    }
}

#[test]
fn test_bit_helper_with_different_sizes() {
    // Test with small array
    let mut array = vec![0u32; 1];
    let mut bit_helper = BitHelper::new(&mut array);
    bit_helper.mark_bit(0);
    bit_helper.mark_bit(31);
    assert!(bit_helper.is_marked(0));
    assert!(bit_helper.is_marked(31));
    
    // Test with larger array
    let mut array = vec![0u32; 10];
    let mut bit_helper = BitHelper::new(&mut array);
    bit_helper.mark_bit(0);
    bit_helper.mark_bit(319);  // Last bit of 10th u32
    assert!(bit_helper.is_marked(0));
    assert!(bit_helper.is_marked(319));
}

