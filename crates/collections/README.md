# emixcollections

`emixcollections` provides generic collection utilities, starting with a flexible
`Range<T>` type that works with any `Step`-implementing type (integers, chars, etc.).

## Highlights

- `Range<T>`: A generic range type with min/max bounds that works with any
  `Step`-implementing type.
- `Step` trait: Enables forward/backward iteration for custom types.
- Range operations: Contains checks, bounding, merging, inflation/deflation,
  shifting, and overlap detection.
- Iterator support: Ranges can be iterated directly using `for` loops or `.iter()`.

```toml
[dependencies]
emixcollections = { path = "../../crates/collections" }
```

## Quick Examples

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

See `tests/range.rs` for comprehensive test coverage of all range operations.

