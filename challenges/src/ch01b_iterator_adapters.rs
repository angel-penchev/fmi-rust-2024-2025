pub struct DynMapIter<T, U> {
    iter: Box<dyn Iterator<Item = T>>,
    func: Box<dyn Fn(T) -> U>,
}

impl<T, U> DynMapIter<T, U> {
    pub fn new(iter: Box<dyn Iterator<Item = T>>, func: Box<dyn Fn(T) -> U>) -> DynMapIter<T, U> {
        DynMapIter { iter, func }
    }
}

impl<T, U> Iterator for DynMapIter<T, U> {
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| (self.func)(x))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dyn_map_iter() {
        let xs = &["x", "xx", "xxx"];
        let map_iter = DynMapIter::new(Box::new(xs.iter()), Box::new(|s| s.len()));

        assert_eq!(map_iter.collect::<Vec<_>>(), vec![1, 2, 3]);
    }
}
