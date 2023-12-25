pub trait ResourceBag<T> {
    fn get_by_id(&self, id: &str) -> &T;
}
