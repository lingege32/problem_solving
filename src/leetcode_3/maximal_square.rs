struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let mut max = 0;
        let (r, c) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0; c]; r];
        for i in 0..r {
            dp[i][0] = if matrix[i][0] == '1' {
                max = 1;
                1
            } else {
                0
            };
        }
        for j in 1..c {
            dp[0][j] = if matrix[0][j] == '1' {
                max = 1;
                1
            } else {
                0
            };
        }
        for i in 1..r {
            for j in 1..c {
                if matrix[i][j] == '1' {
                    let tmp = dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]) + 1;
                    max = max.max(tmp);
                    dp[i][j] = tmp;
                }
            }
        }
        max * max
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
                .map(|t| t.chars().next().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

        assert_eq!(4, Solution::maximal_square(matrix));
    }

    #[test]
    fn test_4() {
        let matrix = [
            ["1", "1", "1", "1", "1"],
            ["1", "1", "1", "1", "1"],
            ["0", "0", "0", "0", "0"],
            ["1", "1", "1", "1", "1"],
            ["1", "1", "1", "1", "1"],
        ]
        .iter()
        .map(|x| {
            x.iter()
                .map(|t| t.chars().next().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

        assert_eq!(4, Solution::maximal_square(matrix));
    }

    #[test]
    fn test_2() {
        let matrix = [["0", "1"], ["1", "0"]]
            .iter()
            .map(|x| {
                x.iter()
                    .map(|t| t.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        assert_eq!(1, Solution::maximal_square(matrix));
    }

    #[test]
    fn test_3() {
        let matrix = [["0"]]
            .iter()
            .map(|x| {
                x.iter()
                    .map(|t| t.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        assert_eq!(0, Solution::maximal_square(matrix));
    }
}
