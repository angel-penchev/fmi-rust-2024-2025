use std::collections::{HashMap, HashSet};
use std::fmt;

pub struct Import<'a>(pub &'a [&'a str]);

#[derive(PartialEq, Eq)]
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

struct ModuleTree {
    root: ModuleNode,
}

struct ModuleNode {
    value: String,
    children: Vec<ModuleNode>,
}

impl ModuleNode {
    fn new(value: String) -> Self {
        Self {
            value,
            children: Vec::new(),
        }
    }
}

pub fn format_flat(imports: &[Import], order: Order) -> Vec<String> {
    let mut processed = HashSet::new();
    let mut results: Vec<String> = imports
        .iter()
        .map(|import| import.to_string())
        .filter(|import| processed.insert(import.clone())) // .insert() returns true if the value was not present before
        .collect();

    if order == Order::Sorted {
        results.sort()
    }

    results
}

pub fn format_nested(imports: &[Import], order: Order) -> Vec<String> {
    let mut modules: HashMap<&str, Vec<&[&str]>> = HashMap::new();

    for import in imports {
        if let Some((module_name, module_path)) = import.0.split_first() {
            let current_module = modules.entry(module_name).or_default();
            current_module.push(module_path);
        }
    }

    let result: Vec<String> = Vec::new(); // TODO
    result
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

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

    /// Validate that the `format_flat` function works as expected, if the imports are to be sorted.
    #[test]
    fn test_format_flat_sorted() {
        let imports = &[
            Import(&["my_crate", "b", "B2"]),
            Import(&["my_crate", "a"]),
            Import(&["my_crate", "b", "B1"]),
        ];
        let order = Order::Sorted;

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
    fn test_format_flat_sorted_with_duplicates() {
        let imports = &[
            Import(&["my_crate", "b", "B1"]),
            Import(&["my_crate", "b", "B2"]),
            Import(&["my_crate", "a"]),
            Import(&["my_crate", "b", "B2"]),
            Import(&["my_crate", "b", "B1"]),
            Import(&["my_crate", "b", "B2"]),
            Import(&["my_crate", "b", "B2"]),
            Import(&["my_crate", "a"]),
            Import(&["my_crate", "a"]),
        ];
        let order = Order::Sorted;

        let expected_result = vec![
            String::from("my_crate::a"),
            String::from("my_crate::b::B1"),
            String::from("my_crate::b::B2"),
        ];
        let actual_result = format_flat(imports, order);

        assert_eq!(expected_result, actual_result);
    }

    /// Validate that the `format_nested` function works as expected, if the original order is kept.
    #[test]
    fn test_format_nested_original() {
        let imports = &[
            Import(&["std", "a"]),
            Import(&["std", "b"]),
            Import(&["foo", "b"]),
        ];
        let order = Order::Original;

        let expected_result = vec![
            String::from("std::{\n    a,\n    b\n}\n"),
            String::from("foo::{\n    b,\n}\n"),
        ];
        let actual_result = format_nested(imports, order);

        assert_eq!(expected_result, actual_result);
    }

    /// Validate that the `format_nested` function works as expected, if the original order is kept.
    #[test]
    fn test_format_nested_original_complex() {
        let imports = &[
            Import(&["std", "a"]),
            Import(&["std", "b"]),
            Import(&["std", "c", "ca"]),
            Import(&["std", "c", "cb"]),
            Import(&["foo", "b"]),
        ];
        let order = Order::Original;

        let expected_result = vec![
            String::from("std::{\n    a,\n    b,\n    c::{\n        ca,\n        cb\n    }\n}\n"),
            String::from("foo::{\n    b,\n}\n"),
        ];
        let actual_result = format_nested(imports, order);

        assert_eq!(expected_result, actual_result);
    }
}
