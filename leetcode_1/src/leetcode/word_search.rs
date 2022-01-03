struct Solution {}

// other's 100% Solution
type Pos = (usize, usize);
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word: Vec<_> = word.chars().collect();
        let m = board.len();
        let n = board[0].len();
        let mut used = Vec::new();

        let mut all = std::collections::HashSet::new();
        for i in 0..m {
            for j in 0..n {
                all.insert(board[i][j]);
            }
        }
        for w in word.iter() {
            if !all.contains(w) {
                return false;
            }
        }

        for i in 0..m {
            for j in 0..n {
                if Self::check(&board, m, n, &word, 0, &mut used, i, j) {
                    return true;
                }
                used.clear();
            }
        }
        return false;
    }

    fn check(
        board: &Vec<Vec<char>>,
        m: usize,
        n: usize,
        word: &Vec<char>,
        word_i: usize,
        used: &mut Vec<Pos>,
        curx: usize,
        cury: usize,
    ) -> bool {
        if word[word_i] != board[curx][cury] {
            return false;
        }

        if word_i == word.len() - 1 {
            return true;
        }

        used.push((curx, cury));
        if curx > 0 {
            if !used.contains(&(curx - 1, cury))
                && Self::check(board, m, n, word, word_i + 1, used, curx - 1, cury)
            {
                return true;
            }
        }
        if curx < m - 1 {
            if !used.contains(&(curx + 1, cury))
                && Self::check(board, m, n, word, word_i + 1, used, curx + 1, cury)
            {
                return true;
            }
        }
        if cury > 0 {
            if !used.contains(&(curx, cury - 1))
                && Self::check(board, m, n, word, word_i + 1, used, curx, cury - 1)
            {
                return true;
            }
        }
        if cury < n - 1 {
            if !used.contains(&(curx, cury + 1))
                && Self::check(board, m, n, word, word_i + 1, used, curx, cury + 1)
            {
                return true;
            }
        }
        used.pop();

        return false;
    }
}

// My 80 ms bad Solution
// impl Solution {
//     pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
//         if board.is_empty() || board[0].is_empty() {
//             return false;
//         }
//         let word = word.chars().collect::<Vec<_>>();
//         let mut visit = vec![vec![false; board[0].len()]; board.len()];
//         for idx1 in 0..board.len() {
//             for idx2 in 0..board[0].len() {
//                 if Self::search(&board, &word, 0, idx1, idx2, &mut visit) {
//                     return true;
//                 }
//             }
//         }
//         false
//     }
//     fn search(
//         board: &Vec<Vec<char>>,
//         word: &[char],
//         word_idx: usize,
//         board_row_idx: usize,
//         board_col_idx: usize,
//         visit: &mut Vec<Vec<bool>>,
//     ) -> bool {
//         if visit[board_row_idx][board_col_idx]
//             || board[board_row_idx][board_col_idx] != word[word_idx]
//         {
//             return false;
//         }
//         visit[board_row_idx][board_col_idx] = true;
//         let res = word_idx + 1 == word.len()
//             || (board_row_idx > 0
//                 && Self::search(
//                     board,
//                     word,
//                     word_idx + 1,
//                     board_row_idx - 1,
//                     board_col_idx,
//                     visit,
//                 ))
//             || (board_col_idx > 0
//                 && Self::search(
//                     board,
//                     word,
//                     word_idx + 1,
//                     board_row_idx,
//                     board_col_idx - 1,
//                     visit,
//                 ))
//             || (board_row_idx < board.len() - 1
//                 && Self::search(
//                     board,
//                     word,
//                     word_idx + 1,
//                     board_row_idx + 1,
//                     board_col_idx,
//                     visit,
//                 ))
//             || (board_col_idx < board[0].len() - 1
//                 && Self::search(
//                     board,
//                     word,
//                     word_idx + 1,
//                     board_row_idx,
//                     board_col_idx + 1,
//                     visit,
//                 ));
//         visit[board_row_idx][board_col_idx] = false;
//         res
//     }
// }

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let a = [
            ['A', 'B', 'C', 'E'],
            ['S', 'F', 'C', 'S'],
            ['A', 'D', 'E', 'E'],
        ]
        .iter()
        .map(|x| x.to_vec())
        .collect();
        assert!(!Solution::exist(a, "ABAB".to_string()));
    }
    #[test]
    fn test_12() {
        let a = [['A']].iter().map(|x| x.to_vec()).collect();
        assert!(Solution::exist(a, "A".to_string()));
    }
}
