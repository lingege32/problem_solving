struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut count = 0;
        let mut stack = Vec::new();
        let (row_num, col_num) = (grid.len(), grid[0].len());
        for x in 0..row_num {
            for y in 0..col_num {
                if grid[x][y] == 2 {
                    stack.push((x, y));
                }
            }
        }

        while !stack.is_empty() {
            let mut tmp = Vec::new();
            let mut hit = false;
            for &(x, y) in stack.iter() {
                if x != 0 && grid[x - 1][y] == 1 {
                    grid[x - 1][y] = 2;
                    tmp.push((x - 1, y));
                    hit = true;
                }
                if y != 0 && grid[x][y - 1] == 1 {
                    grid[x][y - 1] = 2;
                    tmp.push((x, y - 1));
                    hit = true;
                }
                if x + 1 != row_num && grid[x + 1][y] == 1 {
                    grid[x + 1][y] = 2;
                    tmp.push((x + 1, y));
                    hit = true;
                }
                if y + 1 != col_num && grid[x][y + 1] == 1 {
                    grid[x][y + 1] = 2;
                    tmp.push((x, y + 1));
                    hit = true;
                }
            }
            if hit {
                count += 1;
            }
            stack = tmp;
        }
        for x in 0..row_num {
            for y in 0..col_num {
                if grid[x][y] == 1 {
                    return -1;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let orange = [[2, 1, 1], [1, 1, 0], [0, 1, 1]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        assert_eq!(4, Solution::oranges_rotting(orange));
    }

    #[test]
    fn test_2() {
        let orange = [[2, 1, 1], [0, 1, 1], [1, 0, 1]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        assert_eq!(-1, Solution::oranges_rotting(orange));
    }

    #[test]
    fn test_3() {
        let orange = [[0, 2]].iter().map(|x| x.to_vec()).collect();
        assert_eq!(0, Solution::oranges_rotting(orange));
    }
}
