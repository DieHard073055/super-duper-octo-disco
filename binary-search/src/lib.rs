struct Solution {}
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: usize = 0;
        let mut right = nums.len() - 1;
        match nums.len() {
            1 => {
                if nums[0] == target {
                    return 0i32;
                } else {
                    return -1i32;
                }
            }
            2 | 3 => {
                if nums[0] == target {
                    return 0i32;
                } else if nums[1] == target {
                    return 1i32;
                } else if nums.len() > 2 && nums[2] == target {
                    return 2i32;
                } else {
                    return -1i32;
                }
            }
            _ => {}
        }
        //println!("nums = {:?}", nums);
        if nums[left as usize] == target {
            return left as i32;
        }
        if nums[right as usize] == target {
            return right as i32;
        }

        while right - 1 > left {
            let middle = left + ((right - left) / 2);
            // println!(
            //     "l {:} r {:} m {:} = {:} | t {:}",
            //     left, right, middle, nums[middle as usize], target
            // );
            if nums[middle] == target {
                return middle as i32;
            }
            if nums[middle] < target {
                left = middle;
            } else if nums[middle] > target {
                right = middle;
            }
        }
        -1i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_exists() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(Solution::search(vec![-5, -3, 0, 1, 5, 8, 10], 1), 3);
        assert_eq!(Solution::search(vec![1, 2, 3, 4, 5], 5), 4);
    }

    #[test]
    fn test_search_not_exists() {
        assert_eq!(Solution::search(vec![-5], 5), -1);
        assert_eq!(Solution::search(vec![-5, -4, -3], 5), -1);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
        assert_eq!(Solution::search(vec![1, 3, 4, 6, 7, 8], 2), -1);
        assert_eq!(Solution::search(vec![1, 2, 3, 4, 5], 6), -1);
    }
}
