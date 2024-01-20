struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn total_n_queens(n: i32) -> i32 {
        Self::dfs(n, 0, 0, 0)
    }
    fn dfs(n: i32, r: i32, col_restrict: u64, diagonal_restric: u64) -> i32 {
        if r == n {
            return 1;
        }
        let mut ans = 0;
        for next_col in 0..n {
            let (rl, lr) = (r + next_col, 3 * n + r - next_col);
            if col_restrict >> next_col & 0x1 == 0 &&
                diagonal_restric >> rl & 0x1 == 0 &&
                diagonal_restric >> lr & 0x1 == 0
            {
                ans += Self::dfs(
                    n,
                    r + 1,
                    col_restrict | 1 << next_col,
                    diagonal_restric | 1 << rl | 1 << lr,
                );
            }
        }
        ans
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let n = 9;
        let ans = 352;
        assert_eq!(ans, Solution::total_n_queens(n));
    }
}
