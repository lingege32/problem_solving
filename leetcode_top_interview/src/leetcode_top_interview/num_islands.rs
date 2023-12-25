struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let row_len = grid.len();
        let col_len = grid[0].len();
        let mut island_id = 0;
        for row_id in 0..row_len {
            for col_id in 0..col_len {
                if grid[row_id][col_id] == '1' {
                    Self::eliminate(&mut grid, row_id, col_id, row_len, col_len);
                    island_id += 1;
                }
            }
        }
        island_id
    }
    fn eliminate(
        grid: &mut Vec<Vec<char>>,
        row_id: usize,
        col_id: usize,
        row_len: usize,
        col_len: usize,
    ) {
        let mut stack = vec![(row_id, col_id)];
        while let Some((row, col)) = stack.pop() {
            let val = &mut grid[row][col];
            if *val == '1' {
                *val = '0';
                if row != 0 {
                    stack.push((row - 1, col));
                }
                if col != 0 {
                    stack.push((row, col - 1));
                }
                if row + 1 != row_len {
                    stack.push((row + 1, col));
                }
                if col + 1 != col_len {
                    stack.push((row, col + 1));
                }
            }
        }
    }
}
