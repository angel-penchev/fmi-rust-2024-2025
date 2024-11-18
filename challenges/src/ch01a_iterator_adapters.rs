pub struct MapIter<I, F> {
    iter: I,
    func: F,
}

impl<I: Iterator, F: Fn(I::Item) -> B, B> MapIter<I, F> {
    pub fn new(iter: I, func: F) -> MapIter<I, F> {
        MapIter { iter, func }
    }
}

impl<I: Iterator, F: Fn(I::Item) -> B, B> Iterator for MapIter<I, F> {
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| (self.func)(x))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_map_iter() {
        let xs = &["x", "xx", "xxx"];
        let map_iter = MapIter::new(xs.iter(), |s| s.len());

        assert_eq!(map_iter.collect::<Vec<_>>(), vec![1, 2, 3]);
    }
}
