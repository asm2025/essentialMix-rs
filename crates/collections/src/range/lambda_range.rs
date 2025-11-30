use std::cmp::Ordering;
use std::marker::PhantomData;

use super::{Range, Step};

/// A range that can be iterated with custom step functions and boundary inclusion/exclusion.
#[derive(Debug, Clone)]
pub struct LambdaRange<T: Step> {
    pub min: T,
    pub max: T,
    pub includes_start: bool,
    pub includes_end: bool,
}

impl<T: Step> LambdaRange<T> {
    /// Creates a new inclusive range.
    pub fn new(min: T, max: T) -> Self {
        assert!(min <= max, "min must be <= max");
        Self {
            min,
            max,
            includes_start: true,
            includes_end: true,
        }
    }

    /// Creates a new range with specified boundary inclusion.
    pub fn with_bounds(min: T, max: T, includes_start: bool, includes_end: bool) -> Self {
        assert!(min <= max, "min must be <= max");
        Self {
            min,
            max,
            includes_start,
            includes_end,
        }
    }

    /// Returns a new range excluding the start.
    pub fn exclude_start(self) -> Self {
        Self {
            includes_start: false,
            ..self
        }
    }

    /// Returns a new range excluding the end.
    pub fn exclude_end(self) -> Self {
        Self {
            includes_end: false,
            ..self
        }
    }

    /// Returns a new range including the start.
    pub fn include_start(self) -> Self {
        Self {
            includes_start: true,
            ..self
        }
    }

    /// Returns a new range including the end.
    pub fn include_end(self) -> Self {
        Self {
            includes_end: true,
            ..self
        }
    }

    /// Checks if a value is within the range, respecting boundary inclusion.
    pub fn contains(&self, value: T) -> bool {
        let lower = if self.includes_start {
            self.min <= value
        } else {
            self.min < value
        };

        let upper = if self.includes_end {
            value <= self.max
        } else {
            value < self.max
        };

        lower && upper
    }

    /// Returns an iterator from start to end with default step (forward by 1).
    pub fn iter(&self) -> LambdaRangeIter<T, impl Fn(T) -> T> {
        self.from_start(|v| v.forward())
    }

    /// Returns an iterator from start to end with a custom step function.
    pub fn from_start<F>(&self, step: F) -> LambdaRangeIter<T, F>
    where
        F: Fn(T) -> T,
    {
        LambdaRangeIter::new(
            self.min,
            self.max,
            self.includes_start,
            self.includes_end,
            step,
            true,
        )
    }

    /// Returns an iterator from end to start with a custom step function.
    pub fn from_end<F>(&self, step: F) -> LambdaRangeIter<T, F>
    where
        F: Fn(T) -> T,
    {
        LambdaRangeIter::new(
            self.min,
            self.max,
            self.includes_start,
            self.includes_end,
            step,
            false,
        )
    }

    /// Returns an iterator that steps forward by count on each iteration.
    pub fn up_by(&self, count: usize) -> LambdaRangeIter<T, impl Fn(T) -> T> {
        self.from_start(move |v| {
            let mut result = v;
            for _ in 0..count {
                result = result.forward();
            }
            result
        })
    }

    /// Returns an iterator that steps backward by count on each iteration.
    pub fn down_by(&self, count: usize) -> LambdaRangeIter<T, impl Fn(T) -> T> {
        self.from_end(move |v| {
            let mut result = v;
            for _ in 0..count {
                result = result.backward();
            }
            result
        })
    }

    /// Returns an iterator with a custom step function, automatically determining direction.
    pub fn step<F>(&self, step: F) -> LambdaRangeIter<T, F>
    where
        F: Fn(T) -> T,
    {
        let ascending = step(self.min) > self.min;
        if ascending {
            self.from_start(step)
        } else {
            self.from_end(step)
        }
    }
}

impl<T: Step> From<Range<T>> for LambdaRange<T> {
    fn from(range: Range<T>) -> Self {
        LambdaRange::new(range.min, range.max)
    }
}

/// Iterator for LambdaRange with custom step function.
#[derive(Clone)]
pub struct LambdaRangeIter<T: Step, F>
where
    F: Fn(T) -> T,
{
    start: T,
    end: T,
    includes_start: bool,
    includes_end: bool,
    step: F,
    current: Option<T>,
    started: bool,
    _phantom: PhantomData<T>,
}

impl<T: Step, F> LambdaRangeIter<T, F>
where
    F: Fn(T) -> T,
{
    fn new(
        min: T,
        max: T,
        includes_start: bool,
        includes_end: bool,
        step: F,
        ascending: bool,
    ) -> Self {
        let (start, end, includes_start, includes_end) = if ascending {
            (min, max, includes_start, includes_end)
        } else {
            (max, min, includes_end, includes_start)
        };

        Self {
            start,
            end,
            includes_start,
            includes_end,
            step,
            current: None,
            started: false,
            _phantom: PhantomData,
        }
    }
}

impl<T: Step, F> Iterator for LambdaRangeIter<T, F>
where
    F: Fn(T) -> T,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if !self.started {
            self.started = true;
            self.current = Some(if self.includes_start {
                self.start
            } else {
                (self.step)(self.start)
            });
        } else if let Some(cur) = self.current {
            if cur < self.end {
                self.current = Some((self.step)(cur));
            } else {
                return None;
            }
        }

        if let Some(value) = self.current {
            let cmp = value.cmp(&self.end);
            match cmp {
                Ordering::Less => Some(value),
                Ordering::Equal if self.includes_end => Some(value),
                _ => None,
            }
        } else {
            None
        }
    }
}
