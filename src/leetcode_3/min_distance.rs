// - - h o r
// - 0 1 2 3
// r 1 1 2 2
// o 2 2 1 2
// s 3 3 2 2

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let w1 = word1.as_bytes();
        let w2 = word2.as_bytes();
        let w1len = w1.len();
        let w2len = w2.len();
        let mut dp = vec![vec![0; w1len + 1]; w2len + 1];
        for i in 0..=w1len {
            dp[0][i] = i as i32;
        }
        for i in 0..=w2len {
            dp[i][0] = i as i32;
        }
        for r in 1..=w2len {
            for c in 1..=w1len {
                let min = dp[r][c - 1].min(dp[r - 1][c]).min(dp[r - 1][c - 1]);
                if w1[c - 1] == w2[r - 1] {
                    dp[r][c] = dp[r - 1][c - 1];
                } else {
                    dp[r][c] = min + 1;
                }
            }
        }
        // for i in dp.iter() {
        //     println!("{:?}", i);
        // }
        dp[w2len][w1len]
    }
}

#[cfg(test)]
mod test_ {
    use super::*;

    #[test]
    fn test_1() {
        let word1 = "horse".to_string();
        let word2 = "ros".to_string();
        let ans = 3;
        assert_eq!(ans, Solution::min_distance(word1, word2));
    }

    #[test]
    fn test_2() {
        let word1 = "intention".to_string();
        let word2 = "execution".to_string();
        let ans = 5;
        assert_eq!(ans, Solution::min_distance(word1, word2));
    }

    #[test]
    fn test_3() {
        let word1 = "zoologicoarchaeologist".to_string();
        let word2 = "zoogeologist".to_string();
        let ans = 10;
        assert_eq!(ans, Solution::min_distance(word1, word2));
    }

    #[test]
    fn test_4() {
        let word1 = "zoo".to_string();
        let word2 = "zo".to_string();
        let ans = 1;
        assert_eq!(ans, Solution::min_distance(word1, word2));
    }
}
