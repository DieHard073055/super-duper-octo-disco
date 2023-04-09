use std::collections::VecDeque;

struct Solution {}
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut s_stack = VecDeque::new();
        let mut t_stack = VecDeque::new();

        s.chars().for_each(|letter| {
            if letter == '#' {
                s_stack.pop_front();
            } else {
                s_stack.push_front(letter);
            }
        });
        t.chars().for_each(|letter| {
            if letter == '#' {
                t_stack.pop_front();
            } else {
                t_stack.push_front(letter);
            }
        });

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
