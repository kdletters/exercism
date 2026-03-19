pub fn reverse(input: &str) -> String {
    String::from_iter(input.chars().rev().collect::<Vec<_>>())
}