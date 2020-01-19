pub trait Matrix<T> {
    fn row(&self, index: usize) -> Vec<T>;
    fn column(&self, index: usize) -> Vec<T>;
}