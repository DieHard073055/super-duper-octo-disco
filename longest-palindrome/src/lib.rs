use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        println!("{:}", s);
        let mut word_map = HashMap::new();
        let mut doubles_counter = 0;
        let mut single = false;
        s.chars()
            .for_each(|letter| *word_map.entry(letter).or_insert(0) += 1);
        for (letter, qty) in word_map.iter() {
            let doubles = i32::from(qty / 2);
            println!("{:} {:} {:}", letter, qty, doubles);
            if !single {
                if doubles == 0 {
                    single = true;
                }
                if (doubles * 2) == (qty - 1) {
                    single = true
                }
            }
            doubles_counter += doubles;
        }
        let mut result = doubles_counter * 2;
        if single {
            result += 1
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(Solution::longest_palindrome(String::from("abccccdd")), 7);
        assert_eq!(Solution::longest_palindrome(String::from("yvssgssvyy")), 9);
        assert_eq!(Solution::longest_palindrome(String::from("a")), 1);
        assert_eq!(Solution::longest_palindrome(String::from("")), 0);
        assert_eq!(Solution::longest_palindrome(String::from("abcde")), 1);
        assert_eq!(Solution::longest_palindrome(String::from("Aa")), 1);
        assert_eq!(Solution::longest_palindrome(String::from("ccc")), 3);
        assert_eq!(
            Solution::longest_palindrome(String::from("aaaaaaaaddc")),
            11
        );
        assert_eq!(
            Solution::longest_palindrome(String::from("Bbbbbbbvvvvvmmmoo")),
            15
        );
    }
}
