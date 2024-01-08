struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let m = board.len();
        let n = board[0].len();
        let mut board = board
            .into_iter()
            .map(|x| x.into_iter().map(|c| (c, false)).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let word = word.chars().collect::<Vec<_>>();
        for x in 0..m {
            for y in 0..n {
                if Self::inner(&mut board, &word, (x, y)) {
                    return true;
                }
            }
        }
        false
    }
    fn inner(board: &mut Vec<Vec<(char, bool)>>, word: &[char], (x, y): (usize, usize)) -> bool {
        if word.is_empty() {
            return true;
        }
        let b = &mut board[x][y];
        if b.0 != word[0] || b.1 == true {
            return false;
        }
        b.1 = true;

        let (m, n) = (board.len(), board[0].len());
        if x > 0 {
            if Self::inner(board, &word[1..], (x - 1, y)) {
                return true;
            }
        }
        if y > 0 {
            if Self::inner(board, &word[1..], (x, y - 1)) {
                return true;
            }
        }
        if x + 1 < m {
            if Self::inner(board, &word[1..], (x + 1, y)) {
                return true;
            }
        }
        if y + 1 < n {
            if Self::inner(board, &word[1..], (x, y + 1)) {
                return true;
            }
        }
        board[x][y].1 = false;
        return word.len() == 1;
    }
}
