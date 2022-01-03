struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            0
        } else {
            let r = matrix.len();
            let c = matrix[0].len();
            let mut dp = vec![vec![0; c]; r];
            for i in 0..r {
                for j in 0..c {
                    if matrix[i][j] == '1' {
                        dp[i][j] = if j == 0 { 1 } else { dp[i][j - 1] + 1 }
                    }
                }
            }
            let mut ans = 0;
            for i in 0..r {
                for j in 0..c {
                    let mut max = i32::MAX;
                    for k in i..r {
                        max = max.min(dp[k][j]);
                        if max == 0 {
                            break;
                        }
                        ans = ans.max(max * (k - i + 1) as i32);
                    }
                }
            }
            ans
        }
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
                .map(|x| x.chars().next().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
        assert_eq!(6, Solution::maximal_rectangle(matrix));
    }
    #[test]
    fn test_3() {
        let matrix = [["0"]]
            .iter()
            .map(|x| {
                x.iter()
                    .map(|x| x.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(0, Solution::maximal_rectangle(matrix));
    }
    #[test]
    fn test_4() {
        let matrix = [["1"]]
            .iter()
            .map(|x| {
                x.iter()
                    .map(|x| x.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(1, Solution::maximal_rectangle(matrix));
    }
    #[test]
    fn test_2() {
        let matrix: Vec<Vec<char>> = vec![];
        assert_eq!(0, Solution::maximal_rectangle(matrix));
    }
    #[test]
    fn test_5() {
        let matrix = [["0", "0"]]
            .iter()
            .map(|x| {
                x.iter()
                    .map(|x| x.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(0, Solution::maximal_rectangle(matrix));
    }
}
