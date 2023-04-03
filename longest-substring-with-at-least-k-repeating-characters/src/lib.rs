use std::collections::HashMap;

struct Solution {}
impl Solution {
    fn count_all_unique_letters(s: &str) -> usize {
        let mut s_map = HashMap::new();
        s.chars()
            .into_iter()
            .for_each(|letter| *s_map.entry(letter).or_insert(0) += 1);
        return s_map.keys().len();
    }
    fn count_unique_letters(window: &HashMap<char, i32>) -> usize {
        window.keys().len()
    }
    fn unique_letters_with_at_least(window: &HashMap<char, i32>, k: i32) -> i32 {
        let mut result = 0i32;
        let mut non_k_repeat = false;
        window.into_iter().for_each(|(_, freq)| {
            if freq >= &k {
                result += freq;
            } else {
                non_k_repeat = true;
            }
        });
        if !non_k_repeat {
            result
        } else {
            0i32
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
        let mut start: i32 = -1;
        let mut end: i32 = -1;
        let mut window: HashMap<char, i32> = HashMap::new();
        let mut max_substring = 0;
        let s_unique_keys = Solution::count_all_unique_letters(s.as_str());
        // println!("s_unique_keys: {:?}", s_unique_keys);

        for current_unique in 0..s_unique_keys {
            // println!("current_unique: {:}", current_unique);
            while end < s_letters.len() as i32 - 1 {
                // println!("s: {:}", s);
                // println!("k: {:}", k);
                // println!("window: {:?}", window);

                // println!("max_substring: {:}", max_substring);
                if Solution::count_unique_letters(&window) <= (current_unique + 1) {
                    end += 1;
                    let current_letter = s_letters[end as usize];
                    // println!("end: {:}", end);
                    // println!("current_letter: {:}", current_letter);
                    *window.entry(current_letter).or_insert(0) += 1;
                } else {
                    start += 1;
                    let start_letter = s_letters[start as usize];
                    // println!("start: {:}", start);
                    // println!("start_letter: {:}", start_letter);
                    *window.entry(start_letter).or_insert(0) -= 1;
                    if window.get(&start_letter) < Some(&1) {
                        window.remove(&start_letter);
                    }
                }
                let current_k_unique_letter_count =
                    Solution::unique_letters_with_at_least(&window, k);
                // println!(
                //     "current_k_unique_letter_count: {:}",
                //     current_k_unique_letter_count
                // );
                if current_k_unique_letter_count > max_substring {
                    max_substring = current_k_unique_letter_count;
                }
                // println!("window: {:?}", window);
                // println!("\n");
            }
            end = -1;
            start = -1;
            window = HashMap::new();
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
            ("bbaaacbd", 3, 3),
        ];

        for (s, k, expected) in cases {
            assert_eq!(Solution::longest_substring(s.to_string(), k), expected);
        }
    }
}
