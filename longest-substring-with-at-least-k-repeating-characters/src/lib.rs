use std::collections::HashMap;

enum WindowStatus {
    AllValid(i32),
    Valid((Vec<char>, i32)),
    NoValid,
}
struct Solution {}
impl Solution {
    fn len_repeating_substring_in_window(window: &HashMap<char, i32>, k: &i32) -> WindowStatus {
        let mut string_count = 0;
        let mut valid = true;
        let mut invalid_chars = vec![];
        for (key, val) in window.iter() {
            if val >= &k {
                string_count += val;
            } else {
                valid = false;
                invalid_chars.push(key.clone());
            }
        }
        if string_count > 0 {
            if valid {
                WindowStatus::AllValid(string_count)
            } else {
                WindowStatus::Valid((invalid_chars, string_count))
            }
        } else {
            WindowStatus::NoValid
        }
    }
    pub fn longest_substring(s: String, k: i32) -> i32 {
        if k == 1 {
            return s.len() as i32;
        }
        if s.len() < 1 {
            return 0;
        }
        let s_letters = s.chars().collect::<Vec<char>>();
        let mut start = 0;
        let mut end: i32 = -1;
        let mut window = HashMap::new();
        let mut max_substring = 0;

        while end < s_letters.len() as i32 - 1 {
            end += 1;
            let next_letter = s_letters[end as usize];
            *window.entry(next_letter).or_insert(0) += 1;

            loop {
                match Solution::len_repeating_substring_in_window(&window, &k) {
                    WindowStatus::NoValid => {
                        break;
                    }
                    WindowStatus::Valid((invalid_chars, string_count)) => {
                        let window_size = end - start;
                        if ((window_size - string_count) - invalid_chars.len() as i32) > 1 {
                            let previous_letter = s_letters[start as usize];
                            start += 1;
                            *window.entry(previous_letter).or_insert(0) -= 1;
                            if window.get(&previous_letter) == Some(&0) {
                                window.remove(&previous_letter);
                            }
                            if string_count > max_substring {
                                max_substring = string_count;
                            }
                        } else {
                            break;
                        }
                    }
                    WindowStatus::AllValid(string_count) => {
                        if string_count > max_substring {
                            max_substring = string_count;
                        }
                        break;
                    }
                }
            }
        }

        max_substring
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_substring() {
        let cases = vec![
            ("aaabb", 3, 3),
            ("ababbc", 2, 5),
            ("abcabc", 1, 6),
            ("aaabbbccc", 3, 9),
            ("aaabbc", 3, 3),
            ("aabbcc", 2, 6),
            ("abcdef", 2, 0),
            ("", 1, 0),
            ("ababbc", 2, 5),
            ("ababacb", 3, 0),
            // ("bbaaacbd", 3, 3),
        ];

        for (s, k, expected) in cases {
            assert_eq!(Solution::longest_substring(s.to_string(), k), expected);
        }
    }
}
