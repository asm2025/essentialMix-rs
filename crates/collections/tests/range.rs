#[cfg(test)]
mod tests {
    use emixcollections::range::Range;

    #[test]
    fn test_basic_range() {
        let r = Range::new(1, 5);
        assert_eq!(r.min, 1);
        assert_eq!(r.max, 5);
    }

    #[test]
    fn test_single_range() {
        let r = Range::single(42);
        assert!(r.is_single());
        assert_eq!(r.min, 42);
        assert_eq!(r.max, 42);
    }

    #[test]
    fn test_contains() {
        let r = Range::new(10, 20);
        assert!(r.contains(10));
        assert!(r.contains(15));
        assert!(r.contains(20));
        assert!(!r.contains(9));
        assert!(!r.contains(21));
    }

    #[test]
    fn test_contains_exclusive() {
        let r = Range::new(10, 20);
        assert!(!r.contains_exclusive(10));
        assert!(r.contains_exclusive(15));
        assert!(!r.contains_exclusive(20));
    }

    #[test]
    fn test_bound() {
        let r = Range::new(10, 20);
        assert_eq!(r.bound(5), 10);
        assert_eq!(r.bound(15), 15);
        assert_eq!(r.bound(25), 20);
    }

    #[test]
    fn test_overlaps() {
        let r1 = Range::new(10, 20);
        let r2 = Range::new(15, 25);
        let r3 = Range::new(25, 30);

        assert!(r1.overlaps(&r2));
        assert!(r2.overlaps(&r1));
        assert!(!r1.overlaps(&r3));
    }

    #[test]
    fn test_merge() {
        let r1 = Range::new(10, 20);
        let r2 = Range::new(15, 25);
        let merged = r1.merge(&r2);

        assert_eq!(merged.min, 10);
        assert_eq!(merged.max, 25);
    }

    #[test]
    fn test_inflate() {
        let r = Range::new(10, 20);
        let inflated = r.inflate(5);

        assert_eq!(inflated.min, 5);
        assert_eq!(inflated.max, 25);
    }

    #[test]
    fn test_deflate() {
        let r = Range::new(10, 20);
        let deflated = r.deflate(2);

        assert_eq!(deflated.min, 12);
        assert_eq!(deflated.max, 18);
    }

    #[test]
    fn test_shift() {
        let r = Range::new(10, 20);
        let shifted = r.shift_forward(5);

        assert_eq!(shifted.min, 15);
        assert_eq!(shifted.max, 25);

        let shifted_back = r.shift_backward(5);
        assert_eq!(shifted_back.min, 5);
        assert_eq!(shifted_back.max, 15);
    }

    #[test]
    fn test_iteration() {
        let r = Range::new(1, 5);
        let values: Vec<i32> = r.iter().collect();

        assert_eq!(values, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_iteration_single() {
        let r = Range::single(42);
        let values: Vec<i32> = r.iter().collect();

        assert_eq!(values, vec![42]);
    }

    #[test]
    fn test_char_range() {
        let r = Range::new('a', 'e');
        let chars: Vec<char> = r.iter().collect();

        assert_eq!(chars, vec!['a', 'b', 'c', 'd', 'e']);
    }

    #[test]
    fn test_ordering() {
        let r1 = Range::new(1, 5);
        let r2 = Range::new(1, 10);
        let r3 = Range::new(2, 5);

        assert!(r1 < r2);
        assert!(r1 < r3);
        assert!(r2 < r3);
    }
}
