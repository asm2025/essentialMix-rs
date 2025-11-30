mod lambda_range;
pub use lambda_range::*;
mod range;
pub use range::*;

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
