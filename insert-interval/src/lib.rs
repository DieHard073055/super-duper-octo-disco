struct Solution {}
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut output_intervals = vec![];
        let mut inserted = false;
        let mut merged = false;
        if intervals.len() < 1 {
            inserted = true;
            output_intervals.push(new_interval);
            return output_intervals;
        }
        let b_start = new_interval[0];
        let b_end = new_interval[1];
        for i in 0..intervals.len() {
            let a_start = intervals[i][0];
            let a_end = intervals[i][1];
            if b_start < a_start && b_end >= a_end && !merged && !inserted {
                inserted = true;
                merged = true;
                output_intervals.push(vec![b_start, b_end]);
            } else if a_start > b_start && a_end > b_end && a_start <= b_end && !inserted && !merged
            {
                inserted = true;
                merged = true;
                output_intervals.push(vec![b_start, a_end]);
            } else if a_start > b_end && !inserted && !merged {
                inserted = true;
                merged = true;
                output_intervals.push(vec![b_start, b_end]);
                output_intervals.push(vec![a_start, a_end]);
            } else if a_start <= b_start && a_end >= b_start && !inserted {
                // intersection
                if a_end >= b_end {
                    output_intervals.push(vec![a_start, a_end]);
                } else {
                    output_intervals.push(vec![a_start, b_end]);
                }
                inserted = true;
            } else if a_start > b_start && a_end < b_end {
                // ignore
            } else if a_start > b_start && a_start < b_end && a_end > b_end && inserted && !merged {
                // change last interval
                let last_interval = output_intervals.pop().unwrap();
                output_intervals.push(vec![last_interval[0], a_end]);
                merged = true;
            } else if a_start == b_end && a_end > b_end && inserted && !merged {
                // change last interval
                let last_interval = output_intervals.pop().unwrap();
                output_intervals.push(vec![last_interval[0], a_end]);
                merged = true;
            } else {
                output_intervals.push(vec![a_start, a_end]);
            }
        }
        if !inserted {
            output_intervals.push(new_interval);
        }
        output_intervals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        let expected = vec![vec![1, 5], vec![6, 9]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }

    #[test]
    fn test_example_2() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ];
        let new_interval = vec![4, 8];
        let expected = vec![vec![1, 2], vec![3, 10], vec![12, 16]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }

    #[test]
    fn test_empty_intervals() {
        let intervals = vec![];
        let new_interval = vec![5, 7];
        let expected = vec![vec![5, 7]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }

    #[test]
    fn test_new_interval_before_intervals() {
        let intervals = vec![vec![3, 5], vec![8, 10]];
        let new_interval = vec![1, 2];
        let expected = vec![vec![1, 2], vec![3, 5], vec![8, 10]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }

    #[test]
    fn test_new_interval_after_intervals() {
        let intervals = vec![vec![1, 3], vec![5, 7]];
        let new_interval = vec![9, 11];
        let expected = vec![vec![1, 3], vec![5, 7], vec![9, 11]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }

    #[test]
    fn test_new_interval_overlap_multiple_intervals() {
        let intervals = vec![vec![1, 4], vec![6, 9], vec![11, 15]];
        let new_interval = vec![3, 12];
        let expected = vec![vec![1, 15]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }
    /*
        intervals =
        [[1,5]]
        newInterval =
        [0,3]
    */
    #[test]
    fn test_single_interval_inside_new_interval() {
        let intervals = vec![vec![1, 5]];
        let new_interval = vec![2, 3];
        let expected = vec![vec![1, 5]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }
    #[test]
    fn test_single_interval_merge_at_the_start_of_new_interval() {
        let intervals = vec![vec![1, 5]];
        let new_interval = vec![0, 3];
        let expected = vec![vec![0, 5]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }
    #[test]
    fn test_single_interval_merge_at_the_start_of_new_interval_2() {
        let intervals = vec![vec![1, 5]];
        let new_interval = vec![0, 1];
        let expected = vec![vec![0, 5]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }
    #[test]
    fn test_b_swallows_a() {
        let intervals = vec![vec![1, 5]];
        let new_interval = vec![0, 5];
        let expected = vec![vec![0, 5]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }
}
