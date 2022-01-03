struct Solution {}
impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let mut matrix = matrix;
        for row_idx in 1..matrix.len() {
            for col_idx in 0..matrix[row_idx].len() {
                if matrix[row_idx][col_idx] == 1 {
                    matrix[row_idx][col_idx] = matrix[row_idx - 1][col_idx] + 1;
                }
            }
        }

        matrix.iter_mut().map(|each_row| {
            each_row.sort_unstable();
            each_row.iter().rev().enumerate().map(|(idx, s)| {
                (idx+1) as i32 * *s
            }).max().unwrap_or(0)
        }).max().unwrap_or(0)
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let matrix = [[0, 0, 1], [1, 1, 1], [1, 0, 1]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        assert_eq!(4, Solution::largest_submatrix(matrix));
    }
    #[test]
    fn test_2() {
        let matrix = [[1, 0, 1, 0, 1]].iter().map(|x| x.to_vec()).collect();
        assert_eq!(3, Solution::largest_submatrix(matrix));
    }
    #[test]
    fn test_3() {
        let matrix = [[0, 0], [0, 0]].iter().map(|x| x.to_vec()).collect();
        assert_eq!(0, Solution::largest_submatrix(matrix));
    }
}
