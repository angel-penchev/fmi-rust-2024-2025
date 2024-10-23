pub struct FibIter {/* ... */}

impl FibIter {
    pub fn new() -> FibIter {
        todo!()
    }

    pub fn next(&mut self) -> u32 {
        todo!()
    }

    pub fn rev(self) -> RevFibIter {
        todo!()
    }
}

pub fn fib_split(text: &str) -> Vec<String> {
    todo!()
}

pub fn fib_split_n(text: &str, n: u32) -> (Vec<String>, &str) {
    todo!()
}

pub struct RevFibIter {/* ... */}

impl RevFibIter {
    pub fn next(&mut self) -> Option<u32> {
        todo!()
    }
}

pub fn fib_split_n_symmetric(text: &str, n: u32) -> (Vec<String>, &str) {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        let mut fib_iter = FibIter::new();
        fib_iter.next();

        let mut rev_fib_iter: RevFibIter = fib_iter.rev();
        rev_fib_iter.next();

        let _words: Vec<String> = fib_split("Fibonacci words!");

        let (_words, _rest): (Vec<String>, &str) = fib_split_n(
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
            1,
        );

        let (_words, _rest): (Vec<String>, &str) = fib_split_n_symmetric(
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
            1,
        );
    }
}
