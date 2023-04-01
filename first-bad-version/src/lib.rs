struct Solution {
    bad_version: i32,
}
impl Solution {
    fn check_bad_version(&self, v: i32) -> (bool, bool) {
        (self.isBadVersion(v - 1), self.isBadVersion(v))
    }
    /*
    0 0 0 0 0 0 | 1 1 1 1 1 1
    */
    fn check_segment(&self, start: i32, end: i32) -> Option<i32> {
        let fbv = (false, true);

        let next_middle = start + ((end - start) / 2);
        let first_bools = self.check_bad_version(next_middle);
        if fbv == first_bools {
            return Some(next_middle);
        }
        let second_bools = self.check_bad_version(next_middle + 1);
        if fbv == second_bools {
            return Some(next_middle + 1);
        }
        if second_bools == (false, false) {
            return self.check_segment(next_middle, end);
        } else {
            return self.check_segment(start, next_middle);
        }
    }
    pub fn first_bad_version(&self, n: i32) -> i32 {
        if n < 10 {
            for i in 0..n {
                if self.isBadVersion(i) {
                    return i;
                }
            }
        } else {
            // check end and begining
            let fbv = (false, true);
            if fbv == self.check_bad_version(1) {
                return 1;
            }
            if fbv == self.check_bad_version(n) {
                return n;
            }
            return self.check_segment(0, n).unwrap();
        }
        n
    }

    fn isBadVersion(&self, version: i32) -> bool {
        // implementation of isBadVersion function goes here
        version >= self.bad_version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_bad_version() {
        let solution = Solution { bad_version: 5 };
        assert_eq!(solution.first_bad_version(5), 5);
    }
    #[test]
    fn test_first_bad_version_2() {
        let solution = Solution { bad_version: 6 };
        assert_eq!(solution.first_bad_version(12), 6);
    }
    #[test]
    fn test_first_bad_version_3() {
        let solution = Solution { bad_version: 60 };
        assert_eq!(solution.first_bad_version(120), 60);
    }
}
