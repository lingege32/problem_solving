struct Solution {}
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let zeros: Vec<(usize, usize)> = matrix
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &cell)| cell == 0)
                    .map(move |(j, _)| (i, j))
            })
            .collect();
        for (row, col) in zeros {
            matrix[row].iter_mut().for_each(|cell| *cell = 0);
            matrix.iter_mut().map(|row| row.get_mut(col).unwrap()).for_each(|cell| *cell = 0);
        }
    }
}


// hash_set solution get 11 ms runtime, so slow
// use std::collections::HashSet;
// impl Solution {
//     pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
//         let mut row_set = HashSet::new();
//         let mut col_set = HashSet::new();
//         for row in 0..matrix.len() {
//             for col in 0..matrix[0].len() {
//                 if matrix[row][col] == 0 {
//                     row_set.insert(row);
//                     col_set.insert(col);
//                 }
//             }
//         }
//         for row in 0..matrix.len() {
//             for col in 0..matrix[0].len() {
//                 if row_set.contains(&row) || col_set.contains(&col) {
//                     matrix[row][col] = 0;
//                 }
//             }
//         }
//     }
// }

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let mut matrix = [[1, 1, 1], [1, 0, 1], [1, 1, 1]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        let out: Vec<Vec<i32>> = [[1, 0, 1], [0, 0, 0], [1, 0, 1]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        Solution::set_zeroes(&mut matrix);
        assert_eq!(out, matrix);
    }
}
