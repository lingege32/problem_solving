struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let row_len = grid.len();
        let col_len = grid[0].len();
        let mut new_grid = vec![vec![0; col_len + 1]; row_len + 1];
        let mut ans = 0;
        for r in 1..=row_len {
            for c in 1..=col_len {
                let total = new_grid[r - 1][c] + new_grid[r][c - 1] - new_grid[r - 1][c - 1] +
                    grid[r - 1][c - 1];
                if total <= k {
                    ans += 1;
                }
                new_grid[r][c] = total;
            }
        }
        ans
    }
}
