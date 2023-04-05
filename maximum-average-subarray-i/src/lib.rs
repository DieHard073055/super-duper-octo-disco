struct Solution {}
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut window_sum = 0f64;
        let mut window_avg = 0f64;
        let mut window_max_avg = 0f64;
        let mut start = 0;
        let mut end = (k - 1) as usize;

        if nums.len() == 1 {
            if k > 0 {
                return nums[0] as f64;
            }
        }
        // println!("nums: {:?}", nums);
        // println!("k: {:}", k);

        for i in start..(end + 1) {
            // println!("num: {:}", nums[i]);
            window_sum += nums[i] as f64;
            // println!("window sum: {:}", window_sum);
        }
        window_max_avg = window_sum / k as f64;

        // println!("window sum: {:}", window_sum);
        // println!("window max: {:}", window_max_avg);
        while end < nums.len() - 1 {
            end += 1;
            // println!("start: {:}", start);
            // println!("end: {:}", end);
            let next_num = nums[end] as f64;
            let first_num = nums[start] as f64;
            start += 1;
            // println!("next num: {:}", next_num);
            window_sum += next_num;
            // println!("window sum: {:}", window_sum);
            // println!("first num: {:}", first_num);
            window_sum -= first_num;
            // println!("window sum: {:}", window_sum);

            window_avg = window_sum / k as f64;
            // println!("window avg: {:}", window_avg);
            if window_avg > window_max_avg {
                window_max_avg = window_avg;
            }
            // println!("window max: {:}", window_max_avg);
        }
        // println!("window max: {:}", window_max_avg);
        window_max_avg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 12, -5, -6, 50, 3];
        let k = 4;
        assert!((Solution::find_max_average(nums, k) - 12.75).abs() < 1e-5);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![5];
        let k = 1;
        assert!((Solution::find_max_average(nums, k) - 5.0).abs() < 1e-5);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![3];
        let k = 1;
        assert!((Solution::find_max_average(nums, k) - 3.0).abs() < 1e-5);
    }

    #[test]
    fn test_all_positive() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7];
        let k = 3;
        assert!((Solution::find_max_average(nums, k) - 6.0).abs() < 1e-5);
    }

    #[test]
    fn test_all_negative() {
        let nums = vec![-5, -4, -3, -2, -1];
        let k = 2;
        assert!((Solution::find_max_average(nums, k) - (-1.5)).abs() < 1e-5);
    }
}
