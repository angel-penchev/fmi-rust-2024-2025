pub struct FibIter {
    number1: u32,
    number2: u32,
}

/// Default implementation for `FibIter` struct.
impl Default for FibIter {
    fn default() -> Self {
        FibIter {
            number1: 1,
            number2: 1,
        }
    }
}

impl FibIter {
    /// Creates a new `FibIter` with the first two Fibonacci numbers set to 1.
    ///
    /// # Returns
    /// A new `FibIter` struct with the first two numbers set to 1.
    ///
    /// # Example
    /// ```rust
    /// use homework::hw01_fibonacci_words::FibIter;
    /// let fib_iter = FibIter::new();
    /// ```
    pub fn new() -> Self {
        FibIter {
            number1: 1,
            number2: 1,
        }
    }

    /// Returns the next Fibonacci number in the sequence.
    /// Increments the internal state of the iterator.
    ///
    /// # Returns
    /// The next Fibonacci number in the sequence
    ///
    /// # Example
    /// ```rust
    /// use homework::hw01_fibonacci_words::FibIter;
    /// let mut fib_iter = FibIter::new();
    /// let result = fib_iter.next(); // result == 1
    /// let result = fib_iter.next(); // result == 2
    /// let result = fib_iter.next(); // result == 3
    /// let result = fib_iter.next(); // result == 5
    /// let result = fib_iter.next(); // result == 8
    /// ```
    #[allow(clippy::should_implement_trait)] // Disable clippy warning about the method name being
                                             // similar to iterator trait name.
    pub fn next(&mut self) -> u32 {
        let number2_backup = self.number2;
        self.number2 += self.number1;
        self.number1 = number2_backup;

        self.number1
    }

    /// Creates a new `RevFibIter` from a `FibIter`.
    ///
    /// # Returns
    /// A new `RevFibIter` struct with the first two numbers set to the last two numbers of the
    /// Fibonacci sequence iterator.
    ///
    /// # Example
    /// ```rust
    /// use homework::hw01_fibonacci_words::FibIter;
    /// use homework::hw01_fibonacci_words::RevFibIter;
    ///
    /// let mut fib_iter = FibIter::new();
    /// fib_iter.next(); // number1 = 1, number2 = 2
    /// let rev_fib_iter: RevFibIter = fib_iter.rev(); // number1 = 1, number2 = 2
    /// ```
    pub fn rev(self) -> RevFibIter {
        RevFibIter {
            number1: self.number1,
            number2: self.number2,
        }
    }
}

/// Creates a vector with words that have lengths that match the Fibonacci numbers. The last
/// element of the vector is the rest of the string.
///
/// # Arguments
/// * `text` - A string to split into words.
///
/// # Returns
/// A vector of strings with lengths that match the Fibonacci numbers.
///
/// # Example
/// ```rust
/// use homework::hw01_fibonacci_words::fib_split;
/// let text = "Fibonacci words!";
/// let words: Vec<String> = fib_split(text); // vec!["F", "i", "bo", "nac", "ci wo", "rds!"]
/// ```
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

/// Creates a vector with N words that have lengths that match the Fibonacci numbers. The rest of
/// the characters are returned as a string.
///
/// # Arguments
/// * `text` - A string to split into words.
/// * `n` - The number of Fibonacci words to split the text into.
///
/// # Returns
/// A tuple (vector of strings, rest of the text).
///
/// # Example
/// ```rust
/// use homework::hw01_fibonacci_words::fib_split_n;
///
/// let text = "Lorem ipsum dolor sit amet.";
/// let (words, rest) = fib_split_n(text, 5); // results in (
///                                           //   vec!["L", "o", "re", "m i", "psum ", "dolor si"],
///                                           //   "t amet."
///                                           // )
/// ```
pub fn fib_split_n(text: &str, n: u32) -> (Vec<String>, &str) {
    // Gather the vector to return
    let fib_split_of_text = fib_split(text);
    let fib_split_iter = fib_split_of_text.iter();
    let fib_split_n_slice: Vec<String> = fib_split_iter.take(n as usize).cloned().collect();

    // Gather the rest of the text
    let text_skip_n_bytes: usize = fib_split_n_slice.iter().map(|element| element.len()).sum();
    let text_rest = &text[text_skip_n_bytes..];

    (fib_split_n_slice, text_rest)
}

pub struct RevFibIter {
    number1: u32,
    number2: u32,
}

impl RevFibIter {
    /// Returns the next Fibonacci number in the reversed sequence.
    ///
    /// # Returns
    /// The next Fibonacci number in the reversed sequence.
    ///
    /// # Example
    /// ```rust
    /// use homework::hw01_fibonacci_words::FibIter;
    /// use homework::hw01_fibonacci_words::RevFibIter;
    ///
    /// let mut fib_iter = FibIter::new();
    /// fib_iter.next(); // number1 = 1, number2 = 2
    ///
    /// let mut rev_fib_iter: RevFibIter = fib_iter.rev(); // number1 = 1, number2 = 2
    /// rev_fib_iter.next(); // result == Some(1), number1 = 1, number2 = 1
    /// rev_fib_iter.next(); // result == None, number1 = 1, number2 = 1
    /// ```
    #[allow(clippy::should_implement_trait)] // Disable clippy warning about the method name being
                                             // similar to iterator trait name
    pub fn next(&mut self) -> Option<u32> {
        if self.number1 == 1 && self.number2 == 1 {
            return None;
        }

        let number1_backup = self.number1;
        self.number1 = self.number2 - self.number1;
        self.number2 = number1_backup;

        Some(self.number1)
    }
}

/// Creates a vector with 2N symmetric words that have lengths that match the Fibonacci numbers
/// forward and in reverse. The last element of the vector is the rest of the string.
///
/// # Arguments
/// * `text` - A string to split into words.
/// * `n` - The number of Fibonacci words to split the text into forwards and backwards.
///
/// # Returns
/// A tuple (vector of strings, rest of the text).
///
/// # Example
/// ```rust
/// use homework::hw01_fibonacci_words::fib_split_n_symmetric;
///
/// let text = "Lorem ipsum dolor sit amet.";
/// let (words, rest) = fib_split_n_symmetric(text, 5); // results in (
///                                                     //   vec![
///                                                     //     "L",
///                                                     //     "o",
///                                                     //     "re",
///                                                     //     "m i",
///                                                     //     "psum ",
///                                                     //     "dolor",
///                                                     //     "sit",
///                                                     //     "am",
///                                                     //     "e",
///                                                     //     "t",
///                                                     //   ],
///                                                     //   "."
///                                                     // )
/// ```
pub fn fib_split_n_symmetric(text: &str, n: u32) -> (Vec<String>, &str) {
    let (mut words, rest) = fib_split_n(text, n);

    // If there is nothing to split, return the words and rest as they currently are
    if rest.is_empty() {
        return (words, rest);
    }

    // Roll the iterator to N - 1 iterations
    let mut fib_iter = FibIter::new();
    for _index in 0..n - 1 {
        fib_iter.next();
    }
    let mut rev_fib_iter: RevFibIter = fib_iter.rev();

    // Same logic as fib_split, but with a reversed iterator on words
    let mut current_substring_size: u32 = 0;
    let mut bytes_to_skip: usize = 0;
    words.push(String::from(""));

    for char in (*rest).chars() {
        words.last_mut().unwrap().push(char);
        current_substring_size += 1;
        bytes_to_skip += char.len_utf8();

        if current_substring_size == rev_fib_iter.number1 {
            let next = rev_fib_iter.next();
            if next.is_none() {
                break;
            }

            words.push(String::from(""));
            current_substring_size = 0;
        }
    }

    // Remove the bytes to skip from the rest of the text
    let rest = &rest[bytes_to_skip..];

    (words, rest)
}

#[cfg(test)]
mod test {
    use super::*;

    /// Basic test, which validates that all functions compile and run without panicking.
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

    /// Validates `FibIter` can be correctly initialized.
    #[test]
    fn test_fib_iter_new() {
        let fib_iter = FibIter::new();
        assert_eq!(1, fib_iter.number1);
        assert_eq!(1, fib_iter.number2);
    }

    /// Validates the first 8 `FibIter` itterations (produced by `next()`) are correctly created.
    #[test]
    fn test_fib_iter_next_8_times() {
        let mut fib_iter = FibIter::new();
        assert_eq!(1, fib_iter.number1);
        assert_eq!(1, fib_iter.number2);

        let result = fib_iter.next();
        assert_eq!(1, fib_iter.number1);
        assert_eq!(2, fib_iter.number2);
        assert_eq!(1, result);

        let result = fib_iter.next();
        assert_eq!(2, fib_iter.number1);
        assert_eq!(3, fib_iter.number2);
        assert_eq!(2, result);

        let result = fib_iter.next();
        assert_eq!(3, fib_iter.number1);
        assert_eq!(5, fib_iter.number2);
        assert_eq!(3, result);

        let result = fib_iter.next();
        assert_eq!(5, fib_iter.number1);
        assert_eq!(8, fib_iter.number2);
        assert_eq!(5, result);

        let result = fib_iter.next();
        assert_eq!(8, fib_iter.number1);
        assert_eq!(13, fib_iter.number2);
        assert_eq!(8, result);

        let result = fib_iter.next();
        assert_eq!(13, fib_iter.number1);
        assert_eq!(21, fib_iter.number2);
        assert_eq!(13, result);

        let result = fib_iter.next();
        assert_eq!(21, fib_iter.number1);
        assert_eq!(34, fib_iter.number2);
        assert_eq!(21, result);

        let result = fib_iter.next();
        assert_eq!(34, fib_iter.number1);
        assert_eq!(55, fib_iter.number2);
        assert_eq!(34, result);
    }

    /// Validates the first 8 `FibIterRev` itterations (produced by `next()`) are correctly created.
    #[test]
    fn test_fib_iter_rev_next_8_times() {
        let mut fib_iter = FibIter::new();
        fib_iter.number1 = 34;
        fib_iter.number2 = 55;

        let mut rev_fib_iter = fib_iter.rev();
        assert_eq!(34, rev_fib_iter.number1);
        assert_eq!(55, rev_fib_iter.number2);

        let result = rev_fib_iter.next();
        assert_eq!(21, rev_fib_iter.number1);
        assert_eq!(34, rev_fib_iter.number2);
        assert_eq!(21, result.unwrap());

        let result = rev_fib_iter.next();
        assert_eq!(13, rev_fib_iter.number1);
        assert_eq!(21, rev_fib_iter.number2);
        assert_eq!(13, result.unwrap());

        let result = rev_fib_iter.next();
        assert_eq!(8, rev_fib_iter.number1);
        assert_eq!(13, rev_fib_iter.number2);
        assert_eq!(8, result.unwrap());

        let result = rev_fib_iter.next();
        assert_eq!(5, rev_fib_iter.number1);
        assert_eq!(8, rev_fib_iter.number2);
        assert_eq!(5, result.unwrap());

        let result = rev_fib_iter.next();
        assert_eq!(3, rev_fib_iter.number1);
        assert_eq!(5, rev_fib_iter.number2);
        assert_eq!(3, result.unwrap());

        let result = rev_fib_iter.next();
        assert_eq!(2, rev_fib_iter.number1);
        assert_eq!(3, rev_fib_iter.number2);
        assert_eq!(2, result.unwrap());

        let result = rev_fib_iter.next();
        assert_eq!(1, rev_fib_iter.number1);
        assert_eq!(2, rev_fib_iter.number2);
        assert_eq!(1, result.unwrap());

        let result = rev_fib_iter.next();
        assert_eq!(1, rev_fib_iter.number1);
        assert_eq!(1, rev_fib_iter.number2);
        assert_eq!(1, result.unwrap());

        let result = rev_fib_iter.next();
        assert_eq!(1, rev_fib_iter.number1);
        assert_eq!(1, rev_fib_iter.number2);
        assert_eq!(None, result);
    }

    /// Validates that `fib_split()` function correctly splits the input string into words with
    /// lengths that match the Fibonacc numbers.
    #[test]
    fn test_fib_split_ascii() {
        let input = "Fibonacci words!";
        let expected_result = vec!["F", "i", "bo", "nac", "ci wo", "rds!"];
        let actual_result = fib_split(input);

        assert_eq!(expected_result, actual_result);
    }

    /// Validates that `fib_split()` function correctly splits an empty string into an empty vector.
    #[test]
    fn test_fib_split_empty() {
        let input = "";
        let expected_result = vec![""];
        let actual_result = fib_split(input);

        assert_eq!(expected_result, actual_result);
    }

    /// Validates that `fib_split()` function correctly splits a string with Cyrillic UTF-8
    /// characters.
    #[test]
    fn test_fib_split_cyrillic() {
        let input = "Гошо Лошо се обади на авера си Пошо Мошо за да се срещнат с Тошо Рошо. 123";
        let expected_result = vec![
            "Г",
            "о",
            "шо",
            " Ло",
            "шо се",
            " обади н",
            "а авера си По",
            "шо Мошо за да се срещ",
            "нат с Тошо Рошо. 123",
        ];
        let actual_result = fib_split(input);

        assert_eq!(expected_result, actual_result);
    }

    /// Validates that `fib_split()` can correctly process UTF-8 emojis in strings.
    #[test]
    fn test_fib_split_emoji() {
        let input = "💀💀💀💀💀";
        let expected_result = vec!["💀", "💀", "💀💀", "💀"];
        let actual_result = fib_split(input);

        assert_eq!(expected_result, actual_result);
    }

    /// Validates that `fib_split_n()` function correctly splits the input string into N words
    /// with lengths that match the Fibonacci numbers. Also, validates the rest of the string is
    /// correctly returned.
    #[test]
    fn test_fib_split_n_ascii() {
        let input_string = "Lorem ipsum dolor sit amet.";
        let input_number = 6;
        let expected_result_vector = vec!["L", "o", "re", "m i", "psum ", "dolor si"];
        let expected_result_rest = "t amet.";
        let actual_result = fib_split_n(input_string, input_number);

        assert_eq!(expected_result_vector, actual_result.0);
        assert_eq!(expected_result_rest, actual_result.1);
    }

    /// Validates that `fib_split_n()` can correctly process UTF-8 Cyrillic characters in strings.
    #[test]
    fn test_fib_split_n_cyrillic() {
        let input_string = "Гошо Лошо се обади на авера си Пошо Мошо. 123";
        let input_number = 7;
        let expected_result_vector =
            vec!["Г", "о", "шо", " Ло", "шо се", " обади н", "а авера си По"];
        let expected_result_rest = "шо Мошо. 123";
        let actual_result = fib_split_n(input_string, input_number);

        assert_eq!(expected_result_vector, actual_result.0);
        assert_eq!(expected_result_rest, actual_result.1);
    }

    /// Validates that `fib_split_n()` function correctly splits a short string into words.
    #[test]
    fn test_fib_split_n_short() {
        let input_string = "Lorem.";
        let input_number = 5;
        let expected_result_vector = vec!["L", "o", "re", "m."];
        let expected_result_rest = "";
        let actual_result = fib_split_n(input_string, input_number);

        assert_eq!(expected_result_vector, actual_result.0);
        assert_eq!(expected_result_rest, actual_result.1);
    }

    /// Validates that `fib_split_n()` can correctly process UTF-8 emojis in strings.
    #[test]
    fn test_fib_split_n_emoji() {
        // This is the most cursed test I've ever written...
        let input_string =
            "🦀🌝🍑😩👀🥹🫡🦀🦀🦀🦀🦀👉👈🥺3.0🦀🙏🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🍑🍑🍑🍑🍑🍑🍑🍑";
        let input_number = 7;
        let expected_result_vector = vec![
            "🦀",
            "🌝",
            "🍑😩",
            "👀🥹🫡",
            "🦀🦀🦀🦀🦀",
            "👉👈🥺3.0🦀🙏",
            "🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀",
        ];
        let expected_result_rest = "🍑🍑🍑🍑🍑🍑🍑🍑";
        let actual_result = fib_split_n(input_string, input_number);

        assert_eq!(expected_result_vector, actual_result.0);
        assert_eq!(expected_result_rest, actual_result.1);
    }

    /// Validates that `fib_split_n_symmetric()` function correctly splits the input
    /// string into 2N symmetric words with lengths that match the Fibonacci numbers
    /// up to N iterations and then in reverse. Also, validates the rest of the
    /// string is correctly returned.
    #[test]
    fn test_fib_split_n_symmetric_ascii() {
        let input_string = "Lorem ipsum dolor sit amet.";
        let input_number = 5;
        let expected_result_vector = vec![
            "L", "o", "re", "m i", "psum ", "dolor", " si", "t ", "a", "m",
        ];
        let expected_result_rest = "et.";
        let actual_result = fib_split_n_symmetric(input_string, input_number);

        assert_eq!(expected_result_vector, actual_result.0);
        assert_eq!(expected_result_rest, actual_result.1);
    }

    /// Validates that `fib_split_n_symmetric()` can correctly process UTF-8 Cyrillic characters in strings.
    #[test]
    fn test_fib_split_n_symmetric_cyrillic() {
        let input_string = "Гошо Лошо се обади на авера си Пошо Мошо. 123";
        let input_number = 5;
        let expected_result_vector = vec![
            "Г",
            "о",
            "шо",
            " Ло",
            "шо се",
            " обад",
            "и н",
            "а ",
            "а",
            "в",
        ];
        let expected_result_rest = "ера си Пошо Мошо. 123";
        let actual_result = fib_split_n_symmetric(input_string, input_number);

        assert_eq!(expected_result_vector, actual_result.0);
        assert_eq!(expected_result_rest, actual_result.1);
    }

    /// Validates that `fib_split_n()` can correctly process UTF-8 emojis in strings.
    #[test]
    fn test_fib_split_n_symmetric_emoji() {
        // This is the most cursed test I've ever written...
        let input_string =
            "🦀🌝🍑😩👀🥹🫡🦀🦀🦀🦀🦀👉👈🥺3.0🦀🙏🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🍑🍑🍑🍑🍑🍑🍑🍑";
        let input_number = 6;
        let expected_result_vector = vec![
            "🦀",
            "🌝",
            "🍑😩",
            "👀🥹🫡",
            "🦀🦀🦀🦀🦀",
            "👉👈🥺3.0🦀🙏",
            "🦀🦀🦀🦀🦀🦀🦀🦀",
            "🦀🦀🦀🦀🦀",
            "🍑🍑🍑",
            "🍑🍑",
            "🍑",
            "🍑",
        ];
        let expected_result_rest = "🍑";
        let actual_result = fib_split_n_symmetric(input_string, input_number);

        assert_eq!(expected_result_vector, actual_result.0);
        assert_eq!(expected_result_rest, actual_result.1);
    }

    /// Validates that `fib_split()` function correctly splits a short string into words.
    #[test]
    fn test_fib_split_n_symmetric_short() {
        let input_string = "Lorem.";
        let input_number = 5;
        let expected_result_vector = vec!["L", "o", "re", "m."];
        let expected_result_rest = "";
        let actual_result = fib_split_n_symmetric(input_string, input_number);

        assert_eq!(expected_result_vector, actual_result.0);
        assert_eq!(expected_result_rest, actual_result.1);
    }
}
