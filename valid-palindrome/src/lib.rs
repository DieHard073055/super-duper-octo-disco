struct Solution {}
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let filtered_string = s
            .replace(|c: char| !c.is_ascii_alphanumeric(), "")
            .to_ascii_lowercase();
        let reversed_filtered_string = filtered_string.chars().rev().collect::<String>();
        filtered_string == reversed_filtered_string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        // Test case 1
        let s1 = "A man, a plan, a canal: Panama".to_string();
        assert_eq!(Solution::is_palindrome(s1), true);

        // Test case 2
        let s2 = "race a car".to_string();
        assert_eq!(Solution::is_palindrome(s2), false);

        // // Test case 3
        let s3 = " ".to_string();
        assert_eq!(Solution::is_palindrome(s3), true);

        // // Test case 4
        let s4 = "Madam, in Eden, I'm Adam".to_string();
        assert_eq!(Solution::is_palindrome(s4), true);

        // // Test case 5
        let s5 = "Able , was I saw eLba".to_string();
        assert_eq!(Solution::is_palindrome(s5), true);

        // // Test case 6
        let s6 = "0P".to_string();
        assert_eq!(Solution::is_palindrome(s6), false);
    }
}
