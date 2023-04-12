struct Solution {}

impl Solution {
    fn roman_char_to_int(roman: &char) -> i32 {
        match roman {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => !unreachable!("There should'nt be any more roman characters"),
        }
    }
    fn eval_roman_chars(next: i32, previous: i32) -> (bool, i32) {
        // println!("eval roman char: next {:}, previous {:}", next, previous);
        if next > previous {
            // println!("({:}, {:})", true, next - previous);
            return (true, next - previous);
        } else {
            // println!("({:}, {:})", false, previous);
            return (false, previous);
        }
    }
    pub fn roman_to_int(s: String) -> i32 {
        let mut total = 0;
        let s_letters = s.chars().collect::<Vec<char>>();
        if s.len() < 2 {
            return Solution::roman_char_to_int(&s_letters[0]);
        }
        let mut previous_idx = 0;
        let mut previous = Solution::roman_char_to_int(&s_letters[previous_idx]);
        let mut next_idx = 1;
        // println!("s_letters: {:} - len: {:}", s, s_letters.len());
        while next_idx < s_letters.len() {
            previous = Solution::roman_char_to_int(&s_letters[previous_idx]);
            let next = Solution::roman_char_to_int(&s_letters[next_idx]);
            // println!(
            //    "while start: next idx {:}, previous idx {:}",
            //    next_idx, previous_idx
            //);
            // println!("while start: next {:}, previous {:}", next, previous);
            let (skip, value) = Solution::eval_roman_chars(next, previous);
            if skip {
                previous_idx += 1;
                next_idx += 1;
                previous = 0;
            }
            previous_idx += 1;
            next_idx += 1;
            total += value;
            // println!(
            //    "while end: next idx {:}, previous idx {:}",
            //    next_idx, previous_idx
            //);
        }
        if previous_idx < s_letters.len() {
            previous = Solution::roman_char_to_int(&s_letters[previous_idx]);
            total += previous;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
        assert_eq!(Solution::roman_to_int(String::from("IV")), 4);
        assert_eq!(Solution::roman_to_int(String::from("IX")), 9);
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
        assert_eq!(Solution::roman_to_int(String::from("MMXXI")), 2021);
        assert_eq!(Solution::roman_to_int(String::from("D")), 500);
        assert_eq!(Solution::roman_to_int(String::from("XL")), 40);
        assert_eq!(Solution::roman_to_int(String::from("XC")), 90);
        assert_eq!(Solution::roman_to_int(String::from("CD")), 400);
        assert_eq!(Solution::roman_to_int(String::from("CM")), 900);
    }
}
