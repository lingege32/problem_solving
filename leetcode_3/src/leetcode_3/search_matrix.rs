struct Solution {}

// more clear and fast
impl Solution {
    #[allow(dead_code)]
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut i, mut j): (i32, i32) = (0 as i32, (n - 1) as i32);
        while i < (m as i32) && j > -1 {
            match matrix[i as usize][j as usize].cmp(&target) {
                std::cmp::Ordering::Less => i += 1,
                std::cmp::Ordering::Greater => j -= 1,
                std::cmp::Ordering::Equal => return true
            }
        }
        false
    }
}
// 75% suck performance
// use std::collections::VecDeque;
// impl Solution {
//     #[allow(dead_code)]
//     pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
//         if matrix.is_empty() || matrix[0].is_empty() {
//             return false;
//         }
//         let (mut col, mut row) = (0, 0);
//         for i in matrix[0].iter() {
//             match i.cmp(&target) {
//                 std::cmp::Ordering::Equal => {
//                     return true;
//                 }
//                 std::cmp::Ordering::Less => {
//                     col += 1;
//                 }
//                 std::cmp::Ordering::Greater => break,
//             }
//         }
//         for i in matrix.iter() {
//             match i[0].cmp(&target) {
//                 std::cmp::Ordering::Equal => {
//                     return true;
//                 }
//                 std::cmp::Ordering::Less => {
//                     row += 1;
//                 }
//                 std::cmp::Ordering::Greater => break,
//             }
//         }
//         if col == 0 || row == 0 {
//             return false;
//         }
//         let mut visited = vec![vec![false; col]; row];
//         let mut queue = VecDeque::new();
//         queue.push_back((0, 0));
//         while let Some((r, c)) = queue.pop_front() {
//             if visited[r][c] {
//                 continue;
//             }
//             visited[r][c] = true;
//             match matrix[r][c].cmp(&target) {
//                 std::cmp::Ordering::Equal => {
//                     return true;
//                 }
//                 std::cmp::Ordering::Greater => {
//                     continue;
//                 }
//                 std::cmp::Ordering::Less => {
//                     if r + 1 != row && c + 1 != col {
//                         if matrix[r + 1][c] < matrix[r][c + 1] {
//                             queue.push_back((r, c + 1));
//                             queue.push_back((r + 1, c));
//                         } else {
//                             queue.push_back((r + 1, c));
//                             queue.push_back((r, c + 1));
//                         }
//                     } else {
//                         if r + 1 == row && c + 1 != col {
//                             queue.push_back((r, c + 1));
//                         }
//                         if c + 1 == col && r + 1 != row {
//                             queue.push_back((r + 1, c));
//                         }
//                     }
//                 }
//             }
//         }

//         false
//     }
// }

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let matrix = [
            [1, 4, 7, 11, 15],
            [2, 5, 8, 12, 19],
            [3, 6, 9, 16, 22],
            [10, 13, 14, 17, 24],
            [18, 21, 23, 26, 30],
        ]
        .iter()
        .map(|x| x.to_vec())
        .collect();
        let target = 5;
        assert!(Solution::search_matrix(matrix, target));
    }
    #[test]
    fn test_2() {
        let matrix = [
            [1, 4, 7, 11, 15],
            [2, 5, 8, 12, 19],
            [3, 6, 9, 16, 22],
            [10, 13, 14, 17, 24],
            [18, 21, 23, 26, 30],
        ]
        .iter()
        .map(|x| x.to_vec())
        .collect();
        let target = 20;
        assert!(!Solution::search_matrix(matrix, target));
    }
    #[test]
    fn test_3() {
        let matrix = [
            [1, 2, 3, 4, 5],
            [6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15],
            [16, 17, 18, 19, 20],
            [21, 22, 23, 24, 25],
        ]
        .iter()
        .map(|x| x.to_vec())
        .collect();
        let target = 25;
        assert!(Solution::search_matrix(matrix, target));
    }
}
