struct Solution;
use std::collections::VecDeque;
impl Solution {
    #[allow(dead_code)]
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let table = Self::prepare_idx_mapping(n);
        // println!("{:?}", table);
        let pos_index = |pos: i32| table[(pos - 1) as usize];
        let mut shortest_step = vec![-1; n * n + 1];
        let mut queue = VecDeque::new();
        queue.push_back(1);
        let mut step = 0;
        while !queue.is_empty() && shortest_step[n * n] == -1 {
            let next_size = queue.len();
            for _ in 0..next_size {
                let mut next = queue.pop_front().unwrap();

                let (x, y) = pos_index(next);
                if board[x][y] != -1 {
                    next = board[x][y];
                }
                if shortest_step[next as usize] != -1 {
                    continue;
                }

                shortest_step[next as usize] = step;
                for next_pos in next + 1..=((n * n) as i32).min(next + 6) {
                    queue.push_back(next_pos);
                }
            }
            // println!("step: {step}");
            // println!("{:?}", shortest_step);
            step += 1;
        }

        // println!("{:?}", shortest_step);
        shortest_step[n * n]
    }
    fn prepare_idx_mapping(n: usize) -> Vec<(usize, usize)> {
        let finish = n * n;
        let mut ret = vec![(0, 0); finish];
        let (mut dir, mut col_idx) = if n % 2 == 0 {
            (true, 0)
        } else {
            (false, n - 1)
        };
        let mut row_idx = 0;
        let mut same_row_step = 0;
        for idx in (1..=n * n).rev() {
            ret[idx - 1] = (row_idx, col_idx);

            if same_row_step + 1 == n {
                same_row_step = 0;
                row_idx += 1;
                dir = !dir;
            } else {
                same_row_step += 1;
                if dir {
                    col_idx += 1;
                } else {
                    col_idx -= 1;
                }
            }
        }

        ret
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let board = [
            [-1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1],
            [-1, 35, -1, -1, 13, -1],
            [-1, -1, -1, -1, -1, -1],
            [-1, 15, -1, -1, -1, -1],
        ]
        .iter()
        .map(|x| x.to_vec())
        .collect::<Vec<_>>();
        let ans = 4;
        assert_eq!(ans, Solution::snakes_and_ladders(board));
    }
    #[test]
    fn test_12() {
        /*
           -1  4 -1
            6  2  6
           -1  3 -1
        */
        let board = [[-1, 4, -1], [6, 2, 6], [-1, 3, -1]]
            .iter()
            .map(|x| x.to_vec())
            .collect::<Vec<_>>();
        let ans = 2;
        assert_eq!(ans, Solution::snakes_and_ladders(board));
    }
    #[test]
    fn test_13() {
        let board = [
            [-1, -1, 30, 14, 15, -1],
            [23, 9, -1, -1, -1, 9],
            [12, 5, 7, 24, -1, 30],
            [10, -1, -1, -1, 25, 17],
            [32, -1, 28, -1, -1, 32],
            [-1, -1, 23, -1, 13, 19],
        ]
        .iter()
        .map(|x| x.to_vec())
        .collect::<Vec<_>>();
        let ans = 2;
        assert_eq!(ans, Solution::snakes_and_ladders(board));
    }
    #[test]
    fn test_134() {
        let board = [[1, 1, -1], [1, 1, 1], [-1, 1, 1]]
            .iter()
            .map(|x| x.to_vec())
            .collect::<Vec<_>>();
        let ans = -1;
        assert_eq!(ans, Solution::snakes_and_ladders(board));
    }
}
