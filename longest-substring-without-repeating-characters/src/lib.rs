use std::collections::HashMap;

struct Solution {}
impl Solution {
    fn length_of_window(window: &HashMap<char, i32>) -> i32 {
        window.values().sum::<i32>().clone()
    }
    fn duplicates_in(window: &HashMap<char, i32>) -> bool {
        if window.is_empty() {
            return false;
        }
        !window.values().all(|&freq| freq <= 1)
    }
    fn check_dup_update_max_length(max_length: &mut i32, window: &HashMap<char, i32>) {
        let dups = Solution::duplicates_in(&window);
        if !dups {
            let window_len = Solution::length_of_window(&window);
            if window_len > *max_length {
                *max_length = window_len;
            }
        }
    }
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s_letters = s.chars().collect::<Vec<char>>();
        if s_letters.len() < 1 {
            return 0;
        } else if s_letters.len() == 1 {
            return 1;
        }
        let mut start = 0;
        let mut end = 0;

        let mut window = HashMap::new();
        let mut max_length = 0i32;

        while (end as usize) < s_letters.len() {
            Solution::check_dup_update_max_length(&mut max_length, &window);
            let next_letter = s_letters[end];
            *window.entry(next_letter).or_insert(0) += 1;
            end += 1;
            while Solution::duplicates_in(&window) {
                let first_letter = s_letters[start];
                *window.entry(first_letter).or_insert(0) -= 1;
                start += 1;
                if let Some(freq) = window.get(&first_letter) {
                    if freq < &1 {
                        window.remove(&first_letter);
                    }
                }
            }
        }
        Solution::check_dup_update_max_length(&mut max_length, &window);
        max_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = "abcabcbb".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 3);
    }

    #[test]
    fn test_example_2() {
        let s = "bbbbb".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 1);
    }

    #[test]
    fn test_example_3() {
        let s = "pwwkew".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 3);
    }

    #[test]
    fn test_empty_string() {
        let s = "".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 0);
    }

    #[test]
    fn test_single_character() {
        let s = "a".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 1);
    }

    #[test]
    fn test_all_unique_characters() {
        let s = "abcdef".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 6);
    }

    #[test]
    fn test_repeating_characters() {
        let s = "abba".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 2);
    }
}
