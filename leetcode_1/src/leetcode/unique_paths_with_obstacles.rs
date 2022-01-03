struct Solution {}
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid.is_empty()
            || obstacle_grid[0].is_empty()
            || obstacle_grid[0][0] == 1
            || *obstacle_grid.last().unwrap().last().unwrap() == 1
        {
            0
        } else {
            let row_num = obstacle_grid.len();
            let col_num = obstacle_grid[0].len();
            let mut dp = vec![vec![0; col_num]; row_num];

            dp[0][0] = 1;
            for idx in 1..col_num {
                if obstacle_grid[0][idx] == 0 {
                    dp[0][idx] = 1;
                } else {
                    break;
                }
            }
            for idx in 1..row_num {
                if obstacle_grid[idx][0] == 0 {
                    dp[idx][0] = 1;
                } else {
                    break;
                }
            }

            for row_idx in 1..row_num {
                for col_idx in 1..col_num {
                    if obstacle_grid[row_idx][col_idx] == 0 {
                        dp[row_idx][col_idx] = dp[row_idx - 1][col_idx] + dp[row_idx][col_idx - 1];
                    }
                }
            }
            dp[row_num - 1][col_num - 1]
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let obstacleGrid = [[0, 0, 0], [0, 1, 0], [0, 0, 0]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        assert_eq!(2, Solution::unique_paths_with_obstacles(obstacleGrid));
    }

    #[test]
    fn test_2() {
        let obstacleGrid = [[0, 1], [0, 0]].iter().map(|x| x.to_vec()).collect();
        assert_eq!(1, Solution::unique_paths_with_obstacles(obstacleGrid));
    }
}
