use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    let mut word_chars: Vec<char> = word.to_lowercase().chars().collect();
    for p in possible_anagrams {
        if *p.to_lowercase() == word.to_lowercase() {
            continue;
        }
        let mut chars: Vec<char> = p.to_lowercase().chars().collect();
        chars.sort();
        word_chars.sort();
        if chars == word_chars {
            result.insert(*p);
        }
    }
    result
}
