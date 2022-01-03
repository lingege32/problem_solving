
struct Solution {
    
}
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        if n < 2 {
            1
        } else {
            let mut dp = vec![0; (n+1) as usize];
            dp[0] = 1;
            dp[1] = 1;
            Self::num_trees_dp(n, &mut dp)
        }
    }
    fn num_trees_dp(n: i32, dp: &mut[i32]) -> i32{

        if dp[n as usize] == 0 {
            dp[n as usize] = (1..=n).map(|x| {
                Self::num_trees_dp(x-1, dp) * Self::num_trees_dp(n-x, dp)
            }).sum();
        }
        dp[n as usize]
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::num_trees(1));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::num_trees(2));
    }

    #[test]
    fn test_3() {
        assert_eq!(5, Solution::num_trees(3));
    }

}


