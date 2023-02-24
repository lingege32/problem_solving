struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn minimum_deviation(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();

        // use all possible *2 operations
        let mut min = i32::MAX;
        for i in 0..n {
            if nums[i] % 2 == 1 {
                nums[i] = nums[i] * 2;
            }
            min = i32::min(min, nums[i]);
        }

        // at this point we can do only /2 operations
        let mut max = i32::MIN;
        for i in 0..n {
            while nums[i] % 2 == 0 && nums[i] / 2 >= min {
                nums[i] = nums[i] / 2;
            }
            max = i32::max(max, nums[i]);
        }

        let mut best = max - min;
        // early return in case we can't optimize the solution
        if max % 2 == 1 {
            return best;
        }

        // lowest to highest with the propriety that highest/2 < lowest
        nums.sort();

        // otimize solution one whole loop or until we find an odd number
        for i in (0..n).rev() {
            if nums[i] % 2 == 1 {
                break;
            }
            nums[i] = nums[i] / 2;

            min = nums[i];
            max = if i == 0 { nums[n - 1] } else { nums[i - 1] };
            best = i32::min(best, max - min);
        }

        best
    }
}
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 4];
        let ans = 1;
        assert_eq!(ans, Solution::minimum_deviation(nums));
    }

    #[test]
    fn test_2() {
        let nums = vec![4, 1, 5, 20, 3];
        let ans = 3;
        assert_eq!(ans, Solution::minimum_deviation(nums));
    }

    #[test]
    fn test_3() {
        let nums = vec![2, 10, 8];
        let ans = 3;
        assert_eq!(ans, Solution::minimum_deviation(nums));
    }

    #[test]
    fn test_4() {
        let nums = vec![10, 4, 3];
        let ans = 2;
        assert_eq!(ans, Solution::minimum_deviation(nums));
    }
}
