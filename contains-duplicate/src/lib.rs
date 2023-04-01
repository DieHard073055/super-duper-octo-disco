struct Solution {}
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut _nums = nums.clone();
        _nums.sort_unstable();
        _nums.dedup();
        _nums.len() != nums.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate_true() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(
            Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
            true
        );
        assert_eq!(Solution::contains_duplicate(vec![0, 4, 5, 0, -2, 3]), true);
    }

    #[test]
    fn test_contains_duplicate_false() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(Solution::contains_duplicate(vec![3, 6, 7, 10, 45]), false);
    }
}
