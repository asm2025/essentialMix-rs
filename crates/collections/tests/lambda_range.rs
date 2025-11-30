use emixcollections::range::{LambdaRange, Range};

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
fn test_clamp() {
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

#[test]
fn test_lambda_range_inclusive() {
    let r = LambdaRange::new(1, 5);
    let values: Vec<i32> = r.iter().collect();
    assert_eq!(values, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_lambda_range_exclude_start() {
    let r = LambdaRange::new(1, 5).exclude_start();
    let values: Vec<i32> = r.iter().collect();
    assert_eq!(values, vec![2, 3, 4, 5]);
}

#[test]
fn test_lambda_range_exclude_end() {
    let r = LambdaRange::new(1, 5).exclude_end();
    let values: Vec<i32> = r.iter().collect();
    assert_eq!(values, vec![1, 2, 3, 4]);
}

#[test]
fn test_lambda_range_exclude_both() {
    let r = LambdaRange::new(1, 5).exclude_start().exclude_end();
    let values: Vec<i32> = r.iter().collect();
    assert_eq!(values, vec![2, 3, 4]);
}

#[test]
fn test_lambda_range_contains() {
    let r = LambdaRange::new(10, 20);
    assert!(r.contains(10));
    assert!(r.contains(15));
    assert!(r.contains(20));

    let r_ex = r.exclude_start().exclude_end();
    assert!(!r_ex.contains(10));
    assert!(r_ex.contains(15));
    assert!(!r_ex.contains(20));
}

#[test]
fn test_lambda_range_up_by() {
    let r = LambdaRange::new(0, 10);
    let values: Vec<i32> = r.up_by(2).collect();
    assert_eq!(values, vec![0, 2, 4, 6, 8, 10]);
}

#[test]
fn test_lambda_range_down_by() {
    let r = LambdaRange::new(0, 10);
    let values: Vec<i32> = r.down_by(2).collect();
    assert_eq!(values, vec![10, 8, 6, 4, 2, 0]);
}

#[test]
fn test_lambda_range_custom_step() {
    let r = LambdaRange::new(1, 10);
    let values: Vec<i32> = r.step(|x| x * 2).collect();
    assert_eq!(values, vec![1, 2, 4, 8]);
}

#[test]
fn test_lambda_range_from_range() {
    let r = Range::new(1, 5);
    let lr: LambdaRange<i32> = r.into();
    let values: Vec<i32> = lr.iter().collect();
    assert_eq!(values, vec![1, 2, 3, 4, 5]);
}
