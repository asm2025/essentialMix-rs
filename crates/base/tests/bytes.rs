#[cfg(test)]
mod tests {
    use emix::bytes::{read_slice, read_value};

    // Helper function to create test bytes
    fn to_bytes_u16(value: u16) -> Vec<u8> {
        value.to_be_bytes().to_vec()
    }

    fn to_bytes_u32(value: u32) -> Vec<u8> {
        value.to_be_bytes().to_vec()
    }

    fn to_bytes_u64(value: u64) -> Vec<u8> {
        value.to_be_bytes().to_vec()
    }

    fn to_bytes_u128(value: u128) -> Vec<u8> {
        value.to_be_bytes().to_vec()
    }

    fn to_bytes_i16(value: i16) -> Vec<u8> {
        value.to_be_bytes().to_vec()
    }

    fn to_bytes_i32(value: i32) -> Vec<u8> {
        value.to_be_bytes().to_vec()
    }

    fn to_bytes_i64(value: i64) -> Vec<u8> {
        value.to_be_bytes().to_vec()
    }

    fn to_bytes_i128(value: i128) -> Vec<u8> {
        value.to_be_bytes().to_vec()
    }

    fn to_bytes_f32(value: f32) -> Vec<u8> {
        value.to_be_bytes().to_vec()
    }

    fn to_bytes_f64(value: f64) -> Vec<u8> {
        value.to_be_bytes().to_vec()
    }

    // Test unsigned integers
    #[test]
    fn test_read_u8() {
        let bytes = vec![42u8];
        let mut offset = 0;
        let result: u8 = read_value(&bytes, &mut offset).unwrap();
        assert_eq!(result, 42);
        assert_eq!(offset, 1);
    }

    #[test]
    fn test_read_u16() {
        let value = 0x1234u16;
        let bytes = to_bytes_u16(value);
        let mut offset = 0;
        let result: u16 = read_value(&bytes, &mut offset).unwrap();
        assert_eq!(result, value);
        assert_eq!(offset, 2);
    }

    #[test]
    fn test_read_u32() {
        let value = 0x12345678u32;
        let bytes = to_bytes_u32(value);
        let mut offset = 0;
        let result: u32 = read_value(&bytes, &mut offset).unwrap();
        assert_eq!(result, value);
        assert_eq!(offset, 4);
    }

    #[test]
    fn test_read_u64() {
        let value = 0x1234567890ABCDEFu64;
        let bytes = to_bytes_u64(value);
        let mut offset = 0;
        let result: u64 = read_value(&bytes, &mut offset).unwrap();
        assert_eq!(result, value);
        assert_eq!(offset, 8);
    }

    #[test]
    fn test_read_u128() {
        let value = 0x1234567890ABCDEF1234567890ABCDEFu128;
        let bytes = to_bytes_u128(value);
        let mut offset = 0;
        let result: u128 = read_value(&bytes, &mut offset).unwrap();
        assert_eq!(result, value);
        assert_eq!(offset, 16);
    }

    // Test signed integers
    #[test]
    fn test_read_i8() {
        let bytes = vec![0xFEu8]; // -2 in two's complement
        let mut offset = 0;
        let result: i8 = read_value(&bytes, &mut offset).unwrap();
        assert_eq!(result, -2);
        assert_eq!(offset, 1);
    }

    #[test]
    fn test_read_i16() {
        let value: i16 = -32768;
        let bytes = to_bytes_i16(value);
        let mut offset = 0;
        let result: i16 = read_value(&bytes, &mut offset).unwrap();
        assert_eq!(result, value);
        assert_eq!(offset, 2);
    }

    #[test]
    fn test_read_i32() {
        let value: i32 = -2147483648;
        let bytes = to_bytes_i32(value);
        let mut offset = 0;
        let result: i32 = read_value(&bytes, &mut offset).unwrap();
        assert_eq!(result, value);
        assert_eq!(offset, 4);
    }

    #[test]
    fn test_read_i64() {
        let value: i64 = i64::MIN;
        let bytes = to_bytes_i64(value);
        let mut offset = 0;
        let result: i64 = read_value(&bytes, &mut offset).unwrap();
        assert_eq!(result, value);
        assert_eq!(offset, 8);
    }

    #[test]
    fn test_read_i128() {
        let value: i128 = i128::MIN;
        let bytes = to_bytes_i128(value);
        let mut offset = 0;
        let result: i128 = read_value(&bytes, &mut offset).unwrap();
        assert_eq!(result, value);
        assert_eq!(offset, 16);
    }

    // Test floating point
    #[test]
    fn test_read_f32() {
        let value = 42.5f32;
        let bytes = to_bytes_f32(value);
        let mut offset = 0;
        let result: f32 = read_value(&bytes, &mut offset).unwrap();
        assert_eq!(result, value);
        assert_eq!(offset, 4);
    }

    #[test]
    fn test_read_f64() {
        let value = 123.456f64;
        let bytes = to_bytes_f64(value);
        let mut offset = 0;
        let result: f64 = read_value(&bytes, &mut offset).unwrap();
        assert_eq!(result, value);
        assert_eq!(offset, 8);
    }

    // Test read_slice
    #[test]
    fn test_read_slice() {
        let bytes = vec![1, 2, 3, 4, 5];
        let mut offset = 0;
        let result = read_slice(&bytes, &mut offset, 3).unwrap();
        assert_eq!(result, &[1, 2, 3]);
        assert_eq!(offset, 3);
    }

    #[test]
    fn test_read_slice_zero_length() {
        let bytes = vec![1, 2, 3, 4, 5];
        let mut offset = 0;
        let result: &[u8] = read_slice(&bytes, &mut offset, 0).unwrap();
        assert_eq!(result, &[] as &[u8]);
        assert_eq!(offset, 0);
    }

    #[test]
    fn test_read_slice_at_offset() {
        let bytes = vec![1, 2, 3, 4, 5];
        let mut offset = 2;
        let result = read_slice(&bytes, &mut offset, 2).unwrap();
        assert_eq!(result, &[3, 4]);
        assert_eq!(offset, 4);
    }

    // Test sequential reads
    #[test]
    fn test_read_multiple_values() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&to_bytes_u16(0x1234));
        bytes.extend_from_slice(&to_bytes_u32(0x12345678));
        bytes.push(42u8);

        let mut offset = 0;
        let val1: u16 = read_value(&bytes, &mut offset).unwrap();
        let val2: u32 = read_value(&bytes, &mut offset).unwrap();
        let val3: u8 = read_value(&bytes, &mut offset).unwrap();

        assert_eq!(val1, 0x1234);
        assert_eq!(val2, 0x12345678);
        assert_eq!(val3, 42);
        assert_eq!(offset, 7); // 2 + 4 + 1
    }

    #[test]
    fn test_read_mixed_types() {
        let mut bytes = Vec::new();
        bytes.push(100u8);
        bytes.extend_from_slice(&to_bytes_u32(12345));
        bytes.extend_from_slice(&to_bytes_f32(99.99));
        bytes.push(200u8);

        let mut offset = 0;
        let val1: u8 = read_value(&bytes, &mut offset).unwrap();
        let val2: u32 = read_value(&bytes, &mut offset).unwrap();
        let val3: f32 = read_value(&bytes, &mut offset).unwrap();
        let val4: u8 = read_value(&bytes, &mut offset).unwrap();

        assert_eq!(val1, 100);
        assert_eq!(val2, 12345);
        assert_eq!(val3, 99.99);
        assert_eq!(val4, 200);
        assert_eq!(offset, 1 + 4 + 4 + 1);
    }

    #[test]
    fn test_read_slice_and_value() {
        let bytes = vec![1, 2, 3, 4, 5];
        let mut offset = 0;
        let slice = read_slice(&bytes, &mut offset, 2).unwrap();
        let value: u8 = read_value(&bytes, &mut offset).unwrap();

        assert_eq!(slice, &[1, 2]);
        assert_eq!(value, 3);
        assert_eq!(offset, 3);
    }

    // Test error cases
    #[test]
    fn test_read_value_offset_out_of_range() {
        let bytes = vec![1, 2, 3];
        let mut offset = 10;
        let result: Result<u8, _> = read_value(&bytes, &mut offset);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_value_not_enough_data() {
        let bytes = vec![1, 2];
        let mut offset = 0;
        let result: Result<u32, _> = read_value(&bytes, &mut offset);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_slice_offset_out_of_range() {
        let bytes = vec![1, 2, 3];
        let mut offset = 10;
        let result = read_slice(&bytes, &mut offset, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_slice_not_enough_data() {
        let bytes = vec![1, 2, 3];
        let mut offset = 0;
        let result = read_slice(&bytes, &mut offset, 10);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_slice_saturating_add() {
        let bytes = vec![1, 2, 3];
        let mut offset = usize::MAX;
        let result = read_slice(&bytes, &mut offset, 1);
        assert!(result.is_err());
    }

    // Test large sequential reads
    #[test]
    fn test_read_large_value_sequence() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&to_bytes_u64(0x1234567890ABCDEF));
        bytes.extend_from_slice(&to_bytes_u64(0xFEDCBA0987654321));
        bytes.extend_from_slice(&to_bytes_u64(0x1111111111111111));

        let mut offset = 0;
        let val1: u64 = read_value(&bytes, &mut offset).unwrap();
        let val2: u64 = read_value(&bytes, &mut offset).unwrap();
        let val3: u64 = read_value(&bytes, &mut offset).unwrap();

        assert_eq!(val1, 0x1234567890ABCDEF);
        assert_eq!(val2, 0xFEDCBA0987654321);
        assert_eq!(val3, 0x1111111111111111);
        assert_eq!(offset, 24);
    }
}
