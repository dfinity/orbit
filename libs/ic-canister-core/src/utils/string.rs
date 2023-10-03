use convert_case::{Case, Casing};

/// Converts a string to snake case.
pub fn to_snake_case(input: String) -> String {
    input.to_case(Case::Snake)
}
