struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn unique_paths_with_obstacles(mut obstacle_grid: Vec<Vec<i32>>) -> i32 {
        obstacle_grid.iter_mut().for_each(|v| {
            v.iter_mut().for_each(|x| {
                if *x == 0 {
                    *x = 1;
                } else {
                    *x = 0;
                }
            })
        });
        let row_num = obstacle_grid.len();
        let col_num = obstacle_grid[0].len();
        let first_row = &mut obstacle_grid[0];
        for idx in 1..col_num {
            if first_row[idx] == 1 {
                first_row[idx] = first_row[idx - 1];
            }
        }
        for idx in 1..row_num {
            if obstacle_grid[idx][0] == 1 {
                obstacle_grid[idx][0] = obstacle_grid[idx - 1][0];
            }
        }
        for row_id in 1..row_num {
            for col_id in 1..col_num {
                if obstacle_grid[row_id][col_id] == 1 {
                    let up = obstacle_grid[row_id - 1][col_id];
                    let left = obstacle_grid[row_id][col_id - 1];
                    obstacle_grid[row_id][col_id] = up + left;
                }
            }
        }

        *obstacle_grid.last().unwrap().last().unwrap()
    }
}
