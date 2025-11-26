use std::cmp::Ordering;

/// Trait for types that can be stepped through in a range.
pub trait Step: Ord + Copy {
    fn forward(self) -> Self;
    fn backward(self) -> Self;
}

impl Step for i8 {
    fn forward(self) -> Self {
        self.saturating_add(1)
    }
    fn backward(self) -> Self {
        self.saturating_sub(1)
    }
}

impl Step for i16 {
    fn forward(self) -> Self {
        self.saturating_add(1)
    }
    fn backward(self) -> Self {
        self.saturating_sub(1)
    }
}

impl Step for i32 {
    fn forward(self) -> Self {
        self.saturating_add(1)
    }
    fn backward(self) -> Self {
        self.saturating_sub(1)
    }
}

impl Step for i64 {
    fn forward(self) -> Self {
        self.saturating_add(1)
    }
    fn backward(self) -> Self {
        self.saturating_sub(1)
    }
}

impl Step for i128 {
    fn forward(self) -> Self {
        self.saturating_add(1)
    }
    fn backward(self) -> Self {
        self.saturating_sub(1)
    }
}

impl Step for isize {
    fn forward(self) -> Self {
        self.saturating_add(1)
    }
    fn backward(self) -> Self {
        self.saturating_sub(1)
    }
}

impl Step for u8 {
    fn forward(self) -> Self {
        self.saturating_add(1)
    }
    fn backward(self) -> Self {
        self.saturating_sub(1)
    }
}

impl Step for u16 {
    fn forward(self) -> Self {
        self.saturating_add(1)
    }
    fn backward(self) -> Self {
        self.saturating_sub(1)
    }
}

impl Step for u32 {
    fn forward(self) -> Self {
        self.saturating_add(1)
    }
    fn backward(self) -> Self {
        self.saturating_sub(1)
    }
}

impl Step for u64 {
    fn forward(self) -> Self {
        self.saturating_add(1)
    }
    fn backward(self) -> Self {
        self.saturating_sub(1)
    }
}

impl Step for u128 {
    fn forward(self) -> Self {
        self.saturating_add(1)
    }
    fn backward(self) -> Self {
        self.saturating_sub(1)
    }
}

impl Step for usize {
    fn forward(self) -> Self {
        self.saturating_add(1)
    }
    fn backward(self) -> Self {
        self.saturating_sub(1)
    }
}

impl Step for char {
    fn forward(self) -> Self {
        char::from_u32(self as u32 + 1).unwrap_or(self)
    }
    fn backward(self) -> Self {
        char::from_u32((self as u32).saturating_sub(1)).unwrap_or(self)
    }
}

/// A range with minimum and maximum bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range<T: Step> {
    pub min: T,
    pub max: T,
}

impl<T: Step> Range<T> {
    /// Creates a new range. Panics if min > max.
    pub fn new(min: T, max: T) -> Self {
        assert!(min <= max, "min must be <= max");
        Self { min, max }
    }

    /// Creates a range from a single value.
    pub fn single(value: T) -> Self {
        Self {
            min: value,
            max: value,
        }
    }

    /// Returns true if this is a single-value range.
    pub fn is_single(&self) -> bool {
        self.min == self.max
    }

    /// Checks if a value is within the range (inclusive).
    pub fn contains(&self, value: T) -> bool {
        self.min <= value && value <= self.max
    }

    /// Checks if a value is within the range (exclusive).
    pub fn contains_exclusive(&self, value: T) -> bool {
        self.min < value && value < self.max
    }

    /// Checks if a value is within the range (left exclusive, right inclusive).
    pub fn contains_left_exclusive(&self, value: T) -> bool {
        self.min < value && value <= self.max
    }

    /// Checks if a value is within the range (left inclusive, right exclusive).
    pub fn contains_right_exclusive(&self, value: T) -> bool {
        self.min <= value && value < self.max
    }

    /// Bounds a value to be within the range.
    ///
    /// Returns the value if it's within the range, otherwise returns the nearest bound.
    pub fn bound(&self, value: T) -> T {
        if value < self.min {
            self.min
        } else if value > self.max {
            self.max
        } else {
            value
        }
    }

    /// Bounds a value to be within the range (exclusive).
    pub fn bound_exclusive(&self, value: T) -> T {
        let min = self.min.forward();
        let max = self.max.backward();
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }

    /// Bounds a value to be within the range (left exclusive).
    pub fn bound_left_exclusive(&self, value: T) -> T {
        let min = self.min.forward();
        if value < min {
            min
        } else if value > self.max {
            self.max
        } else {
            value
        }
    }

    /// Bounds a value to be within the range (right exclusive).
    pub fn bound_right_exclusive(&self, value: T) -> T {
        let max = self.max.backward();
        if value < self.min {
            self.min
        } else if value > max {
            max
        } else {
            value
        }
    }

    /// Checks if this range contains another range entirely.
    pub fn contains_range(&self, other: &Range<T>) -> bool {
        self.contains(other.min) && self.contains(other.max)
    }

    /// Checks if this range overlaps with another range.
    pub fn overlaps(&self, other: &Range<T>) -> bool {
        self.min <= other.max && other.min <= self.max
    }

    /// Checks if this range is immediately before another range.
    pub fn is_before(&self, other: &Range<T>) -> bool {
        self.max < other.min
    }

    /// Checks if this range is immediately after another range.
    pub fn is_after(&self, other: &Range<T>) -> bool {
        self.min > other.max
    }

    /// Merges this range with another, returning a new range that spans both.
    pub fn merge(&self, other: &Range<T>) -> Range<T> {
        Range {
            min: if self.min < other.min {
                self.min
            } else {
                other.min
            },
            max: if self.max > other.max {
                self.max
            } else {
                other.max
            },
        }
    }

    /// Expands the range by moving min backward and max forward by count steps.
    pub fn inflate(&self, count: usize) -> Range<T> {
        let mut min = self.min;
        let mut max = self.max;
        for _ in 0..count {
            min = min.backward();
            max = max.forward();
        }
        Range { min, max }
    }

    /// Shrinks the range by moving min forward and max backward by count steps.
    pub fn deflate(&self, count: usize) -> Range<T> {
        let mut min = self.min;
        let mut max = self.max;
        for _ in 0..count {
            min = min.forward();
            max = max.backward();
            if min > max {
                break;
            }
        }
        Range { min, max }
    }

    /// Shifts the range forward by count steps.
    pub fn shift_forward(&self, count: usize) -> Range<T> {
        let mut min = self.min;
        let mut max = self.max;
        for _ in 0..count {
            min = min.forward();
            max = max.forward();
        }
        Range { min, max }
    }

    /// Shifts the range backward by count steps.
    pub fn shift_backward(&self, count: usize) -> Range<T> {
        let mut min = self.min;
        let mut max = self.max;
        for _ in 0..count {
            min = min.backward();
            max = max.backward();
        }
        Range { min, max }
    }

    /// Returns an iterator over the values in the range.
    pub fn iter(&self) -> RangeIter<T> {
        RangeIter {
            current: None,
            min: self.min,
            max: self.max,
        }
    }
}

impl<T: Step> PartialOrd for Range<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Step> Ord for Range<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.min.cmp(&other.min) {
            Ordering::Equal => self.max.cmp(&other.max),
            other => other,
        }
    }
}

impl<T: Step> IntoIterator for Range<T> {
    type Item = T;
    type IntoIter = RangeIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T: Step> IntoIterator for &'a Range<T> {
    type Item = T;
    type IntoIter = RangeIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Iterator over a range.
#[derive(Debug, Clone)]
pub struct RangeIter<T: Step> {
    current: Option<T>,
    min: T,
    max: T,
}

impl<T: Step> Iterator for RangeIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let value = match self.current {
            None => self.min,
            Some(cur) => {
                if cur >= self.max {
                    return None;
                }
                cur.forward()
            }
        };

        if value > self.max {
            return None;
        }

        self.current = Some(value);
        Some(value)
    }
}
