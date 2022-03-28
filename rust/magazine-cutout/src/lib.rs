// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut clone_magazine = magazine.to_vec();
    let mut results = Vec::new();
    for word in note.iter() {
        if let Some(index) = clone_magazine.iter().position(|value| value == word) {
            clone_magazine.swap_remove(index);
        } else {
            results.push(false)
        }
    }
    !results.contains(&false)
}
