# emixcollections

`emixcollections` provides generic collection utilities, including flexible range types
that work with any `Step`-implementing type (integers, chars, etc.).

## Highlights

- `Range<T>`: A simple, efficient range type with min/max bounds and standard operations.
- `LambdaRange<T>`: An advanced range with configurable boundary inclusion and custom step functions.
- `BitHelper`: Efficient bit marking and manipulation utilities for collections and bit arrays.
- `Step` trait: Enables forward/backward iteration for custom types.
- Comprehensive operations: Contains checks, bounding, merging, inflation/deflation,
  shifting, overlap detection, and flexible iteration.
- Iterator support: Both range types can be iterated directly using `for` loops or `.iter()`.

```toml
[dependencies]
emixcollections = { path = "../../crates/collections" }
```

## Collection Types

### Range<T>

`Range<T>` is a simple, efficient range type with inclusive bounds. It provides standard
range operations like contains checks, bounding, merging, and transformations.

#### Features

- Inclusive bounds (min and max are both included)
- Contains checks (inclusive, exclusive, and mixed)
- Value bounding/clamping
- Range merging and overlap detection
- Range transformations (inflate, deflate, shift)
- Iterator support

#### Examples

Basic range operations:

```rust
use emixcollections::range::Range;

let r = Range::new(10, 20);
assert!(r.contains(15));
assert_eq!(r.bound(5), 10);
assert_eq!(r.bound(25), 20);
```

Range iteration:

```rust
use emixcollections::range::Range;

let r = Range::new(1, 5);
let values: Vec<i32> = r.iter().collect();
assert_eq!(values, vec![1, 2, 3, 4, 5]);
```

Character ranges:

```rust
use emixcollections::range::Range;

let r = Range::new('a', 'e');
let chars: Vec<char> = r.iter().collect();
assert_eq!(chars, vec!['a', 'b', 'c', 'd', 'e']);
```

Range merging and overlap detection:

```rust
use emixcollections::range::Range;

let r1 = Range::new(10, 20);
let r2 = Range::new(15, 25);
assert!(r1.overlaps(&r2));

let merged = r1.merge(&r2);
assert_eq!(merged.min, 10);
assert_eq!(merged.max, 25);
```

Range transformations:

```rust
use emixcollections::range::Range;

let r = Range::new(10, 20);
let inflated = r.inflate(5);  // Expands by 5 on each side
assert_eq!(inflated.min, 5);
assert_eq!(inflated.max, 25);

let shifted = r.shift_forward(5);  // Moves range forward
assert_eq!(shifted.min, 15);
assert_eq!(shifted.max, 25);
```

Single-value ranges:

```rust
use emixcollections::range::Range;

let r = Range::single(42);
assert!(r.is_single());
let values: Vec<i32> = r.iter().collect();
assert_eq!(values, vec![42]);
```

### LambdaRange<T>

`LambdaRange<T>` is an advanced range type that supports configurable boundary inclusion
and custom step functions for flexible iteration patterns.

#### Features

- Configurable boundary inclusion (include/exclude start and/or end)
- Custom step functions for arbitrary iteration patterns
- Standard step helpers (up_by, down_by)
- Bidirectional iteration (from_start, from_end)
- Automatic direction detection based on step function
- Iterator support with custom stepping

#### Examples

Basic inclusive range:

```rust
use emixcollections::range::LambdaRange;

let r = LambdaRange::new(1, 5);
let values: Vec<i32> = r.iter().collect();
assert_eq!(values, vec![1, 2, 3, 4, 5]);
```

Exclusive boundaries:

```rust
use emixcollections::range::LambdaRange;

// Exclude start
let r = LambdaRange::new(1, 5).exclude_start();
let values: Vec<i32> = r.iter().collect();
assert_eq!(values, vec![2, 3, 4, 5]);

// Exclude end
let r = LambdaRange::new(1, 5).exclude_end();
let values: Vec<i32> = r.iter().collect();
assert_eq!(values, vec![1, 2, 3, 4]);

// Exclude both
let r = LambdaRange::new(1, 5).exclude_start().exclude_end();
let values: Vec<i32> = r.iter().collect();
assert_eq!(values, vec![2, 3, 4]);
```

Step by count:

```rust
use emixcollections::range::LambdaRange;

// Step forward by 2
let r = LambdaRange::new(0, 10);
let values: Vec<i32> = r.up_by(2).collect();
assert_eq!(values, vec![0, 2, 4, 6, 8, 10]);

// Step backward by 2
let values: Vec<i32> = r.down_by(2).collect();
assert_eq!(values, vec![10, 8, 6, 4, 2, 0]);
```

Custom step functions:

```rust
use emixcollections::range::LambdaRange;

let r = LambdaRange::new(1, 10);
// Multiply by 2 each step
let values: Vec<i32> = r.step(|x| x * 2).collect();
assert_eq!(values, vec![1, 2, 4, 8]);
```

Contains with boundaries:

```rust
use emixcollections::range::LambdaRange;

let r = LambdaRange::new(10, 20);
assert!(r.contains(10));
assert!(r.contains(15));
assert!(r.contains(20));

let r_ex = r.exclude_start().exclude_end();
assert!(!r_ex.contains(10));
assert!(r_ex.contains(15));
assert!(!r_ex.contains(20));
```

Conversion from Range:

```rust
use emixcollections::range::{Range, LambdaRange};

let r = Range::new(1, 5);
let lr: LambdaRange<i32> = r.into();
let values: Vec<i32> = lr.iter().collect();
assert_eq!(values, vec![1, 2, 3, 4, 5]);
```

### BitHelper

`BitHelper` provides efficient bit marking and manipulation utilities. It helps with operations
that rely on bit marking to indicate whether an item in a collection should be added, removed,
visited already, etc. It uses a `Vec<u32>` or slice of `u32`s to efficiently store bit flags.

#### Features

- Efficient bit marking and checking using u32 arrays
- Bit array size calculation (`to_int_array_length`)
- Bit block copying from byte arrays
- Bit reading/writing operations
- Bit size calculations for all integer types
- Safe Rust implementation (no unsafe code)

#### Examples

Basic bit marking:

```rust
use emixcollections::bit_helper::BitHelper;

// Calculate array size needed for 100 bits
let int_array_length = BitHelper::to_int_array_length(100);
let mut array = vec![0u32; int_array_length];
let mut bit_helper = BitHelper::new(&mut array);

// Mark some bits
bit_helper.mark_bit(0);
bit_helper.mark_bit(31);
bit_helper.mark_bit(32);
bit_helper.mark_bit(99);

// Check if bits are marked
assert!(bit_helper.is_marked(0));
assert!(bit_helper.is_marked(31));
assert!(bit_helper.is_marked(32));
assert!(bit_helper.is_marked(99));
assert!(!bit_helper.is_marked(1));
```

Tracking visited items:

```rust
use emixcollections::bit_helper::BitHelper;

let num_items = 200;
let int_array_length = BitHelper::to_int_array_length(num_items);
let mut array = vec![0u32; int_array_length];
let mut bit_helper = BitHelper::new(&mut array);

// Mark visited items
bit_helper.mark_bit(0);
bit_helper.mark_bit(50);
bit_helper.mark_bit(199);

// Check if items were visited
assert!(bit_helper.is_marked(0));
assert!(bit_helper.is_marked(50));
assert!(bit_helper.is_marked(199));
assert!(!bit_helper.is_marked(1));
```

Copying bit blocks:

```rust
use emixcollections::bit_helper::BitHelper;

let bytes = vec![0xFFu8, 0x00u8, 0xAAu8, 0x55u8];

// Copy first 8 bits
let result = BitHelper::copy_block(&bytes, 0, 8);
assert_eq!(result, vec![0xFF]);

// Copy with bit offset
let result = BitHelper::copy_block(&bytes, 4, 8);
assert_eq!(result, vec![0xF0]);
```

Reading bits from byte arrays:

```rust
use emixcollections::bit_helper::BitHelper;

let bytes = vec![0x12u8, 0x34u8, 0x56u8, 0x78u8];
let mut offset = 0;

let result = BitHelper::read_from_bytes(&bytes, &mut offset, 8);
assert_eq!(result, 0x12);
assert_eq!(offset, 8);

let result = BitHelper::read_from_bytes(&bytes, &mut offset, 8);
assert_eq!(result, 0x34);
assert_eq!(offset, 16);
```

Bit size calculations:

```rust
use emixcollections::bit_helper::BitHelper;

assert_eq!(BitHelper::get_bit_size_u8(1u8), 8);
assert_eq!(BitHelper::get_bit_size_u32(100u32), 800);
assert_eq!(BitHelper::get_bit_size_usize(42usize), 336);
assert_eq!(BitHelper::get_bit_size(0i8), 0);
```

## Step Trait

The `Step` trait enables forward/backward iteration for types. It's implemented for all
standard integer types (`i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`,
`u64`, `u128`, `usize`) and `char`. You can implement it for your own types to use them
with the range types.

```rust
use emixcollections::range::Step;

pub trait Step: Ord + Copy {
    fn forward(self) -> Self;
    fn backward(self) -> Self;
}
```

## Test Coverage

- See `tests/range.rs` for comprehensive test coverage of `Range<T>` operations.
- See `tests/lambda_range.rs` for comprehensive test coverage of `LambdaRange<T>` operations.
- See `tests/bit_helper.rs` for comprehensive test coverage of `BitHelper` operations.

