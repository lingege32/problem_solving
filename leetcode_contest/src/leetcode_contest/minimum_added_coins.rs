struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn minimum_added_coins(coins: Vec<i32>, target: i32) -> i32 {
        let ori = coins.len();
        let mut coins = coins;
        coins.sort_unstable();
        for t in 1..=target {
            Self::fill_coin(&mut coins, t);
        }
        (coins.len() - ori) as i32
    }

    fn fill_coin(coins: &mut Vec<i32>, target: i32) {
        if !Self::has_solution(coins, target) {
            let idx = Self::upper_bound(coins, target);
            coins.insert(idx, target);
        }
    }

    fn has_solution(coins: &[i32], t: i32) -> bool {
        if t == 0 {
            return true;
        }
        let idx = Self::upper_bound(coins, t);
        if idx == 0 {
            return false;
        }
        let v = coins[idx - 1];
        return Self::has_solution(&coins[0..idx - 1], t - v);
    }

    fn upper_bound(coins: &[i32], target: i32) -> usize {
        let (mut left, mut right) = (0usize, coins.len());
        while left < right {
            let mid = left + (right - left) / 2;
            if coins[mid] <= target {
                left = mid + 1;
            } else {
                right = mid;
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
        let coins = vec![1, 4, 10, 5, 7, 19];
        let target = 19;
        let ans = 1;
        assert_eq!(ans, Solution::minimum_added_coins(coins, target));
    }
    #[test]
    fn test_2() {
        let coins = vec![1, 1, 1];
        let target = 20;
        let ans = 3;
        assert_eq!(ans, Solution::minimum_added_coins(coins, target));
    }
}
