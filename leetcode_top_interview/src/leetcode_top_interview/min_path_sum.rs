struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let row_len = grid.len();
        let column_len = grid[0].len();
        let first_row = &mut grid[0];
        for idx in 1..column_len {
            first_row[idx] += first_row[idx - 1];
        }

        for idx in 1..row_len {
            grid[idx][0] += grid[idx-1][0];
        }
        for row_id in 1..grid.len() {
            for column_id in 1..grid[0].len() {
                let min = grid[row_id - 1][column_id].min(grid[row_id][column_id - 1]);
                grid[row_id][column_id] += min;
            }
        }
        *grid.last().unwrap().last().unwrap()
    }
}
