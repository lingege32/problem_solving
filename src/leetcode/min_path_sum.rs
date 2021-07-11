struct Solution {
    
}
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            0
        } else {
            let mut dp = vec![vec![i32::MAX; grid[0].len()]; grid.len()];
            dp[0][0] = grid[0][0];
            for row in 0..grid.len() {
                for col in 0..grid[row].len() {
                    if row > 0 {
                        dp[row][col] = dp[row][col].min(dp[row-1][col]+grid[row][col]);
                    }
                    if col > 0 {
                        dp[row][col] = dp[row][col].min(dp[row][col-1]+grid[row][col]);
                    }
                }
            }
            *dp.last().unwrap().last().unwrap()
        } 
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let grid = [[1,3,1],[1,5,1],[4,2,1]].iter().map(|x| x.to_vec()).collect();
        assert_eq!(7, Solution::min_path_sum(grid));
    }
    #[test]
    fn test_2() {
        let grid =  [[1,2,3],[4,5,6]].iter().map(|x| x.to_vec()).collect();
        assert_eq!(12, Solution::min_path_sum(grid));
    }
}