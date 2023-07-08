struct Solution;
impl Solution {
    #[allow(dead_code)]
    // Solution:
    // https://leetcode.com/problems/put-marbles-in-bags/editorial/
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        let n = weights.len();
        let mut pair_weights: Vec<_> = (0..n - 1)
            .map(|i| weights[i] + weights[i + 1])
            .collect();

        pair_weights.sort_unstable();
        println!("{:?}", pair_weights);
        (0..(k - 1) as usize).fold(0i64, |ans, i| {
            println!("i = {i}");
            println!("n - 2 - i = {}",n-2-i);
            ans + (pair_weights[n - 2 - i] - pair_weights[i]) as i64
        })
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let weights = vec![1,3,5,1];
        let k = 2;
        let ans = 4;
        assert_eq!(ans, Solution::put_marbles(weights, k));
    }
    #[test]
    fn test_2() {
        let weights = vec![1,3];
        let k = 2;
        let ans = 0;
        assert_eq!(ans, Solution::put_marbles(weights, k));
    }
}