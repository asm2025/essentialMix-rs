pub trait Merge<T> {
    fn merge(&self, model: &mut T);
}
