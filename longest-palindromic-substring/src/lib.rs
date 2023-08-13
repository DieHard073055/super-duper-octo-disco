struct Solution {}
impl Solution {
    fn is_all_same_characters(letters: &Vec<char>) -> bool {
        let first_letter = letters[0];
        for letter in letters {
            if letter != &first_letter {
                return false;
            }
        }
        true
    }
    fn is_palindrom(letters: &Vec<char>) -> bool {
        // println!("is_palindrom {:?}", letters);
        let half_len = letters.len() as i32 / 2i32;
        // println!("half len: {:}", half_len);
        for i in 0..half_len {
            // println!(
            //     "letters[{:}]: ==  letter[{:}]",
            //     i,
            //     letters.len() - 1 - i as usize
            // );
            // println!(
            //     "letters[{:}]: ==  letter[{:}]",
            //     letters[i as usize],
            //     letters[(letters.len() - 1 - i as usize)]
            // );
            if letters[i as usize] != letters[(letters.len() - 1 - i as usize)] {
                return false;
            }
        }
        true
    }
    pub fn longest_palindrome(s: String) -> String {
        let s_len = s.len();
        let s_letters = s.chars().collect::<Vec<char>>();
        if s_len < 3 {
            match s_letters.len() {
                1 => {
                    return s;
                }
                2 => match (s_letters[0], s_letters[1]) {
                    (a, b) if a == b => return s,
                    (a, _) => return a.to_string(),
                },
                _ => unreachable!("string lengths s < 1 and s > 2 should not exist here."),
            }
        }
        if Solution::is_all_same_characters(&s_letters) {
            return s;
        }
        let window_max_size = s_len + 1;
        let mut palindrome = String::from(s[0..1].to_string());

        // println!("window max: {:}", window_max_size);
        for window_size in 1..window_max_size {
            let window_start_max = window_max_size - window_size;
            // println!("window start max: {:}", window_start_max);
            for window_start in 0..window_start_max {
                let window_end = window_start + window_size;
                let window_string = (&s[window_start..window_end]).to_string();
                let window_string_letters = window_string.chars().collect::<Vec<char>>();
                // println!("{:}", &s[window_start..window_end]);
                // println!("{:}, {:}", window_start, window_end);
                if window_string_letters.len() == 2 {
                    if window_string_letters[0] == window_string_letters[1] {
                        // println!("panlindrome set: {:}", window_string);
                        palindrome = window_string;
                    }
                } else if window_string_letters.len() > 2 {
                    if Solution::is_palindrom(&window_string_letters) {
                        // println!("panlindrome set: {:}", window_string);
                        palindrome = window_string;
                    }
                }
            }
        }

        palindrome
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        let test_cases = vec![
            // ("babad", vec!["bab", "aba"]),
            // ("cbbd", vec!["bb"]),
            // ("a", vec!["a"]),
            // ("ac", vec!["a", "c"]),
            // ("bb", vec!["bb"]),
            // ("aaaa", vec!["aaaa"]),
            ("abcba", vec!["abcba"]),
        ];

        for (input, expected_outputs) in test_cases {
            let result = Solution::longest_palindrome(input.to_string());
            assert!(
                expected_outputs.contains(&result.as_str()),
                "Expected one of {:?} for input '{}', but got '{}'",
                expected_outputs,
                input,
                result
            );
        }
    }
}
