use std::char;

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
    let mut fib_iter = FibIter::new();
    let mut result: Vec<String> = vec![String::from("")];
    let mut current_substring_size: u32 = 0;

    for char in (*text).chars() {
        result.last_mut().unwrap().push(char);
        current_substring_size += 1;

        if current_substring_size == fib_iter.number1 {
            result.push(String::from(""));
            fib_iter.next();
            current_substring_size = 0;
        }
    }

    result
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

        let _words: Vec<String> = fib_split("Fibonacci words!");

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

    /// Validates FibIter can be correctly initialized.
    #[test]
    fn test_fib_iter_new() {
        let fib_iter = FibIter::new();
        assert_eq!(1, fib_iter.number1);
        assert_eq!(1, fib_iter.number2);
    }

    /// Validates the fist 8 FibIter itterations (produced by next()) are correctly created.
    #[test]
    fn test_fib_iter_next_8_times() {
        let mut fib_iter = FibIter::new();

        fib_iter.next();
        assert_eq!(1, fib_iter.number1);
        assert_eq!(2, fib_iter.number2);

        fib_iter.next();
        assert_eq!(2, fib_iter.number1);
        assert_eq!(3, fib_iter.number2);

        fib_iter.next();
        assert_eq!(3, fib_iter.number1);
        assert_eq!(5, fib_iter.number2);

        fib_iter.next();
        assert_eq!(5, fib_iter.number1);
        assert_eq!(8, fib_iter.number2);

        fib_iter.next();
        assert_eq!(8, fib_iter.number1);
        assert_eq!(13, fib_iter.number2);

        fib_iter.next();
        assert_eq!(13, fib_iter.number1);
        assert_eq!(21, fib_iter.number2);

        fib_iter.next();
        assert_eq!(21, fib_iter.number1);
        assert_eq!(34, fib_iter.number2);

        fib_iter.next();
        assert_eq!(34, fib_iter.number1);
        assert_eq!(55, fib_iter.number2);
    }

    #[test]
    fn test_fib_split_words() {
        let input = "Fibonacci words!";
        let expected_result = vec!["F", "i", "bo", "nac", "ci wo", "rds!"];
        let actual_result = fib_split(input);
        assert_eq!(expected_result, actual_result);
    }
}
