struct Solution {}
impl Solution {
    fn eval_string(input_string: &str) -> (u32, u32) {
        let mut evaluated_num = 0;
        let mut backspace = 0;
        let mut idx: u32 = 1;
        let mut count_letters = 0;
        input_string.chars().rev().for_each(|letter| match letter {
            '#' => backspace += 1,
            _ if backspace > 0 => backspace -= 1,
            _ => {
                let letter_num = letter as u32 - 97;
                let current_num = letter_num * 10u32.pow(idx);
                evaluated_num += current_num;
                idx += 1;
                if idx >= 8 {
                    idx = 1;
                }
                count_letters += 1;
            }
        });
        (count_letters, evaluated_num)
    }
    pub fn backspace_compare(s: String, t: String) -> bool {
        let s_stack = Solution::eval_string(s.as_str());
        let t_stack = Solution::eval_string(t.as_str());

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
        assert_eq!(
            Solution::backspace_compare(String::from("abc#"), String::from("bca#")),
            false
        );
        assert_eq!(
            Solution::backspace_compare(String::from("abc#"), String::from("bac#")),
            false
        );
        assert_eq!(
            Solution::backspace_compare(String::from("ba"), String::from("ac")),
            false
        );
        assert_eq!(
            Solution::backspace_compare(String::from("aaa###a"), String::from("aaaa###a")),
            false
        );
        let s =
        "tdkag#neumi#wkds####i#qoqk####mrrlcnsjeiq##########qm#sxl##qrupdvndi######telzybxummoodfslfrnuaqhixabuheg#####################ayvlbyvjmjpekofgojkjvse#";
        let t =
        "tdkag#neumi#wkds####i#qoqk####mrrlcnsjeiq##########qm#sxl##qrupdvndi######telz#ybxummoodfslfrnuaqhixabuheg#####################ayvlbyvjmjpekofgojkjvse#";
        assert_eq!(
            Solution::backspace_compare(s.to_string(), t.to_string()),
            false
        );
    }
}
