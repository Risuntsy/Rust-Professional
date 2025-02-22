/*
    Longest Substring Without Repeating Characters
    Given a string, find the length of the longest substring without repeating characters. 
    The substring must not contain any duplicate characters, and its length should be maximized.

    You need to implement the function `longest_substring_without_repeating_chars(s: String) -> i32`.
    The function should return the length of the longest substring without repeating characters.
    
    Hint: Consider using the sliding window technique to efficiently solve this problem in O(n) time complexity.
*/

use std::fmt::{self, Display, Formatter};

pub fn longest_substring_without_repeating_chars(s: String) -> i32 {
    let mut longest_length = 0;
    let mut repetition = [false;26];

    let mut start = 0;

    let chars = s.chars().collect::<Vec<char>>();

    for (i, c) in chars.iter().enumerate() {
        println!("i: {}, c: {}, start: {}, longest_length: {}", i, c, start, longest_length);

        let c_index = (*c as u8 - 'a' as u8) as usize;

        if repetition[c_index] {
            longest_length = longest_length.max(i - start);

            for c in chars[start..i].iter() {
                repetition[(*c as u8 - 'a' as u8) as usize] = false;
            }

            start = i;
        } else {
            longest_length = longest_length.max(i - start + 1);
        }

        repetition[c_index] = true;
    }

    longest_length as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_substring_1() {
        let s = "abcabcbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3);  // "abc"
    }

    #[test]
    fn test_longest_substring_2() {
        let s = "bbbbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 1);  // "b"
    }

    #[test]
    fn test_longest_substring_3() {
        let s = "pwwkew".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3);  // "wke"
    }

    #[test]
    fn test_longest_substring_4() {
        let s = "".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 0);  // Empty string
    }

    #[test]
    fn test_longest_substring_5() {
        let s = "abcde".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 5);  // "abcde"
    }
}
