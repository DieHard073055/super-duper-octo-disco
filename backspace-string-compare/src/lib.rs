use std::collections::VecDeque;

struct Solution {}
impl Solution {
    fn eval_string(input_string: String) -> VecDeque<char> {
        let mut stack = VecDeque::new();
        input_string.chars().for_each(|letter| {
            if letter == '#' {
                stack.pop_front();
            } else {
                stack.push_front(letter);
            }
        });
        stack
    }
    pub fn backspace_compare(s: String, t: String) -> bool {
        let s_stack = Solution::eval_string(s);
        let t_stack = Solution::eval_string(t);

        s_stack == t_stack
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_backspace_compare() {
        assert_eq!(
            Solution::backspace_compare(String::from("ab#c"), String::from("ad#c")),
            true
        );
        assert_eq!(
            Solution::backspace_compare(String::from("ab##"), String::from("c#d#")),
            true
        );
        assert_eq!(
            Solution::backspace_compare(String::from("a#c"), String::from("b")),
            false
        );
        assert_eq!(
            Solution::backspace_compare(String::from("xy#z"), String::from("xzz#")),
            true
        );
        assert_eq!(
            Solution::backspace_compare(String::from("###a"), String::from("a")),
            true
        );
        assert_eq!(
            Solution::backspace_compare(String::from("###"), String::from("")),
            true
        );
    }
}
