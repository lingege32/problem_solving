struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;
        let mut ans = 0;
        let x = grid.len();
        let y = grid[0].len();
        for i in 0..x {
            for j in 0..y {
                if grid[i][j] == '1' {
                    Self::dfs(&mut grid, i, j);
                    ans += 1;
                }
            }
        }
        ans
    }
    fn dfs(grid: &mut Vec<Vec<char>>, x: usize, y: usize) {
        if grid[x][y] == '1' {
            grid[x][y] = '0';
            if x > 0 {
                Self::dfs(grid, x - 1, y);
            }
            if y > 0 {
                Self::dfs(grid, x, y - 1);
            }
            if x + 1 != grid.len() {
                Self::dfs(grid, x + 1, y);
            }
            if y + 1 != grid[0].len() {
                Self::dfs(grid, x, y + 1);
            }
        }
    }
}
