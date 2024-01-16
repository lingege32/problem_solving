struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let mut matrix = matrix
            .into_iter()
            .map(|x| {
                x.into_iter()
                    .map(|x| x as i32 - '0' as i32)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let row_num = matrix.len();
        let col_num = matrix[0].len();
        for row in 1..row_num {
            for col in 1..col_num {
                if matrix[row][col] == 1 {
                    let max_square = matrix[row][col - 1]
                        .min(matrix[row - 1][col])
                        .min(matrix[row - 1][col - 1])
                        + 1;
                    matrix[row][col] = max_square;
                }
            }
        }
        matrix
            .iter()
            .map(|x| x.iter().max().map(|y| *y * *y).unwrap())
            .max()
            .unwrap()
    }
}
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let matrix = [
            ["1", "0", "1", "0", "0"],
            ["1", "0", "1", "1", "1"],
            ["1", "1", "1", "1", "1"],
            ["1", "0", "0", "1", "0"],
        ]
        .iter()
        .map(|x| {
            x.iter()
                .map(|y| y.chars().next().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
        assert_eq!(4, Solution::maximal_square(matrix));
    }
}
