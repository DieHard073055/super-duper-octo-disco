use std::collections::HashMap;

struct Solution {}
impl Solution {
    fn map_from(s: &str) -> HashMap<char, i32> {
        let mut new_map = HashMap::new();
        s.chars()
            .into_iter()
            .for_each(|letter| *new_map.entry(letter).or_insert(0) += 1);
        new_map
    }
    fn inclusion(window: &HashMap<char, i32>, s1_map: &HashMap<char, i32>) -> bool {
        let mut result = true;
        s1_map.into_iter().for_each(|(s1_letter, s1_freq)| {
            if let Some(window_freq) = window.get(s1_letter) {
                if window_freq != s1_freq {
                    result = false;
                }
            } else {
                result = false;
            }
        });
        result
    }
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }
        let s1_map = Solution::map_from(s1.as_str());
        let s2_letters = s2.chars().into_iter().collect::<Vec<char>>();

        if s1.len() == 1 {
            if let Some(_) = s2.find(&s1) {
                return true;
            } else {
                return false;
            }
        }
        let mut window = HashMap::new();
        let window_len = s1.len();

        let mut start = 0;
        let mut end = 0;

        // println!("s1: {:}, s2: {:}", s1, s2);

        // add letters to window
        for _ in 0..window_len {
            let next_letter = s2_letters[end];
            // println!("next_letter = {:}", next_letter);
            *window.entry(next_letter).or_insert(0) += 1;
            end += 1;
        }
        if Solution::inclusion(&window, &s1_map) {
            return true;
        } else {
            if end == s2.len() {
                return false;
            }
        }
        // println!("window: {:?}", window);
        // println!("s1_map: {:?}", s1_map);

        while end < s2.len() {
            // println!("window: {:?}", window);
            // println!("start: {:}, end: {:}", start, end);

            while (end - start) <= window_len {
                let next_letter = s2_letters[end];
                end += 1;
                *window.entry(next_letter).or_insert(0) += 1;
                // println!("next_letter = {:}", next_letter);
            }

            while (end - start) > window_len {
                let first_letter = s2_letters[start];
                start += 1;
                // println!("first_letter = {:}", first_letter);
                *window.entry(first_letter).or_insert(0) -= 1;
                if let Some(value) = window.get(&first_letter) {
                    if value <= &0 {
                        window.remove(&first_letter);
                    }
                }
            }
            // println!("window: {:?}", window);
            if Solution::inclusion(&window, &s1_map) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_example_1() {
        let s1 = "ab".to_string();
        let s2 = "eidbaooo".to_string();
        assert_eq!(Solution::check_inclusion(s1, s2), true);
    }

    #[test]
    #[ignore]
    fn test_example_2() {
        let s1 = "ab".to_string();
        let s2 = "eidboaoo".to_string();
        assert_eq!(Solution::check_inclusion(s1, s2), false);
    }

    #[test]
    #[ignore]
    fn test_empty_s1() {
        let s1 = "".to_string();
        let s2 = "anything".to_string();
        assert_eq!(Solution::check_inclusion(s1, s2), true);
    }

    #[test]
    #[ignore]
    fn test_s1_longer_than_s2() {
        let s1 = "abcdefg".to_string();
        let s2 = "abcde".to_string();
        assert_eq!(Solution::check_inclusion(s1, s2), false);
    }

    #[test]
    #[ignore]
    fn test_s1_equal_to_s2() {
        let s1 = "abcde".to_string();
        let s2 = "abcde".to_string();
        assert_eq!(Solution::check_inclusion(s1, s2), true);
    }

    #[test]
    #[ignore]
    fn test_no_permutation_found() {
        let s1 = "abc".to_string();
        let s2 = "defghijklmnopqrstuvwxyz".to_string();
        assert_eq!(Solution::check_inclusion(s1, s2), false);
    }

    #[test]
    #[ignore]
    fn test_single_letter_s1() {
        let s1 = "a".to_string();
        let s2 = "ab".to_string();
        assert_eq!(Solution::check_inclusion(s1, s2), true);
    }
    #[test]
    fn test_triple_letter_s1() {
        let s1 = "adc".to_string();
        let s2 = "dcda".to_string();
        assert_eq!(Solution::check_inclusion(s1, s2), true);
    }
}
