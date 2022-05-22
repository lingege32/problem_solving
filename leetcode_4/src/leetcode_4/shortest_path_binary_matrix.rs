struct Solution;
use std::collections::VecDeque;
impl Solution {
    #[allow(dead_code)]
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        Self::aStarSearch(grid)
        // Self::bfs(grid)
    }
    
    #[allow(dead_code)]
    fn aStarSearch(grid: Vec<Vec<i32>>) -> i32 {
        0
    }
    
    #[allow(dead_code)]
    fn bfs(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        if grid[0][0] == 1 || *grid.last().map(|x| x.last().unwrap()).unwrap() == 1 {
            return -1;
        }
        if grid.len() == 1 && grid[0].len() == 1 {
            if grid[0][0] == 0 {
                return 1;
            } else {
                return 0;
            }
        }
        let mut path = vec![vec![-1; grid[0].len()]; grid.len()];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 {
                    path[i][j] = 0;
                }
            }
        }

        let mut queue = VecDeque::new();
        queue.push_back((0, 0));
        let mut distance = 0;
        while !queue.is_empty() {
            let len = queue.len();
            for _ in 0..len {
                let cur = queue.pop_front().unwrap();
                grid[cur.0][cur.1] = 1;
                path[cur.0][cur.1] = distance;
                add_candidate(&mut queue, cur, &grid);
            }
            distance += 1;
            if *path.last().map(|x| x.last().unwrap()).unwrap() != 0 {
                break;
            }
        }
        let d = *path.last().map(|x| x.last().unwrap()).unwrap();
        if d == 0 {
            return -1;
        } else {
            return d + 1;
        }

        /// # inner function
        fn add_candidate(
            queue: &mut VecDeque<(usize, usize)>,
            position: (usize, usize),
            grid: &Vec<Vec<i32>>,
        ) {
            let position = (position.0 as i32, position.1 as i32);
            let dirs = [
                (-1, 0),
                (-1, -1),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];
            for dir in dirs {
                let new_x = position.0 + dir.0;
                let new_y = position.1 + dir.1;
                if new_x > -1
                    && new_y > -1
                    && (new_x as usize) < grid.len()
                    && (new_y as usize) < grid[0].len()
                    && grid[new_x as usize][new_y as usize] == 0
                {
                    queue.push_back((new_x as usize, new_y as usize));
                }
            }
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let grid = [[1, 0, 0], [1, 1, 0], [1, 1, 0]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        assert_eq!(-1, Solution::shortest_path_binary_matrix(grid));
    }

    #[test]
    fn test_2() {
        let grid = [[0, 0, 0], [1, 1, 0], [1, 1, 0]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        assert_eq!(4, Solution::shortest_path_binary_matrix(grid));
    }
    #[test]
    fn test_3() {
        let grid = [[0]].iter().map(|x| x.to_vec()).collect();
        assert_eq!(-1, Solution::shortest_path_binary_matrix(grid));
    }
}
