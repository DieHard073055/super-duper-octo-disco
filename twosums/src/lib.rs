use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut old_nums: HashMap<i32, i32> = HashMap::new();
        for (idx, num) in nums.iter().enumerate() {
            let index: i32 = i32::try_from(idx).ok().unwrap();
            let num_2 = target - num;
            if old_nums.contains_key(&num_2) {
                let index_0: i32 = old_nums.get(&num_2).unwrap().clone();
                return vec![index_0, index];
            }
            old_nums.insert(num.clone(), index);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 26;
        let expected = vec![2, 3];
        assert_eq!(Solution::two_sum(nums, target), expected);
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![0, 1];
        assert_eq!(Solution::two_sum(nums, target), expected);
        let nums = vec![2, 7, 11, 15];
        let target = 13;
        let expected = vec![0, 2];
        assert_eq!(Solution::two_sum(nums, target), expected);
        let nums = vec![2, 7, 11, 15];
        let target = 18;
        let expected = vec![1, 2];
        assert_eq!(Solution::two_sum(nums, target), expected);
    }

    #[test]
    #[ignore]
    fn test_two_sum_with_negative_numbers() {
        let nums = vec![-2, -7, 11, 15];
        let target = 9;
        let expected = vec![0, 3];
        assert_eq!(Solution::two_sum(nums, target), expected);
    }

    #[test]
    #[ignore]
    fn test_two_sum_with_duplicate_numbers() {
        let nums = vec![3, 2, 4, 3];
        let target = 6;
        let expected = vec![0, 3];
        assert_eq!(Solution::two_sum(nums, target), expected);
    }

    #[test]
    fn test_two_sum_with_zero_target() {
        let nums = vec![3, 2, 4];
        let target = 0;
        let expected = vec![];
        assert_eq!(Solution::two_sum(nums, target), expected);
    }
}
