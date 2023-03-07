struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        if time.is_empty() {
            return 0;
        }
        let min = *time.iter().min().unwrap();
        let (left, right) = (0i64, (total_trips as i64 * min as i64));
        Self::helper(&time, total_trips, left, right)
    }

    fn helper(time: &[i32], total_trips: i32, mut left: i64, mut right: i64) -> i64 {
        while left < right {
            let mid = left + (right - left) / 2;
            let mut total = 0;
            for t in time {
                total += mid / (*t as i64);
                if total >= total_trips as i64 {
                    break;
                }
            }
            if total >= total_trips as i64 {
                
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let time = vec![1, 2, 3];
        let total_trips = 5;
        let ans = 3;
        assert_eq!(ans, Solution::minimum_time(time, total_trips));
    }

    #[test]
    fn test_2() {
        let time = vec![2];
        let total_trips = 1;
        let ans = 2;
        assert_eq!(ans, Solution::minimum_time(time, total_trips));
    }

    #[test]
    fn test_3() {
        let time = vec![1; 100000];
        let total_trips = 10000000;
        let ans = 100;
        assert_eq!(ans, Solution::minimum_time(time, total_trips));
    }
}
