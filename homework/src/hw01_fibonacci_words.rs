pub struct FibIter {
    number1: u32,
    number2: u32,
}

impl Default for FibIter {
    fn default() -> Self {
        FibIter {
            number1: 1,
            number2: 1,
        }
    }
}

impl FibIter {
    pub fn new() -> Self {
        FibIter {
            number1: 1,
            number2: 1,
        }
    }

    #[allow(clippy::should_implement_trait)] // Disable clippy warning about the method name being
                                             // similar to a trait name
    pub fn next(&mut self) -> u32 {
        let number2_backup = self.number2;
        self.number2 += self.number1;
        self.number1 = number2_backup;

        self.number2
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

        // let mut rev_fib_iter: RevFibIter = fib_iter.rev();
        // rev_fib_iter.next();
        //
        // let _words: Vec<String> = fib_split("Fibonacci words!");
        //
        // let (_words, _rest): (Vec<String>, &str) = fib_split_n(
        //     "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        //     1,
        // );
        //
        // let (_words, _rest): (Vec<String>, &str) = fib_split_n_symmetric(
        //     "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        //     1,
        // );
    }
}
