struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let mut status = vec![vec![0; board[0].len()]; board.len()];
        for (row_idx, row) in board.iter().enumerate() {
            for (col_idx, val) in row.iter().enumerate() {
                if *val == 1 {
                    Self::add_live(&mut status, (row_idx, col_idx));
                }
            }
        }
        for (row_idx, row) in board.iter_mut().enumerate() {
            for (col_idx, val) in row.iter_mut().enumerate() {
                let lives = status[row_idx][col_idx];
                if *val == 1 {
                    if lives < 2 {
                        *val = 0;
                    } else if lives > 3 {
                        *val = 0;
                    }
                } else {
                    if lives == 3 {
                        *val = 1;
                    }
                }
            }
        }
    }
    fn add_live(board: &mut Vec<Vec<i32>>, coor: (usize, usize)) {
        let (x, y) = coor;
        if x + 1 < board.len() {
            let row = &mut board[x + 1];
            if y + 1 < row.len() {
                row[y + 1] += 1;
            }
            row[y] += 1;
            if y > 0 {
                row[y - 1] += 1;
            }
        }
        let row = &mut board[x];
        if y + 1 < row.len() {
            row[y + 1] += 1;
        }
        if y > 0 {
            row[y - 1] += 1;
        }
        if x > 0 {
            let row = &mut board[x - 1];
            if y + 1 < row.len() {
                row[y + 1] += 1;
            }
            row[y] += 1;
            if y > 0 {
                row[y - 1] += 1;
            }
        }
    }
}
