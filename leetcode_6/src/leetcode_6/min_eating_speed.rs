struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let right = *piles.iter().max().unwrap_or(&0);
        Self::helper(&piles, h, right+1)
    }
    fn helper(piles: &[i32], h: i32, mut right: i32) -> i32 {
        let mut left = 1;
        while left < right {
            let mid = left + (right - left) / 2;
            let mut total = 0;
            for &p in piles {
                total += (p+mid-1)/mid;
                if total > h {
                    break;
                }
            }
            if total > h {
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
        let piles=vec![312884470];
        let h=968709470;
        let ans = 4;
        assert_eq!(ans, Solution::min_eating_speed(piles, h));
    }
}