// Other's solution achieve 100%
// impl Solution {
//     pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
//         let mut count = 0;

//         for row in 0..grid.len() {
//             for column in 0..grid[row].len() {
//                 if grid[row][column] == '1' {
//                     count += 1;
//                     dfs(row, column, &mut grid);
//                 }
//             }
//         }

//         fn dfs(row: usize, column: usize, grid: &mut Vec<Vec<char>>) {
//             if grid[row][column] != '1' {
//                 return;
//             }

//             grid[row][column] = '0';

//             if column < grid[row].len() - 1 {
//                 dfs(row, column + 1, grid);
//             }

//             if column > 0 {
//                 dfs(row, column - 1, grid);
//             }

//             if row < grid.len() - 1 {
//                 dfs(row + 1, column, grid);
//             }

//             if row > 0 {
//                 dfs(row - 1, column, grid);
//             }
//         }

//         count as i32
//     }
// }



struct Solution {}
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.len() == 0 || grid[0].len() == 0 {
            return 0;
        }

        let row_num = grid.len();
        let col_num = grid[0].len();
        let mut land_id = 0;
        let mut root_island = vec![vec! {None; col_num}; row_num];
        for row_idx in 0..row_num {
            for col_idx in 0..col_num {
                if Self::is_land(grid[row_idx][col_idx]) && root_island[row_idx][col_idx].is_none()
                {
                    Self::visit(
                        row_idx,
                        row_num,
                        col_idx,
                        col_num,
                        land_id,
                        &mut root_island,
                        &grid,
                    );
                    land_id += 1;
                }
            }
        }
        land_id
    }
    fn visit(
        row_idx: usize,
        row_limit: usize,
        col_idx: usize,
        col_limit: usize,
        land_id: i32,
        root_island: &mut Vec<Vec<Option<i32>>>,
        grid: &Vec<Vec<char>>,
    ) {
        let mut stack = Vec::new();
        stack.push((row_idx, col_idx));
        while !stack.is_empty() {
            let (row, col) = stack.pop().unwrap();
            if root_island[row][col].is_some() {
                continue;
            }
            root_island[row][col] = Some(land_id);
            if Self::is_valid_index(row + 1, row_limit, col, col_limit)
                && Self::is_land(grid[row + 1][col])
            {
                stack.push((row + 1, col));
            }
            if row > 0
                && Self::is_valid_index(row - 1, row_limit, col, col_limit)
                && Self::is_land(grid[row - 1][col])
            {
                stack.push((row - 1, col));
            }
            if Self::is_valid_index(row, row_limit, col + 1, col_limit)
                && Self::is_land(grid[row][col + 1])
            {
                stack.push((row, col + 1));
            }
            if col > 0
                && Self::is_valid_index(row, row_limit, col - 1, col_limit)
                && Self::is_land(grid[row][col - 1])
            {
                stack.push((row, col - 1));
            }
        }
    }
    fn is_land(c: char) -> bool {
        match c {
            '1' => true,
            _ => false,
        }
    }
    fn is_valid_index(row_idx: usize, row_limit: usize, col_idx: usize, col_limit: usize) -> bool {
        row_idx < row_limit && col_idx < col_limit
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let grid = [
            ['1', '1', '1', '1', '0'],
            ['1', '1', '0', '1', '0'],
            ['1', '1', '0', '0', '0'],
            ['0', '0', '0', '0', '0'],
        ]
        .iter()
        .map(|x| x.to_vec())
        .collect();
        assert_eq!(1, Solution::num_islands(grid));
    }

    #[test]
    fn test_2() {
        let grid = [
            ['1', '1', '0', '0', '0'],
            ['1', '1', '0', '0', '0'],
            ['0', '0', '1', '0', '0'],
            ['0', '0', '0', '1', '1'],
        ]
        .iter()
        .map(|x| x.to_vec())
        .collect();
        assert_eq!(3, Solution::num_islands(grid));
    }
}
