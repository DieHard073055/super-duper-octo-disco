struct Solution {}
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut output = vec![];
        if intervals.len() < 1 {
            output.push(new_interval);
            return output;
        }
        let [b_start, b_end] = [new_interval[0], new_interval[1]];
        let mut intersected = false;
        let mut done = false;

        for interval in intervals {
            let [a_start, a_end] = [interval[0], interval[1]];
            match (intersected, done) {
                (false, false) => {
                    // not intersected, not done
                    if b_end <= a_end {
                        intersected = true;
                        done = true;
                        // check if b is before a.
                        if b_end < a_start {
                            // set b contents first then a.
                            output.push(vec![b_start, b_end]);
                            output.push(vec![a_start, a_end]);
                        } else if b_end >= a_start {
                            if b_start >= a_start {
                                // a_start, a_end
                                output.push(vec![a_start, a_end]);
                            } else if b_start < a_start {
                                // b_start, a_end
                                output.push(vec![b_start, a_end]);
                            }
                        }
                    } else if b_start < a_end {
                        intersected = true;
                        if b_start >= a_start {
                            // a_start, b_end
                            output.push(vec![a_start, b_end]);
                        } else if b_start <= a_start {
                            // b_start, b_end
                            output.push(vec![b_start, b_end]);
                        }
                    } else if b_start == a_end {
                        intersected = true;
                        output.push(vec![a_start, b_end]);
                    } else {
                        output.push(vec![a_start, a_end]);
                    }
                }
                (true, false) => {
                    // intersected, not done
                    if a_start > b_end {
                        done = true;
                        output.push(vec![a_start, a_end]);
                    } else if a_start <= b_end {
                        if a_end < b_end {
                            // skip
                        } else if a_end >= b_end {
                            done = true;
                            let last_interval = output.pop().unwrap();
                            output.push(vec![last_interval[0], a_end]);
                        }
                    }
                }
                (false, true) => {}
                (true, true) => {
                    output.push(vec![a_start, a_end]);
                }
            }
        }
        if !done && !intersected {
            output.push(new_interval);
        }

        output
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
    fn test_b_partially_swallows_a() {
        let intervals = vec![vec![1, 5]];
        let new_interval = vec![0, 5];
        let expected = vec![vec![0, 5]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }
    #[test]
    fn test_b_swallows_a() {
        let intervals = vec![vec![1, 5]];
        let new_interval = vec![0, 6];
        let expected = vec![vec![0, 6]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }
    #[test]
    fn test_single_interval_right_before() {
        let intervals = vec![vec![1, 5]];
        let new_interval = vec![5, 7];
        let expected = vec![vec![1, 7]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }
    #[test]
    fn test_new_interval_merges_intervals() {
        let intervals = vec![vec![1, 5], vec![6, 8]];
        let new_interval = vec![5, 6];
        let expected = vec![vec![1, 8]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }
}
