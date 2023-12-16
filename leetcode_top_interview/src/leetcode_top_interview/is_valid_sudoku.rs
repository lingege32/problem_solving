struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let n = 9;
        for i in 0..n {
            let j = (i / 3) * 3;
            let k = (i % 3) * 3;
            for matrix in [
                (0..n).map(|_| i).zip(0..n).collect::<Vec<_>>(),
                (0..n).zip((0..n).map(|_| i)).collect(),
                (0..n)
                    .map(|d| j + d / 3)
                    .zip((k..k + 3).cycle().take(n))
                    .collect(),
            ]
            .iter()
            {
                let mut bmap = 0;
                for &(r, c) in matrix {
                    let d = board[r][c];
                    if let Some(d) = d.to_digit(10) {
                        let index = 2u16 << d;
                        if bmap & index != 0 {
                            return false;
                        }
                        bmap = bmap | index;
                    }
                }
            }
        }
        true
    }
}
