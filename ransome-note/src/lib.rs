struct Solution {}
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }
        let mut letter_map: [usize; 26] = [0; 26];
        magazine
            .chars()
            .for_each(|letter| letter_map[(letter as u8 - b'a') as usize] += 1);
        for letter in ransom_note
            .chars()
            .map(|letter| ((letter as u8 - b'a') as usize))
        {
            if letter_map[letter] == 0 {
                return false;
            }
            letter_map[letter] -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_construct() {
        assert_eq!(
            true,
            Solution::can_construct("a".to_string(), "ab".to_string())
        );
        assert_eq!(
            false,
            Solution::can_construct("aa".to_string(), "ab".to_string())
        );
        assert_eq!(
            true,
            Solution::can_construct("aab".to_string(), "baa".to_string())
        );
    }
}
