//!Given a string, find the length of the longest substring
//!without repeating characters.
//!
//!Examples:
//!Given "abcabcbb", the answer is "abc", which the length is 3.
//!Given "bbbbb", the answer is "b", with the length of 1.
//!Given "pwwkew", the answer is "wke", with the length of 3.
//!Note that the answer must be a substring,
//!"pwke" is a subsequence and not a substring.

use std::collections::HashMap;
use std::cmp::max;

///Find the length of the longest substring
///without repeating characters.
pub fn longest_non_repeat(string: &str) -> usize {
    let mut used = HashMap::<char, usize>::with_capacity(string.len());
    let mut max_length = 0;
    let mut j = 0;
    for (i, ch) in string.chars().enumerate() {
        if let Some(&n) = used.get(&ch) {
            j = max(n, j);
        }
        used.insert(ch, i + 1);
        max_length = max(max_length, i - j + 1);
    }
    max_length
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_non_repeat() {
        assert_eq!(3, longest_non_repeat("abcabcbb"));
    }
}