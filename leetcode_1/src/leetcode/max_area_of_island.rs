struct Solution {}

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }
        let mut grid = grid;

        let row_num = grid.len();
        let col_num = grid[0].len();
        (0..row_num)
            .map(|row_idx| {
                (0..col_num)
                    .map(|col_idx| Self::dfs(row_idx, row_num, col_idx, col_num, &mut grid))
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
    }
    fn dfs(
        row_idx: usize,
        row_limit: usize,
        col_idx: usize,
        col_limit: usize,
        grid: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if grid[row_idx][col_idx] == 0 {
            0
        } else {
            let mut stack = vec![(row_idx, col_idx)];
            let mut area = 0;
            while !stack.is_empty() {
                let (row, col) = stack.pop().unwrap();
                if grid[row][col] == 0 {
                    continue;
                }
                area += 1;
                grid[row][col] = 0;
                if row > 0 {
                    stack.push((row - 1, col));
                }
                if col > 0 {
                    stack.push((row, col - 1));
                }
                if row + 1 < row_limit {
                    stack.push((row + 1, col));
                }
                if col + 1 < col_limit {
                    stack.push((row, col + 1))
                }
            }
            area
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let grid = [
            [0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            [0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            [0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ]
        .iter()
        .map(|x| x.to_vec())
        .collect();
        assert_eq!(6, Solution::max_area_of_island(grid));
    }

    #[test]
    fn test_2() {
        let grid = [[0, 0, 0, 0, 0, 0, 0, 0]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        assert_eq!(0, Solution::max_area_of_island(grid));
    }
}
