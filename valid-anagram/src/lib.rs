use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut s_map: HashMap<char, i32> = HashMap::new();
        s.chars().for_each(|c| *s_map.entry(c).or_insert(0) += 1);
        t.chars().for_each(|c| *s_map.entry(c).or_insert(0) -= 1);
        s_map.into_values().all(|n| n == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        // Test case 1
        let s1 = "anagram".to_string();
        let t1 = "nagaram".to_string();
        assert_eq!(Solution::is_anagram(s1, t1), true);

        // Test case 2
        let s2 = "rat".to_string();
        let t2 = "car".to_string();
        assert_eq!(Solution::is_anagram(s2, t2), false);

        // // Test case 3
        let s3 = "".to_string();
        let t3 = "".to_string();
        assert_eq!(Solution::is_anagram(s3, t3), true);

        // // Test case 4
        let s4 = "listen".to_string();
        let t4 = "silent".to_string();
        assert_eq!(Solution::is_anagram(s4, t4), true);

        // // Test case 5
        let s5 = "hello".to_string();
        let t5 = "holla".to_string();
        assert_eq!(Solution::is_anagram(s5, t5), false);

        let s6 = "ab".to_string();
        let t6 = "a".to_string();
        assert_eq!(Solution::is_anagram(s6, t6), false);

        let s7 = "a".to_string();
        let t7 = "ab".to_string();
        assert_eq!(Solution::is_anagram(s7, t7), false);
    }
}
