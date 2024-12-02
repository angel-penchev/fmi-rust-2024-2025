use std::collections::HashSet;
use std::fmt;

pub struct Import<'a>(pub &'a [&'a str]);

pub enum Order {
    Original,
    Sorted,
}

// As per Clippy recommendation, implemented fmt::Display instead of ToString trait.
impl<'a> fmt::Display for Import<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join("::"))
    }
}

pub fn format_flat(imports: &[Import], order: Order) -> Vec<String> {
    let mut processed = HashSet::new();
    imports
        .iter()
        .map(|import| import.to_string())
        .filter(|import| processed.insert(import.clone())) // .insert() returns true if the value was not present before
        .collect()
}

pub fn format_nested(imports: &[Import], order: Order) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    /// Validate that the `format_flat` function works as expected, if the original order is kept.
    #[test]
    fn test_format_flat_original() {
        let imports = &[
            Import(&["my_crate", "a"]),
            Import(&["my_crate", "b", "B1"]),
            Import(&["my_crate", "b", "B2"]),
        ];
        let order = Order::Original;

        let expected_result = vec![
            String::from("my_crate::a"),
            String::from("my_crate::b::B1"),
            String::from("my_crate::b::B2"),
        ];
        let actual_result = format_flat(imports, order);

        assert_eq!(expected_result, actual_result);
    }

    /// Validate that the `format_flat` function does not display any duplicates.
    #[test]
    fn test_format_flat_original_with_duplicates() {
        let imports = &[
            Import(&["my_crate", "a"]),
            Import(&["my_crate", "b", "B1"]),
            Import(&["my_crate", "b", "B1"]),
            Import(&["my_crate", "b", "B2"]),
            Import(&["my_crate", "b", "B2"]),
        ];
        let order = Order::Original;

        let expected_result = vec![
            String::from("my_crate::a"),
            String::from("my_crate::b::B1"),
            String::from("my_crate::b::B2"),
        ];
        let actual_result = format_flat(imports, order);

        assert_eq!(expected_result, actual_result);
    }
}
