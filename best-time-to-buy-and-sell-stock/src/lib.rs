struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min: i32 = prices[0];
        let mut profit: i32 = 0;
        for price in prices {
            let curr_profit: i32 = price - min;
            if curr_profit > profit {
                profit = curr_profit;
            }
            if price < min {
                min = price;
            }
        }
        profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        // Test case 1
        let prices1 = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(Solution::max_profit(prices1), 5);

        // Test case 2
        let prices2 = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit(prices2), 0);

        // // Test case 3
        let prices3 = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::max_profit(prices3), 4);

        // // Test case 4
        let prices4 = vec![7, 5, 3, 1];
        assert_eq!(Solution::max_profit(prices4), 0);

        // // Test case 5
        let prices5 = vec![2, 4, 1];
        assert_eq!(Solution::max_profit(prices5), 2);
    }
}
