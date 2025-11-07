/// Trait for merging update models into existing models
pub trait Merge<T> {
    fn merge(&self, model: &mut T) -> bool;
}

