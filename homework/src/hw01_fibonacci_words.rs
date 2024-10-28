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
                                             // similar to iterator trait name.
    pub fn next(&mut self) -> u32 {
        let number2_backup = self.number2;
        self.number2 += self.number1;
        self.number1 = number2_backup;

        self.number1
    }

    pub fn rev(self) -> RevFibIter {
        RevFibIter {
            number1: self.number1,
            number2: self.number2,
        }
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

        // let (_words, _rest): (Vec<String>, &str) = fib_split_n_symmetric(
        //     "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        //     1,
        // );
    }

    /// Validates `FibIter` can be correctly initialized.
    #[test]
    fn test_fib_iter_new() {
        let fib_iter = FibIter::new();
        assert_eq!(1, fib_iter.number1);
        assert_eq!(1, fib_iter.number2);
    }

    /// Validates the fist 8 `FibIter` itterations (produced by `next()`) are correctly created.
    #[test]
    fn test_fib_iter_next_8_times() {
        let mut fib_iter = FibIter::new();
        assert_eq!(1, fib_iter.number1);
        assert_eq!(1, fib_iter.number2);

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
    fn test_fib_iter_rev_next_8_times() {
        let mut fib_iter = FibIter::new();
        fib_iter.number1 = 34;
        fib_iter.number2 = 55;

        let mut rev_fib_iter = fib_iter.rev();
        assert_eq!(34, rev_fib_iter.number1);
        assert_eq!(55, rev_fib_iter.number2);

        rev_fib_iter.next();
        assert_eq!(21, rev_fib_iter.number1);
        assert_eq!(34, rev_fib_iter.number2);

        rev_fib_iter.next();
        assert_eq!(13, rev_fib_iter.number1);
        assert_eq!(21, rev_fib_iter.number2);

        rev_fib_iter.next();
        assert_eq!(8, rev_fib_iter.number1);
        assert_eq!(13, rev_fib_iter.number2);

        rev_fib_iter.next();
        assert_eq!(5, rev_fib_iter.number1);
        assert_eq!(8, rev_fib_iter.number2);

        rev_fib_iter.next();
        assert_eq!(3, rev_fib_iter.number1);
        assert_eq!(5, rev_fib_iter.number2);

        rev_fib_iter.next();
        assert_eq!(2, rev_fib_iter.number1);
        assert_eq!(3, rev_fib_iter.number2);

        rev_fib_iter.next();
        assert_eq!(1, rev_fib_iter.number1);
        assert_eq!(2, rev_fib_iter.number2);

        rev_fib_iter.next();
        assert_eq!(1, rev_fib_iter.number1);
        assert_eq!(1, rev_fib_iter.number2);
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

    /// Validates that `fib_split()` function correctly splits a string with cyrillic UTF-8
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

    /// Validates that `fib_split_n()` can correctly process UTF-8 cyrillic characters in strings.
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

    /// Validates that `fib_split_n()` can correctly process UTF-8 emojis in strings.
    #[test]
    fn test_fib_split_n_emoji() {
        // This is the most cursed test I've ever written...
        let input_string =
            "🦀🌝🍑😩👀🥹🫡🦀🦀🦀🦀🦀👉👈🥺3.0🦀🙏🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🍑🍑🍑🍑🍑🍑";
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
        let expected_result_rest = "🍑🍑🍑🍑🍑🍑";
        let actual_result = fib_split_n(input_string, input_number);

        assert_eq!(expected_result_vector, actual_result.0);
        assert_eq!(expected_result_rest, actual_result.1);
    }
}
