struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let row_len = board.len();
        let col_len = board[0].len();
        let mut stack = vec![];
        let mut visit = vec![vec![false; board[0].len()]; board.len()];
        for (row_idx, row) in board.iter_mut().enumerate() {
            if row[0] == 'O' {
                stack.push((row_idx, 0));
                row[0] = 'V';
            }

            if row[col_len - 1] == 'O' {
                stack.push((row_idx, col_len - 1));
                row[col_len - 1] = 'V';
            }
        }
        for (idx, val) in board[0].iter_mut().enumerate() {
            if *val == 'O' {
                *val = 'V';
                stack.push((0, idx));
            }
        }
        for (idx, val) in board[row_len - 1].iter_mut().enumerate() {
            if *val == 'O' {
                *val = 'V';
                stack.push((row_len - 1, idx));
            }
        }

        while let Some((x, y)) = stack.pop() {
            if visit[x][y] == true {
                continue;
            }
            visit[x][y] = true;

            if board[x][y] == 'X' {
                continue;
            }
            board[x][y] = 'V';
            if x != 0 {
                stack.push((x - 1, y));
            }
            if y != 0 {
                stack.push((x, y - 1));
            }
            if x + 1 < row_len {
                stack.push((x + 1, y));
            }
            if y + 1 < col_len {
                stack.push((x, y + 1));
            }
        }

        for row in board.iter_mut() {
            for val in row.iter_mut() {
                if *val == 'V' {
                    *val = 'O';
                } else {
                    *val = 'X';
                }
            }
        }
    }
}
