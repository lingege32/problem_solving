struct Solution;
use std::collections::VecDeque;
impl Solution {
    #[allow(dead_code)]
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        Self::a_star_search(grid)
        // Self::bfs(grid)
    }
    #[allow(dead_code)]
    fn a_star_search(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        if grid[0][0] == 1 || *grid.last().map(|x| x.last().unwrap()).unwrap() == 1 {
            return -1;
        }
        // Transform grid to path
        // 0 0 0      1  0  0      1  2  3
        // 1 1 0 =>  -1 -1  0  => -1 -1  3
        // 1 1 0     -1 -1  0     -1 -1  4
        //
        for rows in grid.iter_mut() {
            for item in rows {
                if *item == 1 {
                    *item = -1;
                }
            }
        }

        let target = (grid.len(), grid[0].len());
        use std::collections::BinaryHeap;
        let mut priority_queue = BinaryHeap::new();

        priority_queue.push(PositionItem::new(predict((0, 0), target), (0, 0)));
        grid[0][0] = 1;
        let mut tmp = Vec::with_capacity(8);

        while !priority_queue.is_empty() {
            // println!("queue: {:?}", priority_queue);
            let PositionItem {
                cost: _,
                coor: cur_coor,
            } = priority_queue.pop().unwrap();
            if cur_coor.0 + 1 == target.0 && cur_coor.1 + 1 == target.1 {
                // for i in grid.iter() {
                //     println!("{:?}", i);
                // }
                return grid[cur_coor.0][cur_coor.1];
            }
            let next_distance = grid[cur_coor.0][cur_coor.1] + 1;

            // update around
            update_arround(&mut tmp, cur_coor, target);
            for &(x, y) in tmp.iter() {
                let c = &mut grid[x][y];
                if *c == 0 || *c > next_distance {
                    *c = next_distance;
                    let predict_c = predict((x, y), target) + (next_distance as usize);
                    priority_queue.push(PositionItem::new(predict_c, (x, y)));
                }
            }
        }

        return -1;
        // end

        fn predict(cur: (usize, usize), tgt: (usize, usize)) -> usize {
            (tgt.0 - 1 - cur.0).max(tgt.1 - 1 - cur.1)
        }

        #[derive(Eq, Ord, Debug)]
        struct PositionItem {
            cost: usize,
            coor: (usize, usize),
        }
        impl PositionItem {
            pub fn new(ct: usize, cr: (usize, usize)) -> Self {
                PositionItem { cost: ct, coor: cr }
            }
        }
        impl PartialEq for PositionItem {
            fn eq(&self, other: &Self) -> bool {
                self.cost == other.cost && self.coor == other.coor
            }
        }
        impl PartialOrd for PositionItem {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(other.cost.cmp(&self.cost))
            }
        }
        fn update_arround(
            tmp: &mut Vec<(usize, usize)>,
            cur: (usize, usize),
            target: (usize, usize),
        ) {
            tmp.clear();
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
            let cur = (cur.0 as i32, cur.1 as i32);
            for dir in dirs {
                let next_x = cur.0 + dir.0;
                let next_y = cur.1 + dir.1;
                if next_x > -1 && next_y > -1 {
                    let next_x = next_x as usize;
                    let next_y = next_y as usize;
                    if next_x < target.0 && next_y < target.1 {
                        tmp.push((next_x, next_y));
                    }
                }
            }
        }
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
        assert_eq!(1, Solution::shortest_path_binary_matrix(grid));
    }
    #[test]
    fn test_4() {
        let grid = [
            [0, 0, 0, 0, 1, 1],
            [0, 1, 0, 0, 1, 0],
            [1, 1, 0, 1, 0, 0],
            [0, 1, 0, 0, 1, 1],
            [0, 1, 0, 0, 0, 1],
            [0, 0, 1, 0, 0, 0],
        ]
        .iter()
        .map(|x| x.to_vec())
        .collect();
        assert_eq!(7, Solution::shortest_path_binary_matrix(grid));
    }
    #[test]
    fn test_5() {
        let grid = [
            [0, 0, 0, 0, 1, 1, 1, 1, 0],
            [0, 1, 1, 0, 0, 0, 0, 1, 0],
            [0, 0, 1, 0, 0, 0, 0, 0, 0],
            [1, 1, 0, 0, 1, 0, 0, 1, 1],
            [0, 0, 1, 1, 1, 0, 1, 0, 1],
            [0, 1, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 0, 1, 0, 0, 0],
            [0, 1, 0, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 0, 1, 0],
        ]
        .iter()
        .map(|x| x.to_vec())
        .collect();
        assert_eq!(11, Solution::shortest_path_binary_matrix(grid));
    }
}
