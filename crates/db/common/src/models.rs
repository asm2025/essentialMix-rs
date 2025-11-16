/// Trait for merging update models into existing models
pub trait TMerge<T> {
    fn merge(&self, model: &mut T) -> bool;
}

