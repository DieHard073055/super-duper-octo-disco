use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut end = 0;
        let mut window_contents = HashMap::new();
        let mut total_fruit: i32 = 0;

        if fruits.len() == 1 {
            return 1;
        }

        *window_contents.entry(fruits[end]).or_insert(0) += 1;

        while end < fruits.len() - 1 {
            // println!("start: {:}, end: {:}", start, end);
            // println!("window: {:?}", window_contents);
            end += 1;
            let next_fruit = fruits[end];
            // println!("next fruit: {:}", next_fruit);
            *window_contents.entry(next_fruit).or_insert(0) += 1;
            if window_contents.keys().len() <= 2 {
                let total = (end - start) as i32 + 1;
                // println!("end - start : {:} - {:}", end, start);
                // println!("total : {:}", total);
                if total > total_fruit {
                    total_fruit = total;
                }
                // println!("total fruit : {:}", total_fruit);
            }
            if window_contents.keys().len() > 2 {
                let previous_fruit = fruits[start];
                // println!("next fruit: {:}", previous_fruit);
                *window_contents.get_mut(&previous_fruit).unwrap() -= 1;
                if window_contents.get(&previous_fruit).unwrap() < &1 {
                    window_contents.remove(&previous_fruit);
                }
                start += 1;
            }
        }

        total_fruit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let fruits = vec![1, 2, 1];
        let result = Solution::total_fruit(fruits);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_example_2() {
        let fruits = vec![0, 1, 2, 2];
        let result = Solution::total_fruit(fruits);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_example_3() {
        let fruits = vec![1, 2, 3, 2, 2];
        let result = Solution::total_fruit(fruits);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_single_tree() {
        let fruits = vec![1];
        let result = Solution::total_fruit(fruits);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_all_same_fruit() {
        let fruits = vec![2, 2, 2, 2, 2];
        let result = Solution::total_fruit(fruits);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_alternating_fruits() {
        let fruits = vec![1, 2, 1, 2, 1];
        let result = Solution::total_fruit(fruits);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_multiple_valid_ranges() {
        let fruits = vec![1, 2, 1, 3, 1, 2, 1, 4];
        let result = Solution::total_fruit(fruits);
        assert_eq!(result, 3);
    }
}
