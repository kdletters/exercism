use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_word = word.to_lowercase();
    let sorted_word = get_sorted(&lower_word);
    possible_anagrams
        .iter()
        .filter(|anagram| {
            let lower = anagram.to_lowercase();
            lower.len() == lower_word.len()
                && lower != lower_word
                && get_sorted(&lower) == sorted_word
        })
        .copied()
        .collect()
}

fn get_sorted(word: &str) -> Vec<char> {
    let mut sorted = word.chars().collect::<Vec<char>>();
    sorted.sort_unstable();
    sorted
}
