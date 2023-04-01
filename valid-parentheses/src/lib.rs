struct Solution {}
impl Solution {
    fn opposite_bracket(c: char) -> char {
        match c {
            '(' => ')',
            '{' => '}',
            '[' => ']',
            _ => ' ',
        }
    }
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' | ']' | '}' => {
                    let last_item = stack.pop().unwrap_or(' ');
                    if c != Solution::opposite_bracket(last_item) {
                        return false;
                    }
                }
                _ => (),
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("(]".to_string()), false);
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
        assert_eq!(Solution::is_valid("{[]}".to_string()), true);
        assert_eq!(Solution::is_valid("".to_string()), true);
        assert_eq!(Solution::is_valid("[".to_string()), false);
        assert_eq!(Solution::is_valid("]".to_string()), false);
        assert_eq!(Solution::is_valid("(())".to_string()), true);
        assert_eq!(Solution::is_valid("(()())".to_string()), true);
        assert_eq!(Solution::is_valid("((())".to_string()), false);
        assert_eq!(Solution::is_valid("())(".to_string()), false);
    }
}
