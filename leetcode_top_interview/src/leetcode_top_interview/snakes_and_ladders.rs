struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let board = Self::prepare_new_board(board);
        let next_of_idx = |pos: i32| board[pos as usize - 1];
        // println!("board: {:?}", board);
        let mut shortest_step = vec![-1; n * n + 1];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(1);
        let mut step = 0;
        while !queue.is_empty() && shortest_step[n * n] == -1 {
            let next_size = queue.len();
            for _ in 0..next_size {
                let mut next = queue.pop_front().unwrap();
                let board_value = next_of_idx(next);
                if board_value != -1 {
                    next = board_value;
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

    fn prepare_new_board(board: Vec<Vec<i32>>) -> Vec<i32> {
        // let mut ret = vec![-1];
        board
            .into_iter()
            .rev()
            .enumerate()
            .map(|(row_idx, mut a_row)| {
                if row_idx % 2 == 1 {
                    a_row.reverse();
                }
                a_row
            })
            .flatten()
            .collect::<Vec<_>>()
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
